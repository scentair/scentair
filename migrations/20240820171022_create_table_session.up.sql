CREATE TABLE
  "session" (
    "session_id" UUID NOT NULL,
    "user_credential_id" UUID NOT NULL,
    "token" VARCHAR(255) NOT NULL,
    "ip_address" INET NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "expired_at" TIMESTAMP NOT NULL,
    PRIMARY KEY ("session_id"),
    FOREIGN KEY ("user_credential_id") REFERENCES "user_credential" ("user_credential_id")
  );
