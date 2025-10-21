-- Таблица авторов для организаций
CREATE TABLE organization_authors (
    organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
    author_id INTEGER NOT NULL REFERENCES authors(id) ON DELETE RESTRICT
);
