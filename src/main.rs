use clap::Parser;
use local_ip_address::local_ip;
use qr2term::print_qr;

#[derive(Parser, Debug)]
struct Args {
   #[arg(short, long)]
    port: u16,
}


fn main() {
    let args = Args::parse();
    let ip = local_ip().unwrap().to_string();
    let address = format!("http://{}:{}", ip, args.port);
    println!("Listening on {}\n", address);

    print_qr(address).unwrap();
}
