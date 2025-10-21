-- Таблица авторов в неформализованной форме
CREATE TABLE neformal_author (
    author_id INTEGER NOT NULL REFERENCES authors(id) ON DELETE RESTRICT,
    name TEXT NOT NULL
);
