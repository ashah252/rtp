use std::net;
use rand::Rng;
use super::tcp_packet::*;

pub enum TcpState {
    Closed,
    Listen,
    SynSent,
    SynReceived,
    Established,
    FinWait1,
    FinWait2
}

#[repr(C)]
pub enum HostType {
    Sender,
    Receiver
}


pub struct TcpSession {
    pub state: TcpState,
    pub sock: net::UdpSocket,
    pub other: Option<net::SocketAddr>,
    pub timeout: f64,
    pub latency_mean: f64,
    pub latency_dev: f64,
    pub outgoing_seq: u32,
    pub ack_seq: u32,
}

impl TcpSession {

    fn new(h: HostType, udp_socket: net::UdpSocket) -> Self {
        
        let mut rand_handler: rand::rngs::ThreadRng = rand::thread_rng();

        let rand_num: u32 = rand_handler.gen();
        let session_obj = TcpSession {
            state: match h {
                HostType::Sender => TcpState::Closed,
                HostType::Receiver => TcpState::Listen
            },
            sock: udp_socket,
            other: None,
            timeout: super::initial_timeout,
            latency_mean: 0.0,
            latency_dev: 0.0,
            outgoing_seq: rand_num,
            ack_seq: rand_num
        };

        session_obj.sock.set_read_timeout(Some(std::time::Duration::from_secs_f64(session_obj.timeout)));
        session_obj
    }

    pub fn socket() -> Self {
        let udp_socket: net::UdpSocket = net::UdpSocket::bind("0.0.0.0:0").expect("Binding Socket Failure");
        TcpSession::new(HostType::Sender, udp_socket)
        
    }

    pub fn server_socket(port: u16) -> Self {
        let udp_socket: net::UdpSocket = net::UdpSocket::bind(format!("0.0.0.0:{}", port)).expect("Binding Socket Failure");
        TcpSession::new(HostType::Receiver, udp_socket)
    }

    pub fn connect(&mut self, connection_info: net::SocketAddr) -> std::io::Result<&Self> {
        self.other = Some(connection_info);

        for mut iteration in 0..super::max_retries {

            match self.state {
                TcpState::Closed => {
                    send_syn(self);
                    self.state = TcpState::SynSent;
                },
                TcpState::SynSent => {

                    match recv_syn_ack(self) {
                        Ok(outgoing_seq) => {
                            self.outgoing_seq = outgoing_seq;
                            self.state = TcpState::Established;
                        },
                        Err(_) => {
                            // if timeout, retry connect(), reset iterations
                            iteration = 0;
                            self.state = TcpState::Closed;
                        }
                    }
                },
                TcpState::Established => {
                    // syncronize heartbeat job to keep connection alive
                },
                _ => {
                    panic!("Shouldn't Get to Unreachable Tcp State in Connect")
                }
            }

        }

        self.state = TcpState::SynSent;
        Ok(self)
    }

    pub fn listen(&mut self) {
        loop {
            match self.state {
                TcpState::Closed => {
                    
                },
                TcpState::Listen => {

                },
                TcpState::SynSent => {
                    
                },
                TcpState::SynReceived => {
                    
                },
                TcpState::Established => {
                    
                },
                TcpState::FinWait1 => {
                    
                },
                TcpState::FinWait2 => {
                    
                }
            }
        }
    }
}