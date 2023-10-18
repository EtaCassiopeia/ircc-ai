# IRCC-AI: Generative AI-Assisted Interactive Resource Center for IRCC

## Description:

The Generative AI-Assisted Interactive Resource Center is an ambitious project aimed at revolutionizing the Immigration, Refugees, and Citizenship Canada (IRCC)’s user engagement experience using RAG. At the core of this project is a Generative AI framework that facilitates a dynamic chat environment, enabling users to pose queries and receive precise, informative responses sourced from IRCC's extensive documentation.

## Retrieval Augmented Generation (RAG)
The idea behind RAG applications is to provide LLMs with additional context at query time for answering the user’s question.

1. When a user asks the support agent a question, the question first goes through an embedding model to calculate its vector representation.
2. The next step is to find the most relevant nodes in the database by comparing the cosine similarity of the embedding values of the user’s question and the documents in the database.
3.Once the relevant nodes are identified using vector search, the application is designed to retrieve additional information from the nodes themselves and also by traversing the relationships in the graph.
4. Finally, the context information from the database is combined with the user question and additional instructions into a prompt that is passed to an LLM to generate the final answer, which is then sent to the user.

## Key Features:

1. **Intelligent Query Handling**:
   - Leveraging cutting-edge Generative AI technology, the system comprehends user queries, regardless of the language they are posed in.
   - The AI agent navigates through the IRCC's documentation, extracting and generating responses that succinctly address user inquiries.
   - Multilingual support ensures inclusivity, allowing users from diverse linguistic backgrounds to interact with the system effortlessly.

2. **Document Retrieval and Analysis**:
   - A robust document retrieval system that swiftly traverses through IRCC’s repository to fetch relevant information.
   - Advanced natural language processing capabilities enable the system to understand and generate accurate responses from the documentation.

3. **Interactive Dashboard**:
   - A comprehensive dashboard that visually represents critical metrics such as the volume of Permanent Residency (PR) applications, distribution of applicants by age, nationality, and other demographics.
   - Real-time updates on the dashboard provide a snapshot of ongoing immigration trends and other pertinent activities.

4. **Web Interface**:
   - A user-friendly web interface serving as the primary interaction point for users.
   - The website will host the interactive chat environment and the dashboard, providing a one-stop solution for users seeking information and insights regarding the IRCC processes.

## Setup and Run

### Scraping

Using `Scrapy`` for web scraping involves several steps. Here's a simplified guide to help you get started:

### 1. **Installation**:
   - You'll need to have Python installed on your machine.
   - Install Scrapy using pip:
     ```bash
     $ pip install scrapy
     $ pip install html2text
     ```

### 2. **Create a new Scrapy project**:
   - In your terminal, navigate to the directory where you want to create your project.
   - Run the following command, replacing `myproject` with the name of your project:
     ```bash
     $ scrapy startproject ircc
     ```

### 3. **Create a Spider**:
   - A spider is a script that tells Scrapy what URLs to scrape and how to extract data from the pages.
   - Inside your project directory, create a spider using the following command, replacing `myspider` and `example.com` with your desired spider name and target URL:
     ```bash
     $ scrapy genspider ircc canada.ca
     ```

### 4. **Define the Spider**:
   - Open the generated spider file (located in the `ircc/spiders` directory).
   - Define the initial URLs, the parsing logic, and how to follow links.
   - You can find some examples in the [scripts](/scripts/scrapy/spiders) directory of this repository.

### 5. **Run the Spider**:
   - In the terminal, navigate to your project directory.
   - Run the spider using the following command, replacing `myspider` with the name of your spider:
     ```bash
     $ scrapy crawl canada_ca_md
     ```
Note: The script folder contains a script to convert the links to markdown inline links. Script can be run as follows:

```bash
$ ./replace_links.sh /Users/mohsen/code/Personal/ircc-dump-scrapy/canadascraper/output-md
```

## Running Locally

To run the project locally, there are a few prerequisites:

- The [Rust toolchain](https://www.rust-lang.org/learn/get-started)
- The [Onnx Runtime](https://onnxruntime.ai/docs/install/). Will be downloaded and installed automatically when building the project.
- [Docker](https://docs.docker.com/engine/install/) to run the [QdrantDB](https://qdrant.tech/) instance.
- `make` for easy automation and development workflow

Once, the above requirements are satisfied, you can run the project like so:

### Docker container

The `ircc-ai` engine (oracle), embed and bot can also be run locally via a docker container and
includes all the necessary dependencies.

Note: Please ensure the scrape data is available in the `content` directory. The `content` directory should contain the `output-md/en` directory from the `scrapy` project.
The data can also be passed in via the `CONTENT_PATH_HOST` environment variable but for sake of simplicity, we will assume the data is available in the `content` directory.

To build the docker images run:

```bash
$ make -f Makefile.local local-image
```

### Create Embeddings

To create the embeddings, run the following command.  It will traverse the directory specified by `CONTENT_PATH_HOST`, creating embeddings for each file. These embeddings will then be stored in `QdrantDB`.

```bash
$ make -f Makefile.local start-embed
```

### Start the Engine
To start the engine, run the following command.  It will start the engine and expose it on port `3000`.

```bash
$ make -f Makefile.local start-oracle
``````


The database dashboard will be accessible at [localhost:6333/dashboard](http://localhost:6333/dashboard), the project communicates with the DB on port `6334`.


## Service Endpoints

> **Note:**
> Since the service returns responses as SSEs, a REST client like Postman is recommended. Download it [here](https://www.postman.com/downloads/). The Postman web client does not support requests to `localhost`.

[![Run in Postman](https://run.pstmn.io/button.svg)](https://app.getpostman.com/run-collection/18073744-276b793e-f5ec-418f-ba0a-9dff94af543e?action=collection%2Ffork&source=rip_markdown&collection-url=entityId%3D18073744-276b793e-f5ec-418f-ba0a-9dff94af543e%26entityType%3Dcollection%26workspaceId%3D8d8a1363-ad0a-45ad-b036-ef6a37e44ef8)

| Endpoint             | Method | Description                                   |
|----------------------|--------|-----------------------------------------------|
| `/`                  | GET    | Redirects to the configured [redirect URL](https://github.com/EtaCassiopeia/ircc-ai).          |
| `/query`             | POST   | Perform a query on the API with a specific question. |


### 1. `/query`

#### Parameters

The parameters are passed as a JSON object in the request body:

- `query` (string, required): The question or query you want to ask.

#### Response

The request is processed by the server and responses are sent as [Server-sent events(SSE)](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events). The event stream will contain [events](https://github.com/EtaCassiopeia/ircc-ai/blob/af60f851b9fc9362ba43c01f88b6f5e6770b1f2a/src/routes/events.rs#L14) with optional data.

#### Example

```bash
$ curl --location 'localhost:3000/query' \
--header 'Content-Type: application/json' \
--data '{
    "query": "How long must I stay in Canada to keep my permanent resident status?"
}'
```

### Start Telegram Bot

To start the telegram bot, run the following command.  It will start the telegram bot and start listening for messages.

```bash
$ -f Makefile.local make start-bot
```