CREATE TABLE
  "term" (
    "term_id" UUID NOT NULL,
    "name" VARCHAR(32) NOT NULL,
    "interval" INTERVAL NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("term_id")
  );

CREATE UNIQUE INDEX "unique_01918c4d-74c1-7371-8f98-d23c3dfead41" ON "term" ("name");

CREATE TABLE
  "user_term" (
    "user_term_id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "term_id" UUID NOT NULL,
    "value" JSONB NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "expired_at" TIMESTAMP NOT NULL,
    PRIMARY KEY ("user_term_id"),
    FOREIGN KEY ("user_id") REFERENCES "user" ("user_id"),
    FOREIGN KEY ("term_id") REFERENCES "term" ("term_id")
  );

CREATE INDEX "unique_01918c4e-8524-772e-adc8-c27d41ae3bb6" ON "user_term" ("user_id", "term_id");
