CREATE TABLE
  "channel" (
    "channel_id" UUID NOT NULL,
    "key" VARCHAR(32) NOT NULL,
    "name" VARCHAR(32) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("channel_id")
  );

CREATE UNIQUE INDEX "unique_0191968a-86eb-7a44-ac9c-99c7aba57c31" ON "channel" ("name");

CREATE UNIQUE INDEX "unique_0191968a-94af-708b-8201-df2aaaba314f" ON "channel" ("key");

INSERT INTO
  "channel" ("channel_id", "key", "name")
VALUES
  (
    '0191968a-86eb-7a44-ac9c-99c7aba57c31',
    'system::email',
    'Email'
  ),
  (
    '0191968a-94af-708b-8201-df2aaaba314f',
    'system::sms',
    'SMS'
  ),
  (
    '0191968a-9b1e-7b5e-8b9b-8f2aaaba314f',
    'system::push',
    'Push'
  );

CREATE TABLE
  "device" (
    "device_id" UUID NOT NULL,
    "key" VARCHAR(32) NOT NULL,
    "name" VARCHAR(32) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("device_id")
  );

CREATE UNIQUE INDEX "unique_01918cdc-77f0-7622-8ed0-b5833d0e5e1d" ON "device" ("name");

CREATE UNIQUE INDEX "unique_0191968a-67f7-7b5e-bf24-c8136fe1dbdb" ON "device" ("key");

INSERT INTO
  "device" ("device_id", "key", "name")
VALUES
  (
    '01918ce4-8441-73ab-aa4b-032655f8c595',
    'system::android',
    'Android'
  ),
  (
    '01918ce4-b9bc-76b1-b5bb-da38efbcf198',
    'system::ios',
    'iOS'
  );

CREATE TABLE
  "user_device" (
    "user_device_id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "device_id" UUID NOT NULL,
    "device_key" VARCHAR(128) NOT NULL,
    "device_token" VARCHAR(128) NOT NULL,
    "config" JSONB NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "accepted_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "revoked_at" TIMESTAMP NOT NULL DEFAULT '2199-12-31 23:59:59',
    PRIMARY KEY ("user_device_id"),
    FOREIGN KEY ("user_id") REFERENCES "user" ("user_id"),
    FOREIGN KEY ("device_id") REFERENCES "device" ("device_id")
  );

CREATE INDEX "index_01918cdc-fcb9-7447-9358-644667d6e463" ON "user_device" ("user_id", "device_id");

CREATE UNIQUE INDEX "unique_01918ce0-1715-7909-8710-ad9d6f2328f5" ON "user_device" ("device_key");

CREATE TABLE
  "notification" (
    "notification_id" UUID NOT NULL,
    "channel_id" UUID NOT NULL,
    "key" VARCHAR(32) NOT NULL,
    "name" VARCHAR(32) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("notification_id"),
    FOREIGN KEY ("channel_id") REFERENCES "channel" ("channel_id")
  );

CREATE UNIQUE INDEX "unique_0191968f-e4af-7194-83c7-d38199f23242" ON "notification" ("key");

CREATE UNIQUE INDEX "unique_01918cd4-5c73-73c2-ace4-592e1925e101" ON "notification" ("name");

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
    "sent_by" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "notification_template_id" UUID NOT NULL,
    "title" VARCHAR(128) NOT NULL,
    "content" TEXT NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("user_notification_id"),
    FOREIGN KEY ("user_device_id") REFERENCES "user_device" ("user_device_id"),
    FOREIGN KEY ("notification_id") REFERENCES "notification" ("notification_id"),
    FOREIGN KEY ("sent_by") REFERENCES "member" ("member_id"),
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
