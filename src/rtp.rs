use std::net;

pub enum TcpState{
    Closed,
    Listen,
    SynSent,
    SynRecv,
    Established,
    FinWait1,
    FinWait2
}

pub enum HostType{
    Sender,
    Receiver
}

pub struct TcpSession{
    host: HostType,
    sock: net::UdpSocket,
    state: TcpState
}

impl TcpSession{
    pub fn new(isSender: bool, port: u16) -> Self {
        match isSender{
            true => {
                TcpSession {
                            host: HostType::Receiver,
                            sock: net::UdpSocket::bind(format!("0.0.0.0:{}", port)).expect("Couldn't Create UDP Socket..."),
                            state: TcpState::Listen
                }
            },
            
            false => {
                TcpSession {
                    host: HostType::Sender,
                    sock: net::UdpSocket::bind("0.0.0.0:0").expect("Couldn't Create UDP Socket..."),
                    state: TcpState::Closed
                }
            }
        }
        
    }
    pub fn runRoutine(&self, end_host: net::SocketAddrV4) -> std::io::Result<TcpState> {
        loop{
            match &self.host {
                HostType::Sender => {
                    match &self.state {
                        Closed => {},
                        SynSent => {},
                        Established => {},
                        FinWait1 => {},
                        FinWait2 => {},
                        _ => {}
                    }
                },
                HostType::Receiver => {
                    match &self.state {
                        Listen => {},
                        SynRecv => {},
                        Established => {},
                        FinWait1 => {},
                        FinWait2 => {},
                        _ => {}
                    }
                }
            }
        }
    }

}