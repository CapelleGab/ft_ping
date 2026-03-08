mod icmp;
mod socket;
mod checksum;
mod stats;
mod dns;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "ft_ping", version, about = "Send ICMP ECHO_REQUEST to network hosts")]
struct Args {
    destination: String,
    #[arg(short = 'v', long)]
    verbose: bool,
    #[arg(short = 'c', long)]
    count: Option<u64>,
    #[arg(short = 'i', long, default_value_t = 1.0)]
    interval: f64,
    #[arg(short = 't', long, default_value_t = 64)]
    ttl: u8,
}

fn main() {
    let args = Args::parse();

    let ip = match dns::resolve(&args.destination) {
        Ok(ip) => ip,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    println!("PING {} ({}) 56(84) bytes of data.", args.destination, ip);

    let fd = socket::create(args.ttl);

    let id = std::process::id() as u16;
    let mut seq = 0u16;
    let mut ping_stats = stats::Stats::new();

    loop {
        let packet = icmp::build_echo_request(seq, id);

        let start = std::time::Instant::now();
        socket::send(fd, ip, &packet);
        ping_stats.add_sent();

        let mut recv_buf = [0u8; 1024];
        let bytes = socket::receive(fd, &mut recv_buf);
        let rtt = start.elapsed().as_secs_f64() * 1000.0;

        if bytes > 0 {
            ping_stats.add_received(rtt);
            println!("64 bytes from {}: icmp_seq={} ttl={} time={:.1} ms", ip, seq, args.ttl, rtt);
        }

        seq += 1;
        if let Some(count) = args.count {
            if seq as u64 >= count {
                break;
            }
        }

        std::thread::sleep(std::time::Duration::from_secs_f64(args.interval));
    }

    ping_stats.print_summary(&args.destination);
    socket::close(fd);
}