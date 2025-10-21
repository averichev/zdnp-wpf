-- Таблица сведений о документе
CREATE TABLE document (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    number TEXT,
    date DATE,
    change_mark TEXT,
    uid TEXT NOT NULL UNIQUE
);
