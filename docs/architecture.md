# Architecture ft_ping

## Vue d'Ensemble

ft_ping suit une architecture modulaire simple avec 6 modules, chacun responsable d'une partie du processus de ping.

## Flux d'Execution

```
1. Parse arguments (clap)
2. Resolve DNS (dns::resolve)
3. Create raw socket (socket::create)
4. Loop:
   a. Build ICMP Echo Request (icmp::build_echo_request)
      └── Compute checksum (checksum::compute)
   b. Send packet (socket::send)
   c. Record sent (stats::add_sent)
   d. Receive reply (socket::receive)
   e. Measure RTT
   f. Record received (stats::add_received)
   g. Print result
   h. Sleep interval
   i. Break if count reached
5. Print statistics (stats::print_summary)
6. Close socket (socket::close)
```

## Modules et Responsabilites

| Module | Responsabilite | Dependances externes |
|--------|---------------|---------------------|
| `main` | Orchestration | `clap` |
| `dns` | Hostname → IPv4 | `libc::getaddrinfo` |
| `socket` | Raw socket ICMP | `libc::socket`, `sendto`, `recvfrom`, `setsockopt`, `close` |
| `icmp` | Construction paquets | `checksum` |
| `checksum` | Checksum RFC 1071 | aucune |
| `stats` | Statistiques RTT | aucune |

## Graphe de Dependances Internes

```
main
 ├── dns
 ├── socket
 ├── icmp
 │    └── checksum
 └── stats
```

## Format du Paquet ICMP

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Type (8)  |    Code (0)   |         Checksum              |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|         Identifier            |       Sequence Number         |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                         Payload (56 bytes)                    |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

Total: 64 bytes (8 header + 56 payload)
```

## Appels Systeme Utilises

| Appel | Module | Usage |
|-------|--------|-------|
| `getaddrinfo` | dns | Resolution DNS IPv4 |
| `socket` | socket | Creation socket `AF_INET, SOCK_RAW, IPPROTO_ICMP` |
| `setsockopt` | socket | Configuration TTL (`IP_TTL`) |
| `sendto` | socket | Envoi paquet ICMP |
| `recvfrom` | socket | Reception reponse ICMP |
| `close` | socket | Fermeture socket |
