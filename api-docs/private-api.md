FORMAT: 1A

# APF rest API documentation

Dokumentasi rest API publik untuk sistem payment Ansvia.

## Group Authorization

### Remove Access Token [POST /auth/v1/remove_access_token]

Menghapus akses token

+ Response 200 (application/json)

        {}

## Group Transactions

### Account Count [GET /payment/v1/account/count]

Mendapatkan jumlah akun secara keseluruhan.

+ Response 200 (application/json)

        {}

### Account Info [GET /payment/v1/account/info]

Mendapatkan data akun.

+ Response 200 (application/json)

        {}

### Credit [POST /payment/v1/credit]

Rest API endpoint for topup
Mengembalikan jumlah balance akun setelah dikredit.

+ Response 200 (application/json)

        {}

### Debit [POST /payment/v1/debit]

Rest API endpoint untuk debit

+ Response 200 (application/json)

        {}

### List Account [GET /payment/v1/accounts]

Listing account

+ Response 200 (application/json)

        {}

### Search Accounts [GET /payment/v1/account/search]

Mencari akun berdasarkan kata kunci.

+ Response 200 (application/json)

        {}

### Transactions [GET /payment/v1/transactions]



+ Response 200 (application/json)

        {}

