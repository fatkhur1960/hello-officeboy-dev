FORMAT: 1A
HOST: http://localhost:9090/

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
