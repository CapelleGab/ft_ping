# socket.rs

> Gestion des raw sockets ICMP : creation, envoi, reception, fermeture.

## Informations

| Propriete | Valeur |
|-----------|--------|
| **Fichier** | `src/socket.rs` |
| **Exporte** | `create`, `send`, `receive`, `close` |
| **Dependances** | `libc` (socket, setsockopt, sendto, recvfrom, close) |

## Role

Fournit une abstraction autour des appels systeme pour manipuler un raw socket ICMP. Separe en 4 fonctions pour permettre la boucle send/receive dans `main`.

## API

### `create(ttl: u8) -> i32`

Cree un raw socket ICMP et configure le TTL.

| Param | Type | Description |
|-------|------|-------------|
| `ttl` | `u8` | Time To Live IP |

**Retour :** file descriptor du socket. Quitte le programme si la creation echoue (droits root requis).

```rust
let fd = unsafe {
    libc::socket(libc::AF_INET, libc::SOCK_RAW, libc::IPPROTO_ICMP)
};
```

---

### `send(fd: i32, ip: Ipv4Addr, packet: &[u8]) -> isize`

Envoie un paquet ICMP vers l'adresse IP cible.

| Param | Type | Description |
|-------|------|-------------|
| `fd` | `i32` | File descriptor du socket |
| `ip` | `Ipv4Addr` | Adresse IP de destination |
| `packet` | `&[u8]` | Paquet ICMP a envoyer |

**Retour :** nombre de bytes envoyes, ou `-1` en cas d'erreur.

Construit une `sockaddr_in` avec l'IP de destination et appelle `libc::sendto`.

---

### `receive(fd: i32, buf: &mut [u8]) -> isize`

Recoit un paquet depuis le socket (bloquant).

| Param | Type | Description |
|-------|------|-------------|
| `fd` | `i32` | File descriptor du socket |
| `buf` | `&mut [u8]` | Buffer de reception |

**Retour :** nombre de bytes recus, ou `-1` en cas d'erreur.

Note : `recvfrom` est **bloquant** — il attend qu'un paquet arrive.

---

### `close(fd: i32)`

Ferme le socket.

## Points d'Attention

- `SOCK_RAW` + `IPPROTO_ICMP` necessite les droits root (`sudo`)
- `setsockopt` configure le TTL au niveau IP (`IPPROTO_IP`, `IP_TTL`)
- L'adresse IP est convertie en network byte order avec `to_bits().to_be()`
- `recvfrom` est bloquant : si aucune reponse n'arrive, le programme reste en attente
