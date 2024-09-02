CREATE TABLE
  "credential" (
    "credential_id" UUID NOT NULL,
    "key" VARCHAR(32) NOT NULL,
    "name" VARCHAR(32) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("credential_id")
  );

CREATE UNIQUE INDEX "unique_01919692-cda8-7741-a2a5-d8d35b5ece60" ON "credential" ("key");

CREATE UNIQUE INDEX "unique_019170ae-24cd-7024-b4ca-7a5e2ae8541f" ON "credential" ("name");

INSERT INTO
  "credential" ("credential_id", "key", "name")
VALUES
  (
    '019170b0-609a-79fe-9314-044d14863643',
    'system::local',
    'Local'
  ),
  (
    '01918d67-5721-7c47-b66e-39be161e6322',
    'system::otp',
    'OTP'
  );

CREATE TABLE
  "user_credential" (
    "user_credential_id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "credential_id" UUID NOT NULL,
    "value" VARCHAR(64) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "expired_at" TIMESTAMP NOT NULL,
    PRIMARY KEY ("user_credential_id"),
    FOREIGN KEY ("user_id") REFERENCES "user" ("user_id"),
    FOREIGN KEY ("credential_id") REFERENCES "credential" ("credential_id")
  );

CREATE INDEX "index_019170dd-ae5a-7e28-a352-19b6a13e13bb" ON "user_credential" ("value");
