-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "bikes" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    description TEXT NOT NULL,
    model TEXT NOT NULL,
    created_at TIMESTAMP  NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP  NOT NULL DEFAULT current_timestamp
);