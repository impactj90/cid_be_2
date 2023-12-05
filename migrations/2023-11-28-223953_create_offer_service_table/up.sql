-- Your SQL goes here
CREATE TABLE offer_services (
    id SERIAL PRIMARY KEY NOT NULL,
    offer_id INT NOT NULL,
    service_id INT NOT NULL,
    position_id INT NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    FOREIGN KEY (offer_id) REFERENCES offers(id),
    FOREIGN KEY (service_id) REFERENCES services(id),
    FOREIGN KEY (position_id) REFERENCES positions(id)
)

