-- Add up migration script here

CREATE TABLE IF NOT EXISTS authentication (
    id UUID DEFAULT gen_random_uuid(),
    username TEXT NOT NULL,
    pw_hash TEXT NOT NULL
);