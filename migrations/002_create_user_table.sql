CREATE TABLE IF NOT EXISTS user (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    title TEXT,
    service_section TEXT,
    ssn TEXT,
    dob TEXT,
    sex TEXT,
    phone TEXT,
    email TEXT
);
