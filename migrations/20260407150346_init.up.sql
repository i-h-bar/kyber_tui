-- Add up migration script here

CREATE TABLE IF NOT EXISTS authentication (
    id UUID UNIQUE DEFAULT gen_random_uuid(),
    username TEXT UNIQUE NOT NULL,
    pw_hash TEXT NOT NULL
);