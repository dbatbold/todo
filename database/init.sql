DROP DATABASE IF EXISTS todo;

DROP ROLE IF EXISTS app;

CREATE ROLE app WITH LOGIN PASSWORD 'md5240aeccb7a1d859a99844d012bfe74cc';

CREATE DATABASE todo OWNER app;

\c todo;

SET ROLE app;

CREATE TABLE list (id serial primary key, title text not null);

INSERT INTO list (title) values ('first todo');
