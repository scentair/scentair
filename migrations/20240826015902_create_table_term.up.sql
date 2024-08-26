CREATE TABLE
  "language" (
    "language_id" UUID NOT NULL,
    "name" VARCHAR(32) NOT NULL,
    "code" CHAR(2) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("language_id")
  );

CREATE INDEX "index_01918d0d-5367-7d48-96c1-8f177cefc077" ON "language" ("name");

CREATE UNIQUE INDEX "unique_01918d0d-7400-7996-b2e1-a0067b849e04" ON "language" ("code");

INSERT INTO
  "language" ("language_id", "name", "code")
VALUES
  (
    '01918d0d-d569-755e-a5ba-33d390e71ed9',
    'English',
    'en'
  ),
  (
    '01918d0d-de27-7962-a016-0060fc3f6dcb',
    '한국어',
    'ko'
  );

CREATE TABLE
  "term" (
    "term_id" UUID NOT NULL,
    "name" VARCHAR(32) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("term_id")
  );

CREATE UNIQUE INDEX "unique_01918c4d-74c1-7371-8f98-d23c3dfead41" ON "term" ("name");

INSERT INTO
  "term" ("term_id", "name")
VALUES
  (
    '01918cf9-86eb-7769-8aaf-eef6f5b0015a',
    'TERMS_OF_SERVICE'
  ),
  (
    '01918cf9-9854-7a5e-b990-ce0ff48c14eb',
    'PRIVACY_POLICY'
  ),
  (
    '01918cf9-b19a-7e0b-a266-879b86cd86f0',
    'MARKETING_CONSENT'
  );

CREATE TABLE
  "term_content" (
    "term_content_id" UUID NOT NULL,
    "term_id" UUID NOT NULL,
    "language_id" UUID NOT NULL,
    "version" INTEGER NOT NULL,
    "interval" INTERVAL NOT NULL,
    "is_mandatory" BOOLEAN NOT NULL DEFAULT TRUE,
    "content" TEXT NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("term_content_id"),
    FOREIGN KEY ("term_id") REFERENCES "term" ("term_id"),
    FOREIGN KEY ("language_id") REFERENCES "language" ("language_id")
  );

CREATE UNIQUE INDEX "unique_01918d12-fbb2-790d-be24-7e64cf53b69c" ON "term_content" ("term_id", "language_id", "version");

CREATE TABLE
  "user_term" (
    "user_term_id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "term_id" UUID NOT NULL,
    "interval" INTERVAL NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "accepted_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "revoked_at" TIMESTAMP NOT NULL DEFAULT '2199-12-31 23:59:59',
    "expired_at" TIMESTAMP NOT NULL GENERATED ALWAYS AS (accepted_at + interval) STORED,
    PRIMARY KEY ("user_term_id"),
    FOREIGN KEY ("user_id") REFERENCES "user" ("user_id"),
    FOREIGN KEY ("term_id") REFERENCES "term" ("term_id")
  );

CREATE INDEX "index_01918c4e-8524-772e-adc8-c27d41ae3bb6" ON "user_term" ("user_id", "term_id");
