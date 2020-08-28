mod rtp;
use rtp::tcp_session::TcpSession;

fn main() {

    let meta_data = clap::App::new("RTP-Recv")
                    .args_from_usage(
                        "<PORT> 'The Port of End Host'"
                    );
    
    let arg_matches: clap::ArgMatches = meta_data.get_matches();
    let port: u16 = arg_matches.value_of("PORT").expect("Value of PORT Not Found After Parsing").parse().expect("Couldn't Port as u16");
    
    let receiver: TcpSession = TcpSession::new_receiver(port);

}