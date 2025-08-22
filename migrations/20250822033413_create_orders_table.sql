-- Add migration script here

CREATE TYPE order_status AS ENUM ('pending', 'confirmed', 'shipped', 'cancelled');
CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    book_ids INTEGER[] NOT NULL,
    total FLOAT8 NOT NULL,
    status order_status NOT NULL DEFAULT 'pending'
);
