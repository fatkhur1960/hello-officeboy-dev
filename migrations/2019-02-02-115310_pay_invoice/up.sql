
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

