# Documentation ft_ping

## Table des Matieres

### Architecture
| Document | Description |
|----------|-------------|
| [Architecture](./architecture.md) | Vue d'ensemble de l'architecture et flux de donnees |

### Modules

| Module | Fichier | Description | Docs |
|--------|---------|-------------|------|
| **main** | `src/main.rs` | Point d'entree, parsing args, boucle de ping | [Details](./modules/main.md) |
| **dns** | `src/dns.rs` | Resolution DNS via `libc::getaddrinfo` | [Details](./modules/dns.md) |
| **socket** | `src/socket.rs` | Creation et utilisation des raw sockets | [Details](./modules/socket.md) |
| **icmp** | `src/icmp.rs` | Construction des paquets ICMP Echo Request | [Details](./modules/icmp.md) |
| **checksum** | `src/checksum.rs` | Calcul du checksum ICMP (RFC 1071) | [Details](./modules/checksum.md) |
| **stats** | `src/stats.rs` | Collecte et affichage des statistiques | [Details](./modules/stats.md) |

### Guides
| Guide | Description |
|-------|-------------|
| [Troubleshooting](./guides/troubleshooting.md) | Resolution de problemes courants |

## Carte du Projet

```
User Input (hostname + options)
        │
        ▼
   ┌─────────┐
   │  main   │  clap::Parser
   └────┬────┘
        │
        ▼
   ┌─────────┐
   │   dns   │  libc::getaddrinfo
   └────┬────┘
        │ Ipv4Addr
        ▼
   ┌─────────┐
   │ socket  │  libc::socket (SOCK_RAW, IPPROTO_ICMP)
   └────┬────┘
        │ fd
        ▼
   ┌──────────────────────┐
   │  Ping Loop           │
   │  ┌──────┐  ┌──────┐  │
   │  │ icmp │→│socket │  │  build packet → sendto
   │  └──────┘  │      │  │
   │  ┌────────┐│      │  │
   │  │checksum││      │  │  compute checksum
   │  └────────┘│      │  │
   │            │      │←─│  recvfrom
   │  ┌──────┐  └──────┘  │
   │  │stats │             │  collect RTT
   │  └──────┘             │
   └──────────────────────┘
        │
        ▼
   Stats Summary
```
