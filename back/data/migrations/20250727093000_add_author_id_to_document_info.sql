-- Добавление поля author_id в таблицу сведений о документе
ALTER TABLE document
ADD COLUMN author_id INTEGER NOT NULL REFERENCES authors(id) ON DELETE RESTRICT;
