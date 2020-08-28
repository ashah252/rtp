use super::tcp_session::{TcpSession, TcpState};

const MTU_SIZE: usize = 1400;

#[derive(PartialEq)]
#[repr(C)]
enum PacketType{
    Syn,
    SynAck,
    Ack,
    Fin,
    FinWait
}


#[derive(PartialEq)]
#[repr(C)]
struct TcpPacketHeader {
    packet_type: PacketType,
    seq: u32,
    ack: u32,
    id: u32,
    window_size: usize
}


impl TcpPacketHeader {
    fn matches(&self, sessionOfPacket: &TcpSession) -> bool {
        self.ack == sessionOfPacket.ack_seq && 
        match sessionOfPacket.state {
            TcpState::Closed => false,
            TcpState::Listen => self.packet_type == PacketType::Syn,
            TcpState::SynSent => self.packet_type == PacketType::SynAck,
            TcpState::SynReceived => self.packet_type == PacketType::Ack,
            TcpState::Established => {
                (self.packet_type == PacketType::Ack) ||
                (self.packet_type == PacketType::FinWait)
            },
            TcpState::FinWait1 => (self.packet_type == PacketType::FinWait),
            TcpState::FinWait2 => (self.packet_type == PacketType::Fin),
            _ => false
        }
    }
}

fn recv_from<P: FnOnce(usize, std::net::SocketAddr, &mut [u8]) -> Result<u32, ()>> (session: &TcpSession, predicate: P) -> Result<u32, ()> {
    let buf = &mut [0u8;super::mtu_size_bytes];
    match session.sock.recv_from(buf) {
        Ok((num_recved, endpoint_addr)) => {
            predicate(num_recved, endpoint_addr, buf)
        },
        Err(_) => Err(())
    }
}

fn recv_hdr_from(session: &TcpSession) -> Result<u32, ()> {
    recv_from(
        session,

        |_, _, buf| {
            let (packet_hdr, _) = parse_packet(buf);
            let hdr: &TcpPacketHeader = packet_hdr.expect("Invalid Header");

                if hdr.matches(session) {
                    Ok(hdr.ack) // next outgoing seq

                } else {
                    Err(())
                }
        }
    )
}

pub fn send_syn(session: &TcpSession) {
    session.sock.send_to(create_packet(PacketType::Syn, 1, 0, 0, 0, None).as_slice(), session.other.expect("Couldn't Send Syn Packet, No SocketAddr Provided"));
}

pub fn recv_syn(session: &TcpSession) -> Result<u32, ()> {
    recv_hdr_from(session)
}

pub fn send_syn_ack(session: &TcpSession) {
    session.sock.send_to(create_packet(PacketType::Syn, 1, 0, 0, 0, None).as_slice(), session.other.expect("Couldn't Send Syn Packet, No SocketAddr Provided"));
}

pub fn recv_syn_ack(session: &TcpSession) -> Result<u32, ()> {
    recv_hdr_from(session)
}

fn create_packet(packet_t: PacketType, outgoing: u32, incoming: u32, ident: u32, win_size: usize, load: Option<&[u8]>) -> Vec<u8> {
    
    let packet_header: TcpPacketHeader = TcpPacketHeader {
        packet_type: packet_t,
        seq: outgoing,
        ack: incoming,
        id: ident,
        window_size: win_size
    };

    let mut buf: Vec<u8> = Vec::new();

    unsafe {
        let hdr_serialized: &[u8; std::mem::size_of::<TcpPacketHeader>()] = std::mem::transmute(&packet_header);
        buf.extend_from_slice(hdr_serialized);
    }

    match load {
        Some(pload) => {
            buf.extend_from_slice(pload);
        },
        None => {
            // do nothing
        }
    };

    buf
}

fn parse_packet(buf: &mut [u8]) -> (Option<&TcpPacketHeader>, Option<&[u8]>) {
    const packet_hdr_size: usize = std::mem::size_of::<TcpPacketHeader>();
    let buf_size: usize = buf.len();
    if buf_size >= packet_hdr_size {
        let packet_hdr: &TcpPacketHeader;

        unsafe {
            packet_hdr = & *(buf as *mut _ as *mut TcpPacketHeader);
        }

        if buf_size > packet_hdr_size {
            let payload: &[u8] = &buf[packet_hdr_size..];
            (Some(packet_hdr), Some(payload))
            
        } else {
            (Some(packet_hdr), None)

        }
    } else {
        (None, None)
    }
}