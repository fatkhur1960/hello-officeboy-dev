Ansvia Payment Framework
============================

[![pipeline status](https://gitlab.com/anvie/payment/badges/master/pipeline.svg)](https://gitlab.com/anvie/payment/commits/master)

Merupakan framework untuk membuat aplikasi _Payment_, dibuat dan ditulis menggunakan [Rust](https://rust-lang.org).

Kebutuhan
----------------

Daftar kebutuhan berikut dibutuhkan apabila kita ingin melakukan build di mesin environment lokal, kamu bisa juga melakukan build menggunakan Docker sehingga tidak perlu menginstall satu-per-satu kebutuhan ini. Untuk build menggunakan Docker lihat bagian *Build menngunakan Docker*.
Berikut kebutuhan pokok untuk bisa melakukan build di mesin lokal:

1. [Rust](https://www.rust-lang.org/)
2. PostgreSQL >= 9.x
3. [diesel](http://diesel.rs)
4. [Aglio](https://www.npmjs.com/package/aglio) (optional, untuk dokumentasi)
5. [Rustfmt](https://github.com/rust-lang/rustfmt)
5. [Cargo clippy](https://github.com/rust-lang/rust-clippy)
6. [Cargo audit](https://github.com/RustSec/cargo-audit)
7. [Protocol Buffer](https://developers.google.com/protocol-buffers/)

Build
-----------

Sebelum melakukan build pastikan dulu Libpq (Library-nya PostgreSQL) telah tersedia, di Ubuntu bisa menggunakan perintah `apt install libpq-dev` atau di Debian `apt install libpq-devel`, di OSX bisa menggunakan perintah: `brew install libpq`.

Setelah semua dependensi tersedia, ketikkan:

    $ cargo build

Build menggunakan Docker
----------------------------

Cara paling mudah untuk melakukan build adalah menggunakan Docker:

    $ docker run -it --rm -v $(pwd):/workdir \
        -v /tmp:/root/.cargo/git \
        -v /tmp:/root/.cargo/registry \
        anvie/rust-musl-build:latest \
        cargo build --release --target=x86_64-unknown-linux-musl

Docker image `anvie/rust-musl-build` adalah container berbasis Linux dan sudah berisi semua kebutuhan development untuk build project ini, setelah build selesai
output bisa didapatkan di `target/x86_64-unknown-linux-musl`.

Kamu bisa juga menjalankan perintah tersebut menggunakan make:

    $ make release-linux

Testing
----------

Testing kebanyakan ditulis terintegrasi (integration testing), untuk itu perlu menjalankan database
dan mempersiapkan environment-nya, ini hanya perlu dijalankan sekali, ketikkan:

    $ make test-env

**CATATAN**: Perintah `test-env` akan membuat database baru dengan nama `apf_test` dimana database ini akan digunakan
sebagai storage ketika proses testing terjadi.

Untuk melakukan test ketikkan:

    $ make test

Frontend
------------

Stack frontend kita menggunakan React, base ada di direktori `/frontends`.

Apabila ingin mencoba menjalankannya bisa check frontend untuk admin:

    $ cd frontends/admin
    $ npm install
    $ npm start

Buka http://localhost:3000/


Dokumentasi
-------------

Dokumentasi dibagikan menjadi beberapa bagian:

1. Dokumentasi pustaka (library).
2. Dokumentasi Rest API.

Untuk menggenerasikan dokumentasi pustaka cukup ketikkan:

    $ make lib-docs

Untuk menggenerasikan dokumentasi rest API:

    $ make api-docs

**CATATAN**: Penggenerasian dokumentasi untuk rest API membutuhkan tool [Aglio](https://www.npmjs.com/package/aglio).


Konvensi
------------

Setiap perubahan pada project ini harus mengikuti konvensi ini.

Sebelum melakukan commit harus:

* Memastikan kodenya telah diformat menggunakan perintah: `make fmt`.
* Memastikan kodenya telah layak sesuai standar dengan cara menjalankan perintah: `make lint`.
* Memastikan kodenya telah lolos unittest dengan cara menjalankan perintah: `make test`.
* Memastikan kodenya telah aman dari dependensi yang bermasalah dengan menjalankan perintah: `make audit`.
* Menggunakan tata bahasa yang mudah dipahami dan menjelaskan perubahan mendasar pada commit message-nya.

----
Apabila ada yang perlu ditanyakan contact: r@ansvia.com
