
CREATE TABLE accounts (
    id BIGSERIAL PRIMARY KEY,
    full_name VARCHAR NOT NULL,
    balance DOUBLE PRECISION NOT NULL,
    email VARCHAR NOT NULL, -- bisa digunakan untuk login
    phone_num VARCHAR NOT NULL, -- bisa digunakan untuk login
    active BOOLEAN NOT NULL,
    register_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- create nobody account
INSERT INTO accounts (id, full_name, balance, email, phone_num, active)
VALUES
(0, 'nobody', 0.0, 'nobody@nowhere.net', '', TRUE);


CREATE UNIQUE INDEX accounts_email ON accounts (
    (lower(email))
);
CREATE UNIQUE INDEX accounts_phone_num ON accounts (
    (lower(phone_num))
);


-- Berisi koleksi passhash dari akun
-- dibuat one-to-many agar ada history-nya setiap user merubah password.
CREATE TABLE account_passhash (
    account_id BIGINT PRIMARY KEY REFERENCES accounts(id) ON DELETE CASCADE,
    passhash VARCHAR NOT NULL,
    deprecated BOOLEAN NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Tabel untuk menampung user-user yang baru mendaftar tapi belum melakukan aktifasi
CREATE TABLE register_accounts (
    -- id BIGSERIAL PRIMARY KEY,
    token VARCHAR(100) PRIMARY KEY,
    full_name VARCHAR NOT NULL,
    email VARCHAR NOT NULL, -- untuk melakukan aktivasi via email
    phone_num VARCHAR NOT NULL, -- untuk melakukan aktivasi via phone (kalau tidak email)
    register_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX register_accounts_email ON register_accounts (
    (lower(email))
);
CREATE UNIQUE INDEX register_accounts_phone_num ON register_accounts (
    (lower(phone_num))
);

-- Tabel untuk alamat akun
CREATE TABLE addresses (
    id BIGSERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL DEFAULT 0 REFERENCES accounts (id) ON DELETE SET DEFAULT,
    kind INT NOT NULL DEFAULT 0, -- 0=Domisili, 1=Asli
    "address" TEXT NOT NULL,
    regency VARCHAR NOT NULL,
    province VARCHAR NOT NULL,
    country VARCHAR NOT NULL,
    phone_num VARCHAR NOT NULL,
    active BOOLEAN NOT NULL,
    notes TEXT NOT NULL DEFAULT ''
);

-- Koleksi key pair untuk akun.
CREATE TABLE account_keys (
    id BIGSERIAL PRIMARY KEY,
    account_id BIGINT NOT NULL DEFAULT 0 REFERENCES accounts (id) ON DELETE CASCADE,
    pub_key TEXT NOT NULL,
    secret_key TEXT NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    active BOOLEAN NOT NULL DEFAULT FALSE
);



CREATE TABLE access_tokens (
    token TEXT PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    created TIMESTAMP NOT NULL,
    valid_thru TIMESTAMP NOT NULL
);

CREATE INDEX idx_access_tokens_account_id ON access_tokens (
    (account_id)
);


CREATE SEQUENCE invoices_id_seq;

CREATE TABLE invoices (
    id BIGSERIAL PRIMARY KEY,
    id_ref TEXT NOT NULL, -- ID di sisi client/merchant.
    issuer_account BIGINT NOT NULL,
    to_account BIGINT NOT NULL,
    discount DOUBLE PRECISION NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    notes TEXT NOT NULL DEFAULT '',
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    paid BOOLEAN NOT NULL DEFAULT FALSE,
    paid_by BIGINT NOT NULL DEFAULT 0,
    paid_at TIMESTAMP
);

CREATE UNIQUE INDEX idx_invoices_id_ref ON invoices ((id));

ALTER SEQUENCE invoices_id_seq OWNED BY invoices.id;

CREATE TABLE invoice_items (
    id BIGSERIAL PRIMARY KEY,
    invoice_id BIGINT NOT NULL REFERENCES invoices (id),
    "name" VARCHAR(100) NOT NULL,
    price DOUBLE PRECISION NOT NULL
);

-- log/journal setiap operasi pembayaran akan dicatat di tabel ini.
CREATE TABLE payment_history (
    id BIGSERIAL PRIMARY KEY,
    invoice_id BIGINT NOT NULL REFERENCES invoices (id),
    payer BIGINT NOT NULL REFERENCES accounts (id),
    via VARCHAR(32) NOT NULL, -- via client, eg: Browser, App, other devices, etc.
    ts TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);


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



-- ini adalah transaction history local
-- digunakan di dalam sistem internal
CREATE TABLE IF NOT EXISTS transaction_histories (
  id                BIGSERIAL PRIMARY KEY NOT NULL,
  dbcr_flag         INT                   NOT NULL, --1 debit(ex. pay service yang bukan melalui merchant bukan doi, disburse),
                                                    --2 credit(ex. top up dr atm),
  ttype             INT                   NOT NULL, --transaction type 1 top up, 2 payment, 3 recharge ,4 transfer
  amount            DOUBLE PRECISION        NOT NULL,       -- Nominal transaksi
  "status"            INT                   NOT NULL, --0 success, 1 in progress, 2 timeout, 3 generic error, 4 cannot contact biller, 5 invalid dest account, 6 invalid from account, 7 insufficient balance
  created        TIMESTAMP             NOT NULL DEFAULT (now()),
  last_updated    TIMESTAMP             NOT NULL DEFAULT (now()),
  invoice_id           BIGINT,
  from_account_id       BIGINT, -- ID dari account pengirim.
  to_account_id         BIGINT, -- ID dari account penerima.
  merchant_id         BIGINT, -- ID dari merchant apabila jenisnya pembayaran ke merchant.
  notes             TEXT                 -- berisi catatan - catatan dari transaksi ini. Misal nomor debit card yang dipakai top up, kode voucher ketika isi pulsa dll
);


CREATE TABLE IF NOT EXISTS external_transaction_histories (
  id                BIGSERIAL PRIMARY KEY NOT NULL,
  internal_tx_id  BIGINT NOT NULL REFERENCES transaction_histories(id), -- mereferensi ke tabel `transactions`.
  ttype             INT                   NOT NULL, --transaction type 1 top up, 2 payment, 3 recharge ,4 transfer
  subttype          INT                   NOT NULL, --untuk payment (1 invoice, 2 pay service listrik, 3 pay service telpon)
  amount            DOUBLE PRECISION        NOT NULL,       -- Nominal transaksi
  "status"            INT                   NOT NULL, --0 success, 1 in progress, 2 timeout, 3 generic error, 4 cannot contact biller, 5 invalid dest account, 6 invalid from account, 7 insufficient balance
  created        TIMESTAMP             NOT NULL DEFAULT (now()),
  invoice_id  BIGINT,
  from_account_id       BIGINT,
  to_account_id         BIGINT,
  merchant_id         BIGINT,
  error_code  INT NOT NULL, -- kode error dari sistem external, 0=success, selain 0 adalah kegagalan.
  error_info  TEXT NOT NULL, -- informasi error dari sistem external, apabila `error_code` = 0 maka ini bisa string kosong atau null.
  notes             TEXT                 -- berisi catatan - catatan dari transaksi ini. Misal nomor debit card yang dipakai top up, kode voucher ketika isi pulsa dll
);

ALTER TABLE register_accounts ADD COLUMN code VARCHAR(10) NOT NULL;