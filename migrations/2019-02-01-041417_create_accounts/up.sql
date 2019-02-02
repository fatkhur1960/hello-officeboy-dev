
CREATE TABLE accounts (
    id BIGSERIAL PRIMARY KEY,
    full_name VARCHAR NOT NULL,
    balance DOUBLE PRECISION NOT NULL,
    email VARCHAR NOT NULL, -- bisa digunakan untuk login
    phone_num VARCHAR NOT NULL, -- bisa digunakan untuk login
    active BOOLEAN NOT NULL,
    register_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Berisi koleksi passhash dari akun
-- dibuat one-to-many agar ada history-nya setiap user merubah password.
CREATE TABLE account_passhash (
    account_id BIGINT PRIMARY KEY REFERENCES accounts(id),
    passhash VARCHAR NOT NULL,
    deprecated BOOLEAN NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Tabel untuk menampung user-user yang baru mendaftar tapi belum melakukan aktifasi
CREATE TABLE register_accounts (
    id BIGSERIAL PRIMARY KEY,
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
    account_id BIGINT NOT NULL,
    kind INT NOT NULL DEFAULT 0, -- 0=Domisili, 1=Asli
    "address" TEXT NOT NULL,
    regency VARCHAR NOT NULL,
    province VARCHAR NOT NULL,
    country VARCHAR NOT NULL,
    phone_num VARCHAR NOT NULL,
    active BOOLEAN NOT NULL,
    notes TEXT NOT NULL DEFAULT ''
);
