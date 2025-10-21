-- Таблица типов авторов
CREATE TABLE author_type (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);

INSERT INTO author_type (name) VALUES
    ('Организация'),
    ('Индивидуальный предприниматель'),
    ('Физическое лицо'),
    ('Автор в неформализованной форме');
