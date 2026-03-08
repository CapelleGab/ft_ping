# ft_ping

> Implementation of the `ping` command in Rust using raw sockets and ICMP protocol.

## About

ft_ping is a network diagnostic tool that sends ICMP Echo Request packets to a target host and measures the round-trip time (RTT) of the responses. Built from scratch in Rust using low-level system calls via `libc`.

## Features

- **DNS Resolution** : Resolve hostnames to IPv4 addresses via `libc::getaddrinfo`
- **Raw ICMP Sockets** : Send and receive ICMP packets using `SOCK_RAW`
- **RTT Measurement** : Measure round-trip time for each packet
- **Statistics** : Packet loss percentage, min/avg/max RTT
- **Configurable** : TTL, packet count, interval between packets, verbose mode

## Architecture

```
ft_ping/
├── src/
│   ├── main.rs        # Entry point, argument parsing, ping loop
│   ├── dns.rs         # DNS resolution (hostname -> IPv4)
│   ├── socket.rs      # Raw socket creation, send/receive
│   ├── icmp.rs        # ICMP Echo Request packet construction
│   ├── checksum.rs    # ICMP checksum calculation (RFC 1071)
│   └── stats.rs       # Ping statistics collection and display
├── Cargo.toml
└── Makefile
```

## Getting Started

### Prerequisites

- Rust (edition 2024)
- Root privileges (required for raw sockets)

### Build

```bash
cargo build
```

### Usage

```bash
sudo ./target/debug/ft_ping [OPTIONS] <destination>
```

Or via Makefile:

```bash
make run ARGS="google.com"
make run ARGS="-c 5 -t 128 google.com"
```

### Options

| Flag | Long | Default | Description |
|------|------|---------|-------------|
| `-v` | `--verbose` | `false` | Verbose output |
| `-c` | `--count` | infinite | Stop after N packets |
| `-i` | `--interval` | `1.0` | Seconds between packets |
| `-t` | `--ttl` | `64` | IP Time to Live |
| `-h` | `--help` | - | Show help |
| `-V` | `--version` | - | Show version |

### Example Output

```
PING google.com (142.251.209.142) 56(84) bytes of data.
64 bytes from 142.251.209.142: icmp_seq=0 ttl=64 time=12.3 ms
64 bytes from 142.251.209.142: icmp_seq=1 ttl=64 time=11.8 ms
64 bytes from 142.251.209.142: icmp_seq=2 ttl=64 time=13.1 ms

--- google.com ping statistics ---
3 packets transmitted, 3 received, 0% packet loss
rtt min/avg/max = 11.800/12.400/13.100 ms
```

## Dependencies

| Crate | Version | Usage |
|-------|---------|-------|
| `libc` | 1.0.0-alpha.3 | System calls (socket, sendto, recvfrom, getaddrinfo) |
| `ctrlc` | 3.5.2 | SIGINT handling |
| `clap` | 4.5.60 | Command-line argument parsing |

## Documentation

[Documentation complete dans `docs/`](./docs/README.md)

| Section | Description |
|---------|-------------|
| [Architecture](./docs/architecture.md) | Architecture globale du projet |
| [DNS](./docs/modules/dns.md) | Resolution DNS |
| [Socket](./docs/modules/socket.md) | Gestion des raw sockets |
| [ICMP](./docs/modules/icmp.md) | Construction des paquets ICMP |
| [Checksum](./docs/modules/checksum.md) | Calcul du checksum |
| [Stats](./docs/modules/stats.md) | Collecte des statistiques |

## Makefile

| Commande | Description |
|----------|-------------|
| `make run ARGS="..."` | Build + execute avec sudo |
| `make clean` | Supprime les artefacts de build |
