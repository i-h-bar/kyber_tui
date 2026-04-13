-- Add down migration script here
DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS credentials CASCADE;
DROP TABLE IF EXISTS notes CASCADE;