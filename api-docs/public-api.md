FORMAT: 1A

# APF rest API documentation

Dokumentasi rest API publik Mainframe.

## Group Account

### Activate Account [POST /payment/v1/account/activate]

Mengaktifkan user yang telah teregister.
Ini nantinya dijadikan link yang akan dikirimkan ke email pendaftar.

+ Request JSON (application/json)

        {
            "token": "8dfa985e73eed236d5ca211854783a7a01fbe5d3e7b59d002d205c3631a167ec", 
            "password": "mymagicpassword"
        }

+ Response 200 (application/json)

        {
            "status": "success", 
            "code": 0, 
            "description": "", 
            "result": {
                "register_time": "2019-02-20T11:00:06.258401", 
                "phone_num": "+6285828382211", 
                "id": 7, 
                "full_name": "Alice", 
                "email": "alice@email.com"
            }
        }

### Register Account [POST /payment/v1/account/register]

Rest API endpoint untuk mendaftarkan akun baru.
Setelah register akun tidak langsung aktif, perlu melakukan
aktifasi menggunakan endpoint `/account/activate`.

+ Request JSON (application/json)

        {
            "phone_num": "+6285828382211", 
            "email": "alice@email.com", 
            "full_name": "Alice"
        }

+ Response 200 (application/json)

        {
            "status": "success", 
            "code": 0, 
            "description": "", 
            "result": "8dfa985e73eed236d5ca211854783a7a01fbe5d3e7b59d002d205c3631a167ec"
        }

## Group Authorization

### Account Get Key [GET /auth/v1/get_key]

Mendapatkan keypair dari account.

+ Response 200 (application/json)

        {}

### Authorize [POST /auth/v1/authorize]

Meng-otorisasi akun yang telah teregister
User bisa melakukan otorisasi menggunakan email / nomor telp.

+ Response 200 (application/json)

        {}

## Group Transactions

### Balance [GET /payment/v1/balance]

Rest API endpoint untuk mendapatkan informasi balance pada akun.

+ Response 200 (application/json)

        {}

### Get Invoice [GET /payment/v1/invoice]

API endpoint untuk mendapatkan data invoice.

+ Response 200 (application/json)

        {}

### Info [GET /payment/v1/info]

Hanya digunakan untuk testing sahaja.

+ Response 200 (application/json)

        {}

### Me Info [GET /payment/v1/me/info]

Mendapatkan informasi current account.

+ Response 200 (application/json)

        {}

### Pay [POST /payment/v1/pay]

API endpoint untuk melakukan pembayaran.

+ Response 200 (application/json)

        {}

### Publish Invoice [POST /payment/v1/invoice/publish]

API endpoint untuk mem-publish invoice (membuat invoice baru).
Mengembalikan ID dari invoice-nya.

+ Response 200 (application/json)

        {}

### Transfer [POST /payment/v1/transfer]

Rest API endpoint untuk transfer

+ Response 200 (application/json)

        {}

