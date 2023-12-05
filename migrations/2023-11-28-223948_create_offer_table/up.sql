-- Your SQL goes here
CREATE TABLE offers (
    id SERIAL PRIMARY KEY NOT NULL,
    customer_id INT NOT NULL,
    name VARCHAR(255),
    date DATE, 
    customer_text TEXT, 
    subject VARCHAR(255), 
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    -- other offer-related attributes
    FOREIGN KEY (customer_id) REFERENCES customers(id)
)

