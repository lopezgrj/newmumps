CREATE TABLE IF NOT EXISTS institution (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    station_number TEXT,
    address_line1 TEXT,
    city TEXT,
    state TEXT,
    zip TEXT,
    phone TEXT
);
