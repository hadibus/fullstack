-- Your SQL goes here

CREATE TABLE IF NOT EXISTS products (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  price MONEY NOT NULL,
  image TEXT NOT NULL,
  quantity INT NOT NULL,
  description TEXT NOT NULL
)
