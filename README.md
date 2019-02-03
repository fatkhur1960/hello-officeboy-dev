Ansvia Payment Framework
============================

Merupakan framework untuk membuat aplikasi _Payment_, dibuat dan ditulis menggunakan [Rust](https://rust-lang.org).

Kebutuhan
----------------

Berikut kebutuhan pokok untuk bisa mem-build library:

1. [Rust](https://www.rust-lang.org/)
2. PostgreSQL >= 9.x
3. [diesel](http://diesel.rs)
4. [Aglio](https://www.npmjs.com/package/aglio) (optional, untuk dokumentasi)


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
* Memastikan kodenya telah layak dan lolos unittest dengan cara menjalankan perintah: `make test`.
* Menggunakan tata bahasa yang mudah dipahami dan menjelaskan perubahan mendasar pada commit message-nya.


----
Apabila ada yang perlu ditanyakan contact: r@ansvia.com
