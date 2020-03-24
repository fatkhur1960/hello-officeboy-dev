
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

