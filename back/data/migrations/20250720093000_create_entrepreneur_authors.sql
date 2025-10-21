-- Таблица авторов для индивидуальных предпринимателей
CREATE TABLE entrepreneur_authors (
    entrepreneur_id INTEGER NOT NULL REFERENCES entrepreneur(id) ON DELETE CASCADE,
    author_id INTEGER NOT NULL REFERENCES authors(id) ON DELETE RESTRICT
);
