
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
    account_id BIGINT NOT NULL REFERENCES accounts (id),
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
    paid_by BIGINT NOT NULL REFERENCES accounts (id) DEFAULT 0,
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

