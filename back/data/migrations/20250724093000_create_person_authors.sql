-- Таблица авторов для физических лиц
CREATE TABLE person_authors (
    person_id INTEGER NOT NULL REFERENCES person(id) ON DELETE CASCADE,
    author_id INTEGER NOT NULL REFERENCES authors(id) ON DELETE RESTRICT
);
