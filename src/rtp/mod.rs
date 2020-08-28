use rand::thread_rng;

pub mod tcp_session;
pub mod tcp_packet;

const initial_timeout: f64 = 20.0;
const max_retries: usize = 3;
const mtu_size_bytes: usize = 1350;