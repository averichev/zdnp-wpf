CREATE TABLE organization (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    full_name TEXT NOT NULL,
    abbreviated_name TEXT NOT NULL,
    ogrn INTEGER,
    rafp INTEGER,
    inn INTEGER NOT NULL,
    kpp INTEGER NOT NULL,
    address_id INTEGER NOT NULL REFERENCES address(id) ON DELETE RESTRICT,
    email TEXT NOT NULL
);
