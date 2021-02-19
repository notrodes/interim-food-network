-- This file should undo anything in `up.sql`
ALTER TABLE listings RENAME TO _listings_old;

CREATE TABLE listings (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

INSERT INTO listings (id, name)
  SELECT id, name
  FROM _listings_old;

DROP TABLE _listings_old;