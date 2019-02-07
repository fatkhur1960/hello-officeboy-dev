FORMAT: 1A
HOST: http://localhost:8081/

# APF rest API documentation

Dokumentasi rest API publik untuk sistem payment Ansvia.

Dokumen ini berisi spesifikasi setiap endpoint untuk beroperasi dengan akun. 
Beberapa operasi berupa CRUD operation seperti mendaftarkan akun baru, mengaktifkan, dan menonaktifkan.

## Group Account

### Mendapatkan Daftar Akun [GET /api/payment/v1/accounts]

Mendapatkan daftar akun yang telah teregister dan teraktivasi di dalam sistem payment.

+ Response 200 (application/json)

        {
            "count": 1,
            "entries": [
                {
                "id":123,
                "full_name": "Robin",
                "email": "robin@email.com",
                "phone_num": "+62123123"
                }
            ]
        }

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

### Hapus Akun [POST /api/payment/v1/account/delete]

Menghapus akun berdasarkan ID.

::: warning
#### <i class="fa fa-warning"></i> Perhatian
Operasi ini tidak dapat dikembalikan.
:::

+ Requset JSON (application/json)

        { "body": {"account_id": 1}, "signature": "ad5e669ef12339eddad5e669ef12339ead5e669ef12339eddad5e669ef12339e" }

+ Response 200 (application/json)

        {
            "code": 0,
            "status": "success",
            "description": ""
        }

