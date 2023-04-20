CREATE DATABASE askbend;
USE askbend;

-- doc table.
CREATE TABLE doc (path VARCHAR, content VARCHAR, embedding ARRAY(FLOAT32));

-- doc query answer.
CREATE TABLE doc_answer(question VARCHAR, prompt VARCHAR, similar_distances ARRAY(FLOAT32), similar_sections VARCHAR, answer VARCHAR, ts TIMESTAMP);

