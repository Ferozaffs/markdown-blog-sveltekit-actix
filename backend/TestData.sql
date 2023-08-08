-- Adminer 4.8.1 PostgreSQL 15.3 (Debian 15.3-1.pgdg120+1) dump

\connect "postgres";

DROP TABLE IF EXISTS "project_categories";
CREATE TABLE "public"."project_categories" (
    "id" uuid NOT NULL,
    "category" text NOT NULL,
    "description" text
) WITH (oids = false);

INSERT INTO "project_categories" ("id", "category", "description") VALUES
('71f99ce7-64c6-4344-927a-8fa15acd357e',	'Large',	'Project spanning a long time'),
('c9284c8d-02a4-4b25-9818-20c8a95178c6',	'Small',	'Projects spanning a short time'),
('6ce5b42a-0c54-4aec-b2a5-0b8b4e538d4b',	'Misc',	'Random projects');

DROP TABLE IF EXISTS "projects";
CREATE TABLE "public"."projects" (
    "id" uuid NOT NULL,
    "name" text NOT NULL,
    "image" text NOT NULL,
    "status" integer NOT NULL,
    "category_id" uuid,
    "content" text
) WITH (oids = false);

INSERT INTO "projects" ("id", "name", "image", "status", "category_id", "content") VALUES
('4c38bf38-59f1-4603-9438-e2766663cf2b',	'Test project',	'testImage.jpg',	0,	'71f99ce7-64c6-4344-927a-8fa15acd357e',	'## Lorem

Lorem is currently extended with the following plugins.
Instructions on how to use them in your application are linked below.

![TestImage](http://127.0.0.1:8080/images/testImage.jpg)

| Plugin | README |
| ------ | ------ |
| Dropbox | [plugins/dropbox/README.md](Link) |
| Medium | [plugins/medium/README.md](Link) |
| Google Analytics | [plugins/googleanalytics/README.md](Link) |'),
('1e33f43d-0193-460a-9128-bffb1d12e57c',	'Test project 2',	'testImage2.jpg',	1,	'c9284c8d-02a4-4b25-9818-20c8a95178c6',	'## Lorem

Lorem is currently extended with the following plugins.
Instructions on how to use them in your application are linked below.

![TestImage](http://127.0.0.1:8080/images/testImage2.jpg)

| Plugin | README |
| ------ | ------ |
| Dropbox | [plugins/dropbox/README.md](Link) |
| Medium | [plugins/medium/README.md](Link) |
| Google Analytics | [plugins/googleanalytics/README.md](Link) |');

-- 2023-08-08 14:23:14.53423+00