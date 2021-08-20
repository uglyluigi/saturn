-- Your SQL goes here
CREATE TABLE clubs (
  id SERIAL PRIMARY KEY,
  maintainer INT NOT NULL,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  publish_date timestamp with TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  expiry_date timestamp with TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email TEXT NOT NULL UNIQUE
);