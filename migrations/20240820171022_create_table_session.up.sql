CREATE TABLE
  "session" (
    "session_id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "token" VARCHAR(255) NOT NULL,
    "ip_address" INET NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "expired_at" TIMESTAMP NOT NULL,
    PRIMARY KEY ("session_id"),
    FOREIGN KEY ("user_id") REFERENCES "user" ("user_id")
  );
