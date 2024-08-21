CREATE TABLE
  "organization" (
    "organization_id" UUID NOT NULL,
    "created_by" UUID NOT NULL,
    "name" VARCHAR(32) NOT NULL,
    "handle" VARCHAR(32) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("organization_id"),
    FOREIGN KEY ("created_by") REFERENCES "user" ("user_id")
  );

CREATE UNIQUE INDEX "unique_019172b8-787b-7c05-af61-8973c1846b66" ON "organization" ("name");

CREATE UNIQUE INDEX "unique_019172b8-e286-7f9b-874f-42de82ea09ab" ON "organization" ("handle");

CREATE TABLE
  "member" (
    "member_id" UUID NOT NULL,
    "organization_id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("member_id"),
    FOREIGN KEY ("organization_id") REFERENCES "organization" ("organization_id"),
    FOREIGN KEY ("user_id") REFERENCES "user" ("user_id")
  );

CREATE UNIQUE INDEX "unique_019172b7-b49e-7e86-8059-f206e1833c46" ON "member" ("organization_id", "user_id");
