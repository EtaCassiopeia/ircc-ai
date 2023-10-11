use std::time::Duration;

use eventsource_client as es;
use futures::TryStreamExt;
use ircc_ai::constants::ORACLE_QUERY_URL_DEFAULT;
use teloxide::prelude::*;
use tokio::sync::mpsc;

enum EventMessage {
	Start,
	Done(String),
	Event { event_type: String, data: String },
	Comment(String)
}

#[cfg(feature = "bot")]
#[tokio::main]
async fn main() {
	use ircc_ai::convrsation::Query;

	pretty_env_logger::init();
	log::info!("Starting IRCC bot...");

	let bot = Bot::from_env();

	teloxide::repl(bot, |bot: Bot, msg: Message| async move {
		log::info!("Received {:?} from @{:?}", msg.text(), msg.from());

		if let Some(q) = msg.text() {
			if q != "/start" {
				let json_query = Query { query: q.to_string() };
				let json_string = serde_json::to_string(&json_query).unwrap();
				// TODO: handle errors
				process_query(bot, msg.chat.id, &json_string).await;
			}
		};
		Ok(())
	})
	.await;
}

async fn process_query(bot: Bot, chat_id: ChatId, query: &str) -> Result<(), es::Error> {
	let url = std::env::var("ORACLE_QUERY_URL").unwrap_or(ORACLE_QUERY_URL_DEFAULT.into());

	let client = es::ClientBuilder::for_url(&url)?
		.header("Content-Type", "application/json")?
		.method(String::from("POST"))
		.body(query.to_string())
		.reconnect(
			es::ReconnectOptions::reconnect(false)
				.retry_initial(true)
				.delay(Duration::from_secs(1))
				.backoff_factor(2)
				.delay_max(Duration::from_secs(5))
				.build()
		)
		.build();

	let (tx, mut rx) = mpsc::unbounded_channel();
	tokio::spawn(async move {
		tail_events(client, tx).await;
	});

	// Receive messages from the channel and process them
	while let Some(event) = rx.recv().await {
		match event {
			EventMessage::Start => bot
				.send_message(chat_id, "Started receiving events and processing data.")
				.await
				.map(|_| ())?,
			EventMessage::Done(final_data) => bot.send_message(chat_id, &final_data).await.map(|_| ())?,
			EventMessage::Event { event_type, data } => println!("got an event: {}\n{}", event_type, data),
			EventMessage::Comment(comment) => println!("got a comment: \n{}", comment)
		}
	}

	Ok(())
}

async fn tail_events(client: impl es::Client, tx: mpsc::UnboundedSender<EventMessage>) {
	let started = std::sync::atomic::AtomicBool::new(false);

	let mut stream = client.stream();

	while let Ok(Some(event)) = stream.try_next().await {
		match event {
			es::SSE::Event(ev) => {
				if !started.load(std::sync::atomic::Ordering::Relaxed) {
					let _ = tx.send(EventMessage::Start);
					started.store(true, std::sync::atomic::Ordering::Relaxed);
				}
				if ev.event_type == "DONE" {
					let _ = tx.send(EventMessage::Done(ev.data));
					break; // Stop processing further events
				} else {
					let _ = tx.send(EventMessage::Event {
						event_type: ev.event_type,
						data: ev.data
					});
				}
			}
			es::SSE::Comment(comment) => {
				let _ = tx.send(EventMessage::Comment(comment));
			}
		}
	}
}
