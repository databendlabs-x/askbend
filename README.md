# AskBend: SQL-based Knowledge Base Search and Completion using Databend

AskBend is a Rust project that utilizes the power of Databend and OpenAI to create a SQL-based knowledge base from Markdown files.

Databend is a cloud-native data warehouse adept at storing and performing vector computations, making it suitable for this use case.

[Databend Cloud](https://databend.com) seamlessly integrates with OpenAI's capabilities, such as embedding generation, cosine distance calculation, and text completion. This integration means you don't need to interact with OpenAI directly; Databend Cloud manages everything.

The project automatically generates document embeddings from the content, enabling users to search and retrieve the most relevant information to their queries using SQL.

SQL-Based means you don't need any OpenAI API knowledge. With the Databend Cloud platform, you can perform these tasks using SQL. Some SQL AI functions of Databend Cloud include:

- `ai_embedding_vector`: Get the vector from OpenAI API
- `ai_text_completion`: Get the completion of a text
- `cosine_distance`: Calculate the distance between embedding vectors

## Overview

The project follows this general process:

1. Read and parse Markdown files from a directory.
2. Extract the content and store it in the askbend.doc table.
3. Compute embeddings for the content using Databend Cloud's built-in AI capabilities, including OpenAI's embedding generation, all through SQL.
4. When a user queries, generate the query embedding using Databend Cloud's SQL-based ai_embedding_vector function.
5. Perform a vector calculation to find the most relevant doc.content using Databend Cloud's SQL-based cosine_distance function.
6. Concatenate the retrieved content and use OpenAI's completion capabilities with Databend Cloud's SQL-based ai_text_completion function.
7. Output the completion result in Markdown format.

## Setup

### 1. Clone the repository
```
git clone https://github.com/datafuselabs/askbend
cd askbend
```

### 2. Build the project
```
make setup
make build
```

### 3. Create a database in your Databend Cloud:
schema/table.sql:
```
CREATE DATABASE askbend;
USE askbend;

CREATE TABLE doc (path VARCHAR, content VARCHAR, embedding ARRAY(FLOAT32));
```

### 4. Modify the configuration file `conf/askbend.toml`:
```
# Usage:
# askbend -c askbend.toml

[data]
# Path to the directory containing your markdown documents
path = "data/"

[database]
database = "askbend"
table = "doc"
# Data source name (DSN) for connecting to your Databend cloud warehouse
# https://docs.databend.com/using-databend-cloud/warehouses/connecting-a-warehouse
dsn = "databend://<sql-user>:<sql-password>@<your-databend-cloud-warehouse>/default"

[server]
host = "0.0.0.0"
port = 8081
# If true, will insert and embedding all for path/*.
rebuild = false
```

### 5. Prepare your Markdown files by copying them to the data/ directory

### 6. Parse the Markdown files and generate embeddings:
```
./target/release/askbend -c conf/askbend.toml --rebuild

[2023-04-01T07:17:13Z INFO ] Step-1: begin parser all markdown files
[2023-04-01T07:17:14Z INFO ] Step-1: finish parser all markdown files:397, sections:969, tokens:117758
[2023-04-01T07:17:14Z INFO ] Step-2: begin insert to table
[2023-04-01T07:17:14Z INFO ] Step-2: finish insert to table
[2023-04-01T07:17:14Z INFO ] Step-3: begin generate embedding, may take some minutes
[2023-04-01T07:26:03Z INFO ] Step-3: finish generate embedding
[2023-04-01T07:26:03Z INFO ] Step-4: start api server 0.0.0.0:8081
... ...
```

The `--rebuild` flag rebuilds all the embeddings for the data directory. This process may take a few minutes, depending on the number of Markdown files. When the embedding is complete, the API server will start.

### 7. Query your Markdown knowledge base using the API:
```
curl -X POST -H "Content-Type: application/json" -d '{"query": "tell me how to do copy"}' http://localhost:8081/query
```
Response:
```
{"result":["\n\nYou can use the `COPY INTO <table>` command to copy data from an internal stage, Amazon S3 bucket, or a remote file into a table in Databend. \n\nFor example, to copy data from an internal stage, you can use the following command:\n\n```\nCOPY INTO <table>\nFROM (\n    SELECT <columns>\n    FROM @<stage>\n    FILE_FORMAT = (TYPE = PARQUET)\n)\n```\n\nFor more information, please refer to the [Tutorial: Load from an internal stage](../../12-load-data/00-stage.md) and [Tutorial: Load from an Amazon S3 bucket](../../12-load-data/01-s3.md) sections in the Databend documentation."]}
```


