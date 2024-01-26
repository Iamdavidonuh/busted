-- Add migration script here
-- Add migration script here

-- Create subscriptions Table

CREATE TABLE courses(
	id uuid NOT NULL,
	PRIMARY KEY(id),
	name TEXT NOT NULL UNIQUE,
    author TEXT NOT NULL UNIQUE,
	subscribed_at timestamptz NOT NULL
);