-- Your SQL goes here
CREATE TABLE customers (
  id VARCHAR(36) PRIMARY KEY,
  first_name VARCHAR(255) NOT NULL,
  last_name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  hotel_id INTEGER NULL, 
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
)