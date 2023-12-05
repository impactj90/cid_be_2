-- Your SQL goes here
CREATE TABLE services (
    id SERIAL PRIMARY KEY NOT NULL,
    service_text VARCHAR(255) NOT NULL,
    quantity_unit VARCHAR(50) NOT NULL,
    quantity INT NOT NULL,
    price_per_unit DECIMAL(10, 2) NOT NULL,
    total_cost DECIMAL(10, 2) NOT NULL,

    created_at TIMESTAMP,
    updated_at TIMESTAMP
    -- other service-related attributes
);

