pub mod consumer_mod;
pub mod producer_mod;
pub mod kafka {

    use rdkafka::util::get_rdkafka_version;

    pub fn log_version_info() {
        let (version_n, version_s) = get_rdkafka_version();
        info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);
    }
}