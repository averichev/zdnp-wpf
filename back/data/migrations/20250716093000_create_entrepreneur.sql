-- Таблица индивидуальных предпринимателей
CREATE TABLE entrepreneur (
    id INTEGER PRIMARY KEY AUTOINCREMENT, -- Идентификатор записи
    surname TEXT NOT NULL,               -- Фамилия
    name TEXT NOT NULL,                  -- Имя
    patronymic TEXT,                     -- Отчество
    ogrnip INTEGER NOT NULL,             -- Основной государственный регистрационный номер
    inn INTEGER NOT NULL,                         -- Индивидуальный номер налогоплательщика
    address_id INTEGER NOT NULL REFERENCES address(id) ON DELETE RESTRICT, -- Почтовый адрес
    email TEXT                           -- Адрес электронной почты
);
