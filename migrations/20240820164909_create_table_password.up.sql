CREATE TABLE
  "user_credential_password" (
    "user_credential_password_id" UUID NOT NULL,
    "user_credential_id" UUID NOT NULL,
    "value" VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "expired_at" TIMESTAMP NOT NULL,
    PRIMARY KEY ("user_credential_password_id"),
    FOREIGN KEY ("user_credential_id") REFERENCES "user_credential" ("user_credential_id")
  );

CREATE INDEX "index_01918d8b-3478-7d84-8c36-b457dfb1a40d" ON "user_credential_password" ("value");
