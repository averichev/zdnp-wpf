CREATE TABLE address
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    region_code TEXT NOT NULL,
    note        TEXT,
    country     TEXT,
    district    TEXT,
    city        TEXT,
    settlement  TEXT,
    street      TEXT,
    building    TEXT,
    room        TEXT
);
