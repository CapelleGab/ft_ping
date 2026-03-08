# main.rs

> Point d'entree du programme, parsing des arguments et boucle de ping.

## Informations

| Propriete | Valeur |
|-----------|--------|
| **Fichier** | `src/main.rs` |
| **Dependances internes** | `dns`, `socket`, `icmp`, `stats` |
| **Dependances externes** | `clap` |

## Role

Orchestre l'ensemble du programme :
1. Parse les arguments CLI avec `clap`
2. Resout le hostname en IPv4
3. Cree le raw socket
4. Boucle d'envoi/reception de paquets ICMP
5. Affiche les statistiques finales

## Structure Args

```rust
#[derive(Parser, Debug)]
#[command(name = "ft_ping", version, about = "Send ICMP ECHO_REQUEST to network hosts")]
struct Args {
    destination: String,          // Hostname ou IP (positional)
    #[arg(short = 'v', long)]
    verbose: bool,                // Mode verbose
    #[arg(short = 'c', long)]
    count: Option<u64>,           // Nombre de paquets (None = infini)
    #[arg(short = 'i', long, default_value_t = 1.0)]
    interval: f64,                // Intervalle en secondes
    #[arg(short = 't', long, default_value_t = 64)]
    ttl: u8,                      // Time To Live
}
```

## Boucle de Ping

```rust
loop {
    let packet = icmp::build_echo_request(seq, id);  // Build ICMP packet
    socket::send(fd, ip, &packet);                    // Send
    ping_stats.add_sent();
    let bytes = socket::receive(fd, &mut recv_buf);   // Receive
    let rtt = start.elapsed().as_secs_f64() * 1000.0; // Measure RTT (ms)
    if bytes > 0 {
        ping_stats.add_received(rtt);
        // Print result
    }
    // Break if count reached
    // Sleep interval
}
```

## Identifiant de Session

L'identifiant ICMP est derive du PID du processus :

```rust
let id = std::process::id() as u16;
```

Cela permet de distinguer les reponses de notre ping de celles d'autres processus.
