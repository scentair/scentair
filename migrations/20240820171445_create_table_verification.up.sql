CREATE TABLE
  "verification" (
    "verification_id" UUID NOT NULL,
    "key" VARCHAR(32) NOT NULL,
    "name" VARCHAR(32) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("verification_id")
  );

CREATE UNIQUE INDEX "unique_01919694-76f1-79fd-9888-05ca3b6507d0" ON "verification" ("key");

CREATE UNIQUE INDEX "unique_019170c9-7a73-707c-9fe5-8fb7b1654fa7" ON "verification" ("name");

INSERT INTO
  "verification" ("verification_id", "key", "name")
VALUES
  (
    '019170cb-2286-71b4-9404-8442808614d8',
    'system::email_verified',
    'Email Verified'
  );

CREATE TABLE
  "user_verification" (
    "user_verification_id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "verification_id" UUID NOT NULL,
    "token" VARCHAR(255) NOT NULL,
    "payload" JSONB NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "expired_at" TIMESTAMP NOT NULL,
    "verified_at" TIMESTAMP NOT NULL,
    PRIMARY KEY ("user_verification_id"),
    FOREIGN KEY ("user_id") REFERENCES "user" ("user_id"),
    FOREIGN KEY ("verification_id") REFERENCES "verification" ("verification_id")
  );

CREATE INDEX "unique_019170d0-bd55-7e97-a38b-788d4df8f3b2" ON "user_verification" ("user_id", "verification_id");
