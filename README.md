# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Subscriber model struct.`
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Subscriber repository.`
    -   [ ] Commit: `Implement list_all function in Subscriber repository.`
    -   [ ] Commit: `Implement delete function in Subscriber repository.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
1. In the Observer pattern diagram explained by the Head First Design Pattern book, Subscriber is defined as an interface. Explain based on your understanding of Observer design patterns, do we still need an interface (or trait in Rust) in this BambangShop case, or a single Model struct is enough?

Dalam kasus BambangShop, struct Subscriber saja sudah cukup. Alasannya karena:
- Observer pattern klasik memerlukan interface/trait untuk memastikan semua observer memiliki metode update yang konsisten. Namun, di sini notifikasi dilakukan melalui HTTP request ke URL subscriber, sehingga semua subscriber diperlakukan sama tanpa perlu variasi perilaku.
- Karena tidak ada perbedaan cara notifikasi (semua via URL), menggunakan struct langsung lebih simpel dan mengurangi kompleksitas kode.
- Jika nanti ada tipe subscriber berbeda seperti notifikasi via SMS, Email yang memerlukan implementasi logika berbeda, baru diperlukan trait untuk abstraksi.

2. id in Program and url in Subscriber is intended to be unique. Explain based on your understanding, is using Vec (list) sufficient or using DashMap (map/dictionary) like we currently use is necessary for this case?

DashMap lebih tepat karena:
- DashMap (hashmap) memungkinkan pencarian dan penghapusan berdasarkan key (ID/URL) dengan kompleksitas O(1), sedangkan Vec memerlukan iterasi O(n).
- DashMap sudah dirancang untuk akses konkuren (thread-safe), sementara Vec biasa tidak aman untuk operasi baca/tulis bersamaan.
- DashMap secara otomatis menjamin key unik, sementara dengan Vec perlu pengecekan manual yang rentan race condition.

3. When programming using Rust, we are enforced by rigorous compiler constraints to make a thread-safe program. In the case of the List of Subscribers (SUBSCRIBERS) static variable, we used the DashMap external library for thread safe HashMap. Explain based on your understanding of design patterns, do we still need DashMap or we can implement Singleton pattern instead?

DashMap lebih direkomendasikan karena beberapa alasan, yakni:
- DashMap menggunakan sharded locking yang meminimalkan kontensi antar thread, sementara Singleton dengan Mutex/RwLock bisa menjadi bottleneck karena mengunci seluruh struktur.
- DashMap menyediakan API thread-safe siap pakai seperti ada function insert dan get, sedangkan Singleton perlu implementasi manual locking yang rawan error.
- DashMap lebih optimal untuk skenario high concurrency karena tidak mengunci seluruh data, berbeda dengan Mutex yang mengunci akses ke seluruh map.

#### Reflection Publisher-2

1. In the Model-View Controller (MVC) compound pattern, there is no “Service” and “Repository”. Model in MVC covers both data storage and business logic. Explain based on your understanding of design principles, why we need to separate “Service” and “Repository” from a Model?

Dalam MVC tradisional, Model memang menggabungkan logika bisnis dan akses data. Namun, pemisahan Service dan Repository dilakukan karena prinsip desain berikut:
- **Single Reponsibility Principle (SRP)**, dimana Repository bertugas hanya untuk mengelola interaksi dengan penyimpanan data (CRUD). Service hanya mengimplementasikan logika bisnis seperti validasi, transformasi data, notifikasi. Contohnya, ProductRepository hanya menyimpan data produk, sementara ProductService mengatur aturan bisnis seperti mengubah product_type menjadi huruf besar.
- **Dependency Inversion Principle (DIP)**, dimana service bergantung pada abstraksi Repository (meski di Rust tidak menggunakan trait di sini), sehingga memudahkan perubahan storage seperti dari DashMap ke database tanpa mengganggu logika bisnis.
- **Reusability dan Testability**, dimana service bisa digunakan oleh banyak controller seperti API dan CLI, sementara Repository mudah di-mock untuk pengujian.

2. What happens if we only use the Model? Explain your imagination on how the interactions between each model (Program, Subscriber, Notification) affect the code complexity for each model?

Jika semua logika bisnis dan penyimpanan digabung dalam Model, dampaknya adalah:
- Tingginya keterikatan atau coupling karena perubahan pada penyimpanan data seperti mengganti DashMap dengan database akan memaksa modifikasi langsung di Model, yang berisiko merusak logika bisnis.
- Kompleksitas model, sebagai contoh Product harus mengurus storage (DashMap) dan validasi harga, Subscriber akan menangani HTTP request untuk notifikasi dan penyimpanan URL.
- Duplikasi kode, logika seperti "notifikasi ke subscriber" harus diulang di setiap Model (Product, Notification), alih-alih dipusatkan di Service.

3. Have you explored more about Postman? Tell us how this tool helps you to test your current work. You might want to also list which features in Postman you are interested in or feel like it is helpful to help your Group Project or any of your future software engineering projects.

Peran postman sangat membantu untuk:
- Menguji endpoint seperti POST /product atau POST /notification/subscribe tanpa menulis kode client.
- Memudahkan pengujian API tanpa perlu menulis kode client manual
- Menyediakan fitur Collections untuk mengorganisir berbagai endpoint terkait
- Memungkinkan penggunaan Environment Variables untuk pengelolaan konfigurasi
- Mendukung Automated Testing melalui Collection Runner dan Newman
- Membuat dokumentasi API otomatis dari koleksi yang ada
- Menyediakan Mock Server untuk pengembangan frontend sebelum backend siap
- Memfasilitasi kolaborasi tim melalui shared workspace
- Menawarkan Pre-request Scripts untuk otomatisasi tugas testing
- Memungkinkan pengujian berbagai skenario termasuk error handling
- Menyederhanakan verifikasi respons API dan format data

Dalam project ini, postman sangat berguna untuk:
- Menguji seluruh CRUD operations untuk Product
- Memverifikasi fungsi subscribe/unsubscribe Notification
- Mengecek error handling seperti saat data tidak ditemukan
- Memastikan konsistensi format respons API

#### Reflection Publisher-3

1. Observer Pattern has two variations: Push model (publisher pushes data to subscribers) and Pull model (subscribers pull data from publisher). In this tutorial case, which variation of Observer Pattern that we use?

Pada kode ini digunakan Push Model. Ini terlihat dari cara ProductService mengirim data lengkap ke subscriber via HTTP POST saat notifikasi. Contoh di service/product.rs:

`NotificationService.notify(&product.product_type, "CREATED", product_result.clone());`

Dan di model/subscriber.rs, subscriber langsung menerima payload lengkap:

`REQWEST_CLIENT.post(&self.url).body(to_string(&payload).unwrap())`

Publisher (Product) aktif mengirim semua data yang diperlukan ke subscriber tanpa perlu subscriber menarik data tambahan.

2. What are the advantages and disadvantages of using the other variation of Observer Pattern for this tutorial case? (example: if you answer Q1 with Push, then imagine if we used Pull)

Jika pakai Pull Model (subscriber ambil data sendiri dari publisher), kelebihannya adalah:
- Subscriber bisa kontrol data yang diambil (e.g., hanya butuh product_url saja)
- Mengurangi ukuran data yang dikirim saat notifikasi

Kekurangannya adalah:
- Subscriber harus tahu cara akses API Product (/product/<id>), menambah kompleksitas
- Beban server meningkat karena banyak request ke endpoint Product
- Rentan error jika Product sudah dihapus sebelum subscriber melakukan pull

3 Explain what will happen to the program if we decide to not use multi-threading in the notification process.

Jika thread::spawn di service/notification.rs dihapus:
`// Tanpa thread::spawn
subscriber.update(payload.clone());`

Maka, hal ini akan mengakibatkan:
- Blocking request, dimana setiap HTTP POST ke subscriber akan menunggu sampai selesai sebelum melanjutkan proses.
- Lambatnya response API, dimana waktu tunggu API create/publish/delete product akan bergantung pada subscriber paling lambat.
- Error berantai, karena jika satu subscriber down/lemot, seluruh proses notifikasi dan operasi product akan terhambat.
- Scalability buruk karena jumlah subscriber yang besar akan membuat sistem sulit menangani beban tinggi.