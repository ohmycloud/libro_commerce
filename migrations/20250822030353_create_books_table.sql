-- Add migration script here
CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    price FLOAT8 NOT NULL,
    stock INTEGER NOT NULL DEFAULT 0,
    metadata JSONB
);
