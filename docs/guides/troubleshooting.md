# Troubleshooting

## Problemes Courants

### "Operation not permitted"

```
ft_ping: socket: Operation not permitted
```

**Cause :** Les raw sockets necessitent les droits root.

**Solution :** Lancer avec `sudo` :
```bash
sudo ./target/debug/ft_ping google.com
# ou
make run ARGS="google.com"  # si le Makefile utilise sudo
```

### "Name or service not known"

```
ft_ping: example.invalid: Name or service not known
```

**Cause :** Le hostname n'a pas pu etre resolu en adresse IPv4.

**Solutions :**
- Verifier l'orthographe du hostname
- Verifier la connexion reseau
- Essayer avec une adresse IP directe : `8.8.8.8`

### Pas de reponse (programme bloque)

**Cause :** `recvfrom` est bloquant. Si le host ne repond pas, le programme attend indefiniment.

**Solution temporaire :** Ctrl+C pour interrompre.

**TODO :** Implementer un timeout sur le socket avec `setsockopt` + `SO_RCVTIMEO`.

### IPv6 au lieu d'IPv4

**Cause :** La resolution DNS retourne une adresse IPv6.

**Solution :** Le module `dns.rs` force `AF_INET` dans les hints de `getaddrinfo`, ce qui garantit une resolution IPv4.
