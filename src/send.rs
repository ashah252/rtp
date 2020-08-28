use std::net::SocketAddr;
use std::net::IpAddr;

mod rtp;
use rtp::tcp_session::TcpSession;

fn main() {

    let meta_data = clap::App::new("RTP-Send")
                    .args_from_usage(
                        "<IP> 'The Ip Address of End Host'
                        <PORT> 'The Port of End Host'"
                    );
    
    let arg_matches: clap::ArgMatches = meta_data.get_matches();
    let ip: IpAddr = arg_matches
                    .value_of("IP")
                    .expect("Value of IP Not Found After Parsing")
                    .parse()
                    .expect("Couldn't Parse IP String as IpAddr");
    let port: u16 = arg_matches
                    .value_of("PORT")
                    .expect("Value of PORT Not Found After Parsing")
                    .parse()
                    .expect("Couldn't Parse PORT String as u16");
    

    let mut sender: TcpSession = TcpSession::socket();
    sender.connect(SocketAddr::new(ip, port));
}