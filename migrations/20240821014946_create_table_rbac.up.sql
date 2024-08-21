CREATE TABLE
  "role" (
    "role_id" UUID NOT NULL,
    "organization_id" UUID NOT NULL,
    "created_by" UUID NOT NULL,
    "name" VARCHAR(32) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("role_id"),
    FOREIGN KEY ("organization_id") REFERENCES "organization" ("organization_id"),
    FOREIGN KEY ("created_by") REFERENCES "member" ("member_id")
  );

CREATE UNIQUE INDEX "unique_019172c2-9ba3-71fa-9550-c142b2799c93" ON "role" ("organization_id", "name");

CREATE TABLE
  "permission" (
    "permission_id" UUID NOT NULL,
    "organization_id" UUID NOT NULL,
    "created_by" UUID NOT NULL,
    "name" VARCHAR(32) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("permission_id"),
    FOREIGN KEY ("organization_id") REFERENCES "organization" ("organization_id"),
    FOREIGN KEY ("created_by") REFERENCES "member" ("member_id")
  );

CREATE UNIQUE INDEX "uniuqe_019172c2-b957-7069-85c1-d2fb637f2648" ON "permission" ("organization_id", "name");

CREATE TABLE
  "role_permission" (
    "role_permission_id" UUID NOT NULL,
    "role_id" UUID NOT NULL,
    "permission_id" UUID NOT NULL,
    "attached_by" UUID NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("role_permission_id"),
    FOREIGN KEY ("role_id") REFERENCES "role" ("role_id"),
    FOREIGN KEY ("permission_id") REFERENCES "permission" ("permission_id"),
    FOREIGN KEY ("attached_by") REFERENCES "member" ("member_id")
  );

CREATE UNIQUE INDEX "unique_019172a1-5d74-7992-89fb-8087d2269439" ON "role_permission" ("role_id", "permission_id");

CREATE TABLE
  "member_role" (
    "member_role_id" UUID NOT NULL,
    "member_id" UUID NOT NULL,
    "role_id" UUID NOT NULL,
    "attached_by" UUID NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("member_role_id"),
    FOREIGN KEY ("member_id") REFERENCES "member" ("member_id"),
    FOREIGN KEY ("role_id") REFERENCES "role" ("role_id")
  );

CREATE UNIQUE INDEX "unique_019172aa-fbd8-7e2e-a8aa-9c8ce6a145b0" ON "member_role" ("member_id", "role_id");

CREATE TABLE
  "member_permission" (
    "member_permission_id" UUID NOT NULL,
    "member_id" UUID NOT NULL,
    "permission_id" UUID NOT NULL,
    "attached_by" UUID NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("member_permission_id"),
    FOREIGN KEY ("member_id") REFERENCES "member" ("member_id"),
    FOREIGN KEY ("permission_id") REFERENCES "permission" ("permission_id")
  );

CREATE UNIQUE INDEX "unique_019172ab-4b7a-78a9-bfd1-67552dbee33c" ON "member_permission" ("member_id", "permission_id");
