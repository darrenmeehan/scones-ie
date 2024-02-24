CREATE TABLE clients (
    id SERIAL PRIMARY KEY,
    client_type VARCHAR(255) NOT NULL,
    client_id VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    website VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL
)
