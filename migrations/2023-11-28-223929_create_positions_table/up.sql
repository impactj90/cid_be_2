-- Your SQL goes here
CREATE TABLE positions (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255),
    created_at TIMESTAMP,
    updated_at TIMESTAMP
    -- other position-related attributes
);

