# AskBend: Leveraging Databend Cloud for Advanced AI Services

## Demo https://ask.databend.rs/

AskBend is a project built in Rust that leverages the [llmchain.rs](https://github.com/shafishlabs/llmchain.rs) library to create:
- SQL-based knowledge base from Markdown files
- GitHub Pull Request Summary

## Setup

<details>
  <summary>Setup Question&Answering</summary>

### 1. Download 

https://github.com/datafuselabs/askbend/releases

### 2. Modify the configuration file [conf/askbend.toml](conf/askbend.toml)

```
# Usage:
# askbend -c askbend.toml

[server]
host = "0.0.0.0"
port = 8081


[qa]
# Path to the directory containing your markdown documents
path = "data/"

database = "askbend"
table = "doc"
# Data source name (DSN) for connecting to your Databend cloud warehouse
# https://docs.databend.com/using-databend-cloud/warehouses/connecting-a-warehouse
dsn = "databend://<sql-user>:<sql-password>@<your-databend-cloud-warehouse>/default"
top = 3

```

### 3. Prepare your Markdown files by copying them to the `data/` directory

### 4. Parse the Markdown files and build embeddings

```
./target/release/askbend -c conf/askbend.toml --rebuild

[2023-04-01T07:17:13Z INFO ] Step-1: begin parser all markdown files
[2023-04-01T07:17:14Z INFO ] Step-1: finish parser all markdown files:397, sections:969, tokens:117758
[2023-04-01T07:17:14Z INFO ] Step-2: begin insert to table
[2023-04-01T07:17:14Z INFO ] Step-2: finish insert to table
[2023-04-01T07:17:14Z INFO ] Step-3: begin generate embedding, may take some minutes
[2023-04-01T07:26:03Z INFO ] Step-3: finish generate embedding
... ...
```

The `--rebuild` flag rebuilds all the embeddings for the data directory. This process may take a few minutes, depending on the number of Markdown files.


### 5. Start the API server

```
./target/release/askbend -c conf/askbend.toml
```

### 6. Query your Markdown knowledge base using the API
```
curl -X POST -H "Content-Type: application/json" -d '{"query": "tell me how to do copy"}' http://localhost:8081/qa/query
```
Response:
```
{"result":["\n\nYou can use the `COPY INTO <table>` command to copy data from an internal stage, Amazon S3 bucket, or a remote file into a table in Databend. \n\nFor example, to copy data from an internal stage, you can use the following command:\n\n```\nCOPY INTO <table>\nFROM (\n    SELECT <columns>\n    FROM @<stage>\n    FILE_FORMAT = (TYPE = PARQUET)\n)\n```\n\nFor more information, please refer to the [Tutorial: Load from an internal stage](../../12-load-data/00-stage.md) and [Tutorial: Load from an Amazon S3 bucket](../../12-load-data/01-s3.md) sections in the Databend documentation."]}
```

## AskBend Query API

This API document describes how to use the Databend query API to submit queries and receive results.

### Endpoint

http://<your-ip>:8081/qa/query

### Request

The request body should be a JSON object containing a single field `query`, which is the query string.

**Example:**

```json
{
    "query": "whats the fast way to load data to databend"
}
```

### Response

On successful query execution, the API will return a 200 OK status code, along with a JSON object containing the field result.

The result field is an array of strings. However, we only need to consider the first string in the array as the final result. 

The API assumes that if the query was successful, the first item in the result array is the most relevant answer.

## How to open the UI

To open the UI, you need to make sure that you have installed `Node` and the `Yarn/npm` package manager. Once you have confirmed this, you can proceed with the following steps:

```js
cd web
yarn
yarn run dev (or npm run dev)
```

If you want to expose your host:

```js
yarn run dev-host (or npm run dev-host)
```
</details>

<details>
  <summary>Setup Github Pull Request Summary</summary>

### 1. Download

https://github.com/datafuselabs/askbend/releases

### 2. Modify the configuration file [conf/askbend.toml](conf/askbend.toml)

```
# Usage:
# askbend -c askbend.toml

[server]
host = "0.0.0.0"
port = 8081


[github]
github_token = "your-github-token"
llm_max_tokens = 100000
# Data source name (DSN) for connecting to your Databend cloud warehouse
# https://docs.databend.com/using-databend-cloud/warehouses/connecting-a-warehouse
dsn = "databend://<sql-user>:<sql-password>@<your-databend-cloud-warehouse>/default"
repos = ["your-github-repo"]

```
### 3. Start the API server

```
./target/release/askbend -c conf/askbend.toml
```

</details>


