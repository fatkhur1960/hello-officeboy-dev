FORMAT: 1A

# APF rest API documentation

Dokumentasi rest API privat untuk Mainframe.

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

+ Request JSON (application/json)

        {
            "body": {
                "timestamp": 1550663774000, 
                "account": 1, 
                "amount": 100
            }, 
            "signature": "2757cbf03fa4b89a685254e0daf8e8bb63e6197682bf2dd20b77e3163a18c322"
        }

+ Response 200 (application/json)

        {}

### Debit [POST /payment/v1/debit]

Rest API endpoint untuk debit

+ Response 200 (application/json)

        {}

### List Account [GET /payment/v1/accounts?{page,limit}]

Listing account

+ Parameters

    + page: 0 (number) - Mirip seperti offset.
    + limit: 10 (number) - Membatasi pengembalian daftar akun.

+ Response 200 (application/json)

        {
            "status": "success", 
            "code": 0, 
            "description": "", 
            "result": {
                "count": 8, 
                "entries": [
                    {
                        "phone_num": "+628123123", 
                        "register_time": "2019-02-18T14:24:12.512695", 
                        "id": 1, 
                        "full_name": "Zufar", 
                        "active": true, 
                        "balance": 0, 
                        "email": "zufar@mail.com"
                    }, 
                    {
                        "phone_num": "+628123124", 
                        "register_time": "2019-02-18T14:24:12.524275", 
                        "id": 2, 
                        "full_name": "Akmal", 
                        "active": true, 
                        "balance": 0, 
                        "email": "akmal@mail.com"
                    }, 
                    {
                        "phone_num": "+628123125", 
                        "register_time": "2019-02-18T14:24:12.533736", 
                        "id": 3, 
                        "full_name": "Anto", 
                        "active": true, 
                        "balance": 0, 
                        "email": "anto@mail.com"
                    }, 
                    {
                        "phone_num": "+628123126", 
                        "register_time": "2019-02-18T14:24:12.542059", 
                        "id": 4, 
                        "full_name": "Hanky", 
                        "active": true, 
                        "balance": 0, 
                        "email": "hanky@mail.com"
                    }, 
                    {
                        "phone_num": "+628123127", 
                        "register_time": "2019-02-18T14:24:12.550564", 
                        "id": 5, 
                        "full_name": "Andrie", 
                        "active": true, 
                        "balance": 0, 
                        "email": "andrie@mail.com"
                    }, 
                    {
                        "phone_num": "+628123128", 
                        "register_time": "2019-02-18T14:24:12.559816", 
                        "id": 6, 
                        "full_name": "Ubai", 
                        "active": true, 
                        "balance": 0, 
                        "email": "ubai@mail.com"
                    }, 
                    {
                        "phone_num": "+6285828382211", 
                        "register_time": "2019-02-20T11:00:06.258401", 
                        "id": 7, 
                        "full_name": "Alice", 
                        "active": true, 
                        "balance": 0, 
                        "email": "alice@email.com"
                    }
                ]
            }
        }

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

