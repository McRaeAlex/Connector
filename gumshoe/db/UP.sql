-- Creates the tables but doesn't drop them
CREATE TABLE issues (
    id PRIMARY KEY,
    title VARCHAR(256),
    body TEXT
    -- TODO come user who created it
);