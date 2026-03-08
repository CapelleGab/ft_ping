# dns.rs

> Resolution DNS : convertit un hostname en adresse IPv4.

## Informations

| Propriete | Valeur |
|-----------|--------|
| **Fichier** | `src/dns.rs` |
| **Exporte** | `resolve(host: &str) -> Result<Ipv4Addr, String>` |
| **Dependances** | `libc::getaddrinfo`, `libc::freeaddrinfo` |

## Role

Resout un hostname (ex: `google.com`) ou une adresse IP en `Ipv4Addr` en utilisant l'appel systeme `getaddrinfo` via `libc`. Force la resolution en IPv4 uniquement (`AF_INET`).

## API

### `resolve(host: &str) -> Result<Ipv4Addr, String>`

**Parametres :**

| Param | Type | Description |
|-------|------|-------------|
| `host` | `&str` | Hostname ou adresse IP a resoudre |

**Retour :** `Ok(Ipv4Addr)` ou `Err(String)` avec message d'erreur.

## Fonctionnement

1. Convertit le hostname en `CString` (requis par l'API C)
2. Configure les hints : `ai_family = AF_INET`, `ai_socktype = SOCK_RAW`
3. Appelle `libc::getaddrinfo`
4. Extrait l'adresse IPv4 du `sockaddr_in` retourne
5. Libere la memoire avec `libc::freeaddrinfo`

## Code

```rust
pub fn resolve(host: &str) -> Result<Ipv4Addr, String> {
    let c_host = CString::new(host)
        .map_err(|_| format!("ft_ping: {}: Invalid hostname", host))?;

    let mut hints: libc::addrinfo = unsafe { std::mem::zeroed() };
    hints.ai_family = libc::AF_INET;
    hints.ai_socktype = libc::SOCK_RAW;

    let mut res: *mut libc::addrinfo = ptr::null_mut();
    let ret = unsafe {
        libc::getaddrinfo(c_host.as_ptr(), ptr::null(), &hints, &mut res)
    };

    if ret != 0 || res.is_null() {
        return Err(format!("ft_ping: {}: Name or service not known", host));
    }

    let addr = unsafe {
        let sockaddr = (*res).ai_addr as *const libc::sockaddr_in;
        let ip = (*sockaddr).sin_addr.s_addr.to_be();
        libc::freeaddrinfo(res);
        Ipv4Addr::from(ip)
    };

    Ok(addr)
}
```

## Points d'Attention

- La memoire allouee par `getaddrinfo` doit etre liberee avec `freeaddrinfo`
- `AF_INET` force la resolution en IPv4 uniquement
- Le hostname est converti en `CString` car `getaddrinfo` attend un `*const c_char`
