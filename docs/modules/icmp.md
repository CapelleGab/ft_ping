# icmp.rs

> Construction des paquets ICMP Echo Request.

## Informations

| Propriete | Valeur |
|-----------|--------|
| **Fichier** | `src/icmp.rs` |
| **Exporte** | `build_echo_request`, `ICMP_PACKET_SIZE` |
| **Dependances internes** | `checksum` |

## Role

Construit un paquet ICMP Echo Request (type 8) de 64 bytes pret a etre envoye via un raw socket.

## Constantes

| Constante | Valeur | Description |
|-----------|--------|-------------|
| `ICMP_ECHO_REQUEST` | `8` | Type ICMP Echo Request |
| `ICMP_HEADER_SIZE` | `8` | Taille du header ICMP en bytes |
| `ICMP_PAYLOAD_SIZE` | `56` | Taille du payload en bytes |
| `ICMP_PACKET_SIZE` | `64` | Taille totale du paquet (8 + 56) |

## API

### `build_echo_request(seq: u16, id: u16) -> [u8; 64]`

| Param | Type | Description |
|-------|------|-------------|
| `seq` | `u16` | Numero de sequence (incrementant) |
| `id` | `u16` | Identifiant (PID du processus) |

**Retour :** paquet ICMP de 64 bytes avec checksum calcule.

## Structure du Paquet

```
Offset  Taille  Champ
0       1       Type (8 = Echo Request)
1       1       Code (0)
2       2       Checksum
4       2       Identifier
6       2       Sequence Number
8       56      Payload (zeros)
```

## Fonctionnement

1. Cree un buffer de 64 bytes initialise a zero
2. Ecrit le type (8), code (0), identifier et sequence number
3. Calcule le checksum sur le paquet entier via `checksum::compute`
4. Ecrit le checksum aux offsets 2-3
5. Retourne le paquet pret a envoyer

## Code

```rust
pub fn build_echo_request(seq: u16, id: u16) -> [u8; ICMP_PACKET_SIZE] {
    let mut packet = [0u8; ICMP_PACKET_SIZE];

    packet[0] = ICMP_ECHO_REQUEST;  // Type: 8
    packet[1] = 0;                   // Code: 0
    packet[4] = (id >> 8) as u8;     // Identifier (big-endian)
    packet[5] = id as u8;
    packet[6] = (seq >> 8) as u8;    // Sequence (big-endian)
    packet[7] = seq as u8;

    let cksum = checksum::compute(&packet);
    packet[2] = (cksum >> 8) as u8;
    packet[3] = cksum as u8;

    packet
}
```
