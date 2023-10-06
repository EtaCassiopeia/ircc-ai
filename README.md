# IRCC-AI: Generative AI-Assisted Interactive Resource Center for IRCC

## Description:

The Generative AI-Assisted Interactive Resource Center is an ambitious project aimed at revolutionizing the Immigration, Refugees, and Citizenship Canada (IRCC)’s user engagement experience. At the core of this project is a Generative AI framework that facilitates a dynamic chat environment, enabling users to pose queries and receive precise, informative responses sourced from IRCC's extensive documentation.

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
     scrapy genspider ircc canada.ca
     ```

### 4. **Define the Spider**:
   - Open the generated spider file (located in the `ircc/spiders` directory).
   - Define the initial URLs, the parsing logic, and how to follow links.
   - You can find some examples in the [scripts](/scripts/scrapy/spiders) directory of this repository.

### 5. **Run the Spider**:
   - In the terminal, navigate to your project directory.
   - Run the spider using the following command, replacing `myspider` with the name of your spider:
     ```bash
     scrapy crawl canada_ca_txt
     ```

### Create Embeddings

```bash
$ cargo run --bin embed -- --path "/Users/mohsen/code/Personal/ircc-dump-scrapy/canadascraper/outputtxt"
```
