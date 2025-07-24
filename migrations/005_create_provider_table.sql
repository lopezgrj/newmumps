CREATE TABLE IF NOT EXISTS provider (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    npi TEXT,
    specialty TEXT,
    phone TEXT,
    email TEXT
);
