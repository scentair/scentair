CREATE TABLE
  "credential" (
    "credential_id" UUID NOT NULL,
    "name" VARCHAR(32) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("credential_id")
  );

CREATE UNIQUE INDEX "unique_019170ae-24cd-7024-b4ca-7a5e2ae8541f" ON "credential" ("name");

INSERT INTO
  "credential" ("credential_id", "name")
VALUES
  ('019170b0-609a-79fe-9314-044d14863643', 'local');

CREATE TABLE
  "user_credential" (
    "user_credential_id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "credential_id" UUID NOT NULL,
    "payload" JSONB NOT NULL,
    "hash" VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "expired_at" TIMESTAMP NOT NULL,
    PRIMARY KEY ("user_credential_id"),
    FOREIGN KEY ("user_id") REFERENCES "user" ("user_id"),
    FOREIGN KEY ("credential_id") REFERENCES "credential" ("credential_id")
  );

CREATE INDEX "unique_019170dd-ae5a-7e28-a352-19b6a13e13bb" ON "user_credential" ("hash");
