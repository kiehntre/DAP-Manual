# Docker

## Overview

Docker is the default deployment method for services in the homelab.

It keeps services isolated, repeatable and easier to back up.

## Standard Compose Pattern

```yaml
services:
  example:
    image: example/example:latest
    container_name: example
    restart: unless-stopped
    networks:
      - saltbox

networks:
  saltbox:
    external: true
```

## Useful Commands

```bash
docker ps
docker compose ps
docker compose logs -f
docker compose pull
docker compose up -d
docker compose down
docker network ls
docker volume ls
```

## Troubleshooting

### Container keeps restarting

```bash
docker logs container-name --tail=100
docker inspect container-name
```

### Port conflict

```bash
ss -tulpn
docker ps --format "table {{.Names}}\t{{.Ports}}"
```

### Network issue

```bash
docker network inspect saltbox
docker exec -it container-name sh
```

## DAP Rule

!!! warning
    Do not blindly restart half the stack. Check logs first, then change one thing at a time.
