
1. Install docker and docker-compose

```zsh
$ apt-get update
$ apt-get upgrade -y

$ apt install docker.io docker-compose -y
$ systemctl enable docker
```

2. Create a pi hole dir

```zsh
$ mkdir ~/pihole
$ cd ~/pihole
```

3. Disable systemd resolved

```zsh
$ systemctl stop systemd-resolved
$ systemctl disable systemd-resolved.service
```

4. Use cloudflare DNS

```
$ vim /etc/resolv.conf

```

```
nameserver 1.1.1.1
```

5. Run PI hole.

```yaml
version: "3"

services:
  pihole:
    container_name: pihole
    image: pihole/pihole:latest
    ports:
      - "53:53/tcp"
      - "53:53/udp"
      - "67:67/udp"
      - "80:80/tcp"
    environment:
      TZ: 'Europe/Amsterdam'
      WEBPASSWORD: 'p0106323098P'
    volumes:
      - './etc-pihole:/etc/pihole'
      - './etc-dnsmasq.d:/etc/dnsmasq.d'
    cap_add:
      - NET_ADMIN
    restart: unless-stopped
```

```zsh
$ docker-compose up -d
```

