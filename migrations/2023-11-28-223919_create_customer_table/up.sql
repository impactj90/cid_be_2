-- Your SQL goes here
CREATE TABLE customers (
    id SERIAL PRIMARY KEY NOT NULL,
    phone VARCHAR(20),
    name VARCHAR(255),
    zip VARCHAR(10),
    city VARCHAR(50),
    street VARCHAR(255),
    created_at TIMESTAMP,
    updated_at TIMESTAMP
)

