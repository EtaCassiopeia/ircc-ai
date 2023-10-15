use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use ctrlc::set_handler;
use eventsource_client as es;
use futures::TryStreamExt;
use ircc_ai::constants::ORACLE_QUERY_URL_DEFAULT;
use serde::Serialize;
use teloxide::prelude::*;
use tokio::sync::mpsc;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;


#[derive(Serialize)]
pub struct Query {
	pub query: String
}

enum EventMessage {
	Start,
	Done(String),
	Event { event_type: String, data: String },
	Comment(String)
}

#[cfg(feature = "bot")]
#[tokio::main]
async fn main() {
	pretty_env_logger::init();
	info!("Starting IRCC bot...");

	// Initialize Ctrl+C signal handling.
	let running = Arc::new(AtomicBool::new(true));
	let r = running.clone();

	set_handler(move || {
		r.store(false, Ordering::SeqCst);
	})
	.expect("Error setting Ctrl+C handler");

	// Spawn the REPL in a separate thread.
	let (tx, mut rx) = mpsc::channel(32);

	let repl_handle = tokio::spawn(async move {
		run().await;
		tx.send(()).await.expect("Error sending signal to main thread");
	});

	// Wait for Ctrl+C or the REPL to finish.
	tokio::select! {
		_ = rx.recv() => {
			// REPL finished
			info!("REPL finished, terminating...");
		}
		_ = tokio::signal::ctrl_c() => {
			// Ctrl+C signal received, terminate the REPL gracefully.
			info!("Ctrl+C received, terminating REPL...");
			running.store(false, Ordering::SeqCst);
		}
	}

	// Wait for the REPL thread to finish.
	repl_handle.await.expect("Error waiting for REPL thread");
}

async fn run() {

	let bot = Bot::from_env();
	debug!("Bot started...");

	teloxide::repl(bot, |bot: Bot, msg: Message| async move {
		debug!("Received {:?} from @{:?}", msg.text(), msg.from());

		if let Some(q) = msg.text() {
			if q != "/start" {
				let json_query = Query { query: q.to_string() };
				let json_string = serde_json::to_string(&json_query).unwrap();

				process_query(bot, msg.chat.id, &json_string).await.log_on_error().await;
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
				.send_message(chat_id, "Processing your question, please wait. I'll get back to you shortly.")
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
