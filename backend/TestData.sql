-- Adminer 4.8.1 PostgreSQL 15.3 (Debian 15.3-1.pgdg120+1) dump

DROP TABLE IF EXISTS "projects";
CREATE TABLE "public"."projects" (
    "id" uuid NOT NULL,
    "name" text NOT NULL,
    "image" text NOT NULL,
    "status" integer NOT NULL
) WITH (oids = false);

INSERT INTO "projects" ("id", "name", "image", "status") VALUES
('4c38bf38-59f1-4603-9438-e2766663cf2b',	'Test project',	'testImage.jpg',	0),
('1e33f43d-0193-460a-9128-bffb1d12e57c',	'Test project 2',	'testimage2.jpg',	1);

-- 2023-08-01 12:25:24.276557+00