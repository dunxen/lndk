use futures::executor::block_on;
use std::error::Error;
use std::fmt;
use tonic_lnd::{Client, ConnectError};

#[tokio::main]
async fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(args) => panic!("Bad arguments: {}", args),
    };

    let mut client = get_lnd_client(args).expect("failed to connect");

    let mut peer_stream = match block_on(
        client
            .lightning()
            .subscribe_peer_events(tonic_lnd::lnrpc::PeerEventSubscription {}),
    ) {
        Ok(t) => t.into_inner(),
        Err(e) => panic!("Could not get peer stream: {}", e),
    };

    while let Some(update) = peer_stream.message().await.expect("no peer events") {
        println!("Peer event: {:?}", update);
    }
}

fn get_lnd_client(cfg: LndCfg) -> Result<Client, ConnectError> {
    block_on(tonic_lnd::connect(cfg.address, cfg.cert, cfg.macaroon))
}

#[derive(Debug)]
enum ArgsError {
    NoArgs,
    AddressRequired,
    CertRequired,
    MacaroonRequired,
}

impl Error for ArgsError {}

impl fmt::Display for ArgsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArgsError::NoArgs => write!(f, "No command line arguments provided."),
            ArgsError::AddressRequired => write!(f, "LND's RPC server address is required."),
            ArgsError::CertRequired => write!(f, "Path to LND's tls certificate is required."),
            ArgsError::MacaroonRequired => write!(f, "Path to LND's macaroon is required."),
        }
    }
}

struct LndCfg {
    address: String,
    cert: String,
    macaroon: String,
}

impl LndCfg {
    fn new(address: String, cert: String, macaroon: String) -> LndCfg {
        LndCfg {
            address: address,
            cert: cert,
            macaroon: macaroon,
        }
    }
}

fn parse_args() -> Result<LndCfg, ArgsError> {
    let mut args = std::env::args_os();
    match args.next() {
        None => return Err(ArgsError::NoArgs),
        _ => {}
    };

    let address = match args.next() {
        Some(arg) => arg.into_string().expect("address is not UTF-8"),
        None => return Err(ArgsError::AddressRequired),
    };

    let cert_file = match args.next() {
        Some(arg) => arg.into_string().expect("cert is not UTF-8"),
        None => return Err(ArgsError::CertRequired),
    };

    let macaroon_file = match args.next() {
        Some(arg) => arg.into_string().expect("macaroon is not UTF-8"),
        None => return Err(ArgsError::MacaroonRequired),
    };

    Ok(LndCfg::new(address, cert_file, macaroon_file))
}
