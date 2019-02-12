
CREATE TABLE IF NOT EXISTS transactions (
  id                BIGSERIAL PRIMARY KEY NOT NULL,
  dbcr_flag         INT                   NOT NULL, --1 debit(ex. pay service yang bukan melalui merchant bukan doi, disburse),
                                                    --2 credit(ex. top up dr atm),
                                                    --3 debit credit(ex. transfer or pay wallet atau merchant doi)
  ttype             INT                   NOT NULL, --transaction type 1 top up, 2 payment, 3 recharge ,4 transfer
  subttype          INT                   NOT NULL, --untuk payment (1 invoice, 2 pay service listrik, 3 pay service telpon)
  amount            DOUBLE PRECISION        NOT NULL,       -- Nominal transaksi
  status            INT                   NOT NULL, --0 success, 1 in progress, 2 timeout, 3 generic error, 4 cannot contact biller, 5 invalid dest account, 6 invalid from account, 7 insufficient balance
  created        TIMESTAMP             NOT NULL DEFAULT (now()),
  last_updated    TIMESTAMP             NOT NULL DEFAULT (now()),
  invoice           VARCHAR(40),
  from_wallet       BIGINT,
  to_wallet         BIGINT,
  merchant_id         BIGINT,
  notes             TEXT                 -- berisi catatan - catatan dari transaksi ini. Misal nomor debit card yang dipakai top up, kode voucher ketika isi pulsa dll
);

