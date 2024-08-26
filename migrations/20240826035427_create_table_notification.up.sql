CREATE TABLE
  "device" (
    "device_id" UUID NOT NULL,
    "name" VARCHAR(32) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("device_id")
  );

CREATE UNIQUE INDEX "unique_01918cdc-77f0-7622-8ed0-b5833d0e5e1d" ON "device" ("name");

INSERT INTO
  "device" ("device_id", "name")
VALUES
  ('01918ce4-f165-752d-ac6d-38c32d4e6129', 'EMAIL'),
  ('01918ce5-40d4-7f27-8f78-b26e0783a589', 'SMS'),
  ('01918ce4-8441-73ab-aa4b-032655f8c595', 'ANDROID'),
  ('01918ce4-b9bc-76b1-b5bb-da38efbcf198', 'IOS');

CREATE TABLE
  "user_device" (
    "user_device_id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "device_id" UUID NOT NULL,
    "key" VARCHAR(128) NOT NULL,
    "token" VARCHAR(128) NOT NULL,
    "config" JSONB NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("user_device_id"),
    FOREIGN KEY ("user_id") REFERENCES "user" ("user_id"),
    FOREIGN KEY ("device_id") REFERENCES "device" ("device_id")
  );

CREATE INDEX "index_01918cdc-fcb9-7447-9358-644667d6e463" ON "user_device" ("user_id", "device_id");

CREATE UNIQUE INDEX "unique_01918ce0-1715-7909-8710-ad9d6f2328f5" ON "user_device" ("key");

CREATE TABLE
  "notification" (
    "notification_id" UUID NOT NULL,
    "name" VARCHAR(32) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("notification_id")
  );

CREATE UNIQUE INDEX "unique_01918cd4-5c73-73c2-ace4-592e1925e101" ON "notification" ("name");

INSERT INTO
  "notification" ("notification_id", "name")
VALUES
  ('01918cd5-8eb4-7a78-8e31-3ec8efcd8cc7', 'FCM');

CREATE TABLE
  "notification_template" (
    "notification_template_id" UUID NOT NULL,
    "notification_id" UUID NOT NULL,
    "language_id" UUID NOT NULL,
    "title" VARCHAR(128) NOT NULL,
    "content" TEXT NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("notification_template_id"),
    FOREIGN KEY ("notification_id") REFERENCES "notification" ("notification_id"),
    FOREIGN KEY ("language_id") REFERENCES "language" ("language_id")
  );

CREATE UNIQUE INDEX "unique_01918d37-7e6d-7da6-99d3-dc8dfc4d08c8" ON "notification_template" ("notification_id", "language_id");

CREATE TABLE
  "notification_bcc" (
    "notification_bcc_id" UUID NOT NULL,
    "notification_id" UUID NOT NULL,
    "user_device_id" UUID NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("notification_bcc_id"),
    FOREIGN KEY ("notification_id") REFERENCES "notification" ("notification_id"),
    FOREIGN KEY ("user_device_id") REFERENCES "user_device" ("user_device_id")
  );

CREATE UNIQUE INDEX "unique_01918d4b-3f03-7b26-8cf4-2874a127f34b" ON "notification_bcc" ("notification_id", "user_device_id");

CREATE TABLE
  "user_notification" (
    "user_notification_id" UUID NOT NULL,
    "user_device_id" UUID NOT NULL,
    "notification_id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "notification_template_id" UUID NOT NULL,
    "title" VARCHAR(128) NOT NULL,
    "content" TEXT NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("user_notification_id"),
    FOREIGN KEY ("user_device_id") REFERENCES "user_device" ("user_device_id"),
    FOREIGN KEY ("notification_id") REFERENCES "notification" ("notification_id"),
    FOREIGN KEY ("user_id") REFERENCES "user" ("user_id"),
    FOREIGN KEY ("notification_template_id") REFERENCES "notification_template" ("notification_template_id")
  );

CREATE INDEX "index_01918cd6-6f05-7f3e-ba81-4c5cdbdba180" ON "user_notification" ("user_device_id", "notification_id");

CREATE TABLE
  "user_notification_bcc" (
    "user_notification_bcc_id" UUID NOT NULL,
    "user_notification_id" UUID NOT NULL,
    "user_device_id" UUID NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("user_notification_bcc_id"),
    FOREIGN KEY ("user_notification_id") REFERENCES "user_notification" ("user_notification_id"),
    FOREIGN KEY ("user_device_id") REFERENCES "user_device" ("user_device_id")
  );

CREATE UNIQUE INDEX "unique_01918d4b-b570-7f02-accf-f2d056f98c36" ON "user_notification_bcc" ("user_notification_id", "user_device_id");
