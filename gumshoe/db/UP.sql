CREATE TABLE issues (
    id SERIAL PRIMARY KEY,
    title VARCHAR(256) NOT NULL,
    body TEXT NOT NULL
);
