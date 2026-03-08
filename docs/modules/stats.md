# stats.rs

> Collecte et affichage des statistiques de ping.

## Informations

| Propriete | Valeur |
|-----------|--------|
| **Fichier** | `src/stats.rs` |
| **Exporte** | `Stats` (struct) |
| **Dependances** | aucune |

## Role

Collecte les donnees de chaque ping (envoyes, recus, RTT) et affiche un resume final identique a celui du vrai `ping`.

## Structure

```rust
pub struct Stats {
    pub sent: u64,       // Paquets envoyes
    pub received: u64,   // Paquets recus
    pub rtts: Vec<f64>,  // RTT de chaque reponse (ms)
}
```

## API

### `Stats::new() -> Self`

Cree une instance avec compteurs a zero et vecteur RTT vide.

### `add_sent(&mut self)`

Incremente le compteur de paquets envoyes.

### `add_received(&mut self, rtt: f64)`

Incremente le compteur de paquets recus et enregistre le RTT.

### `loss_percent(&self) -> f64`

Calcule le pourcentage de perte : `(sent - received) / sent * 100`.

### `min(&self) -> f64`

Retourne le RTT minimum.

### `max(&self) -> f64`

Retourne le RTT maximum.

### `avg(&self) -> f64`

Retourne le RTT moyen.

### `print_summary(&self, destination: &str)`

Affiche le resume final :

```
--- google.com ping statistics ---
3 packets transmitted, 3 received, 0% packet loss
rtt min/avg/max = 11.800/12.400/13.100 ms
```

## Utilisation dans main

```rust
let mut ping_stats = stats::Stats::new();

// Dans la boucle :
ping_stats.add_sent();
if bytes > 0 {
    ping_stats.add_received(rtt);
}

// A la fin :
ping_stats.print_summary(&args.destination);
```
