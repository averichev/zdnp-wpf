-- Таблица разработчиков для физических лиц
CREATE TABLE person_developer (
    person_id INTEGER NOT NULL REFERENCES person(id) ON DELETE CASCADE,
    developer_id INTEGER NOT NULL REFERENCES developer(id) ON DELETE RESTRICT
);
