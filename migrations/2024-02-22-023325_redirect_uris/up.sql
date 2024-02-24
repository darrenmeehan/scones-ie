CREATE TABLE redirect_uris (
    id SERIAL PRIMARY KEY,
    uri VARCHAR(255) NOT NULL,
    client_id VARCHAR(255) NOT NULL REFERENCES clients(client_id)
)