-- Таблица разработчиков для организаций
CREATE TABLE organization_developer (
    organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
    developer_id INTEGER NOT NULL REFERENCES developer(id) ON DELETE RESTRICT
);
