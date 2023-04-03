CREATE DATABASE askbend;
USE askbend;

-- doc table.
CREATE TABLE doc (path VARCHAR, content VARCHAR, embedding ARRAY(FLOAT32));

-- rust table.
-- CREATE TABLE rust (path VARCHAR, content VARCHAR, embedding ARRAY(FLOAT32));
