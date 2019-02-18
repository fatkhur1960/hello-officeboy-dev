FORMAT: 1A
HOST: http://localhost:8080/

# APF rest API documentation

Dokumentasi rest API publik untuk sistem payment Ansvia.

Dokumen ini berisi spesifikasi setiap endpoint untuk beroperasi dengan akun. 
Beberapa operasi berupa CRUD operation seperti mendaftarkan akun baru, mengaktifkan, dan menonaktifkan.

## Group Authorization

### Melakukan Otorisasi [POST /api/auth/v1/authorize]

Biasanya digunakan untuk login, mengembalikan akses token.

+ Request JSON message (application/json)

        {
            "email": "agus@mail.com",
            "phone": "+62857180212",
            "passhash": "95f24de5d8f717263a07fe4dafacb6d9bc6c0163ffc162800fb026bc46c8c59c"
        }

+ Response 200 (application/json)

{
    "code": 0,
    "status": "success",
    "result": {
        
    }
}

## Group Account

### Register New Account [POST /api/payment/v1/account/register]

Meregister akun baru. Perlu diketahui bahwa akun yang telah teregister tidak
serta merta aktif, perlu mengaktifkan menggunakan endpoint `/account/activate`.

+ Response 200 (application/json)

        {
            "code": 0,
            "status": "success",
            "description": ""
        }

### Activate New Account [POST /api/payment/v1/account/activate]

Mengaktifkan akun yang telah teregister.

+ Request JSON message (application/json)

        {
            "body": {
                "account_id": 1,
                "initial_balance": 1000.0
            },
            "signature": "ad5e669ef12339eddad5e669ef12339ead5e669ef12339eddad5e669ef12339e"
        }

+ Response 200 (application/json)

        {
            "code": 0,
            "status": "success",
            "result": {
                "id": 1,
                "full_name": "Agus Pramono",
                "balance": 10,
                "email": "agus@pramono.com",
                "phone_num": "+628576393485",
                "active": true,
                "register_time": "2019-02-01T17:55:32.378872"
            }
        }

## Group Transactions

### Mendapatkan Daftar Transaksi [GET /api/payment/v1/transactions]

Mendapatkan daftar transaksi.

+ Response 200 (application/json)

        {
            "code":0,
            "status":"success",
            "result":{
                "count":2,
                "entries":[
                    {
                        "id":123,
                        "kind":"transfer",
                        "credit":1000.0,
                        "debit":0,
                        "timestamp":838439430423
                    },
                    {
                        "id":456,
                        "kind":"topup",
                        "credit":5000.0,
                        "debit":0,
                        "timestamp":838439430423
                    }
                ]
            }
        }
