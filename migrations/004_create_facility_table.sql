CREATE TABLE IF NOT EXISTS facility (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    type_code TEXT,
    address_line1 TEXT,
    city TEXT,
    state TEXT,
    zip TEXT,
    phone TEXT
);
