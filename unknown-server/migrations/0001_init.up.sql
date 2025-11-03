-- Create users
CREATE TABLE IF NOT EXISTS users
(
    id       uuid PRIMARY KEY NOT NULL,
    username text             NOT NULL,
    email    text             NOT NULL,
    created  timestamptz      NOT NULL default now(),
    modified timestamptz      NOT NULL default now(),
    pw_hash  text             NOT NULL
);