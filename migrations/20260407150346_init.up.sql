-- Add up migration script here

CREATE TABLE IF NOT EXISTS users (
    id UUID UNIQUE DEFAULT gen_random_uuid() PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    pw_hash TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS credentials (
    id UUID UNIQUE DEFAULT gen_random_uuid() PRIMARY KEY,
    service BYTEA NOT NULL,
    username BYTEA,
    password BYTEA NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id),
    version INT DEFAULT 1
);

CREATE TABLE IF NOT EXISTS notes (
    id UUID UNIQUE DEFAULT gen_random_uuid() PRIMARY KEY,
    credential_id UUID NOT NULL REFERENCES credentials(id),
    content BYTEA NOT NULL
);
