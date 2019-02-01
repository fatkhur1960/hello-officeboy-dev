FORMAT: 1A

# Ansvia Payment Framework API

Dokumentasi rest API publik untuk sistem payment Ansvia.

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
            "id": 1,
            "full_name": "Agus Pramono",
            "balance": 10.0,
            "email": "agus@pramono.com",
            "phone_num": "+628576393485",
            "active": true,
            "register_time": "2019-02-01T17:55:32.378872"
        }

