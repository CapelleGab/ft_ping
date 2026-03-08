# checksum.rs

> Calcul du checksum ICMP selon la RFC 1071.

## Informations

| Propriete | Valeur |
|-----------|--------|
| **Fichier** | `src/checksum.rs` |
| **Exporte** | `compute(data: &[u8]) -> u16` |
| **Dependances** | aucune |

## Role

Calcule le checksum Internet (RFC 1071) utilise dans les paquets ICMP. L'algorithme est le complement a 1 de la somme des mots de 16 bits.

## API

### `compute(data: &[u8]) -> u16`

| Param | Type | Description |
|-------|------|-------------|
| `data` | `&[u8]` | Donnees sur lesquelles calculer le checksum |

**Retour :** checksum sur 16 bits.

## Algorithme

1. **Somme** tous les mots de 16 bits (big-endian) dans un accumulateur 32 bits
2. **Byte impair** : si la longueur est impaire, ajoute le dernier byte decale de 8 bits
3. **Repli** : tant que les bits hauts (>16) ne sont pas zero, ajoute le carry au resultat
4. **Complement** : inverse tous les bits (complement a 1)

## Code

```rust
pub fn compute(data: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    let mut i = 0;

    // Somme des mots de 16 bits
    while i + 1 < data.len() {
        sum += u16::from_be_bytes([data[i], data[i + 1]]) as u32;
        i += 2;
    }

    // Byte impair restant
    if i < data.len() {
        sum += (data[i] as u32) << 8;
    }

    // Repli du carry
    while sum >> 16 != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }

    // Complement a 1
    !(sum as u16)
}
```

## Pourquoi un Checksum ?

Le checksum ICMP permet au destinataire de verifier que le paquet n'a pas ete corrompu pendant le transport. Lors de la construction du paquet, le champ checksum est initialise a 0, puis rempli avec le resultat de `compute()` sur le paquet entier.
