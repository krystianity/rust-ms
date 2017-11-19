# rust-microservice skeleton

:fishing_pole_and_fish: Secure High Performance Microservices

## Ships with

* Logger
* JSON Config Loader
* HTTP Server `using iron/hyper`
* Redis Client
* HTTP Client `using reqwest/hyper`
* ORM (MySQL) Client `using diesel`
* Kafka Producer/Consumer `using rdkafka`

## Setup Instructions

* `git clone https://github.com/krystianity/rust-ms.git`
* `cd rust-ms`
* `cargo build`
* `RUST_LOG=rust_ms cargo run`