-- Таблица физических лиц
CREATE TABLE person (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    patronymic TEXT,
    surname TEXT NOT NULL,
    snils INTEGER NOT NULL,
    email TEXT NOT NULL,
    address_id INTEGER NOT NULL REFERENCES address(id) ON DELETE RESTRICT
);
