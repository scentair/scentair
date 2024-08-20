CREATE TABLE
  "user" (
    "user_id" UUID NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "deleted_at" TIMESTAMP NOT NULL,
    PRIMARY KEY ("user_id")
  );

CREATE TABLE
  "user_profile" (
    "user_id" UUID NOT NULL,
    "name" VARCHAR(32) NOT NULL,
    "image" VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("user_id"),
    FOREIGN KEY ("user_id") REFERENCES "user" ("user_id")
  );
