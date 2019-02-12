
CREATE TABLE IF NOT EXISTS bank (
  id                BIGSERIAL PRIMARY KEY NOT NULL,
  name              VARCHAR(40)           UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS merchant (
  id                BIGSERIAL PRIMARY KEY NOT NULL,
  name              VARCHAR(40)           UNIQUE NOT NULL,
  balance           NUMERIC(15, 6)        NOT NULL,
  account_inst_id   BIGINT                REFERENCES bank(id),
  account_no        VARCHAR(255),
  account_id        BIGINT                NOT NULL REFERENCES accounts(id)
);


