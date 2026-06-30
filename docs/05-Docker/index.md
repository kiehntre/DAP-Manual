# Docker

## Overview

Docker is one of the core building blocks of the homelab.

It is used because containers are repeatable, easy to update, easy to back up, and easier to reason about than a pile of hand-installed services scattered across the host.

This chapter documents the Docker patterns used in the DAP homelab.

## Role in the Homelab

Docker is used for:

- Media services
- AI services
- Search and index tools
- Documentation services
- Supporting databases and caches
- Reverse-proxied web applications
- Internal tools and experiments

The general rule is simple:

!!! tip
    If a service can sensibly run in Docker and does not need deep host integration, run it in Docker.

## Core Principles

### 1. Compose first

Prefer Docker Compose over long `docker run` commands.

Compose gives:

- repeatable configuration
- readable service definitions
- easy upgrades
- version control
- better disaster recovery

### 2. One service, one directory

Use a predictable layout.

Example:

```text
/opt/service-name/
    compose.yml
    .env
    config/
    data/
```

For the DAP Manual development repo:

```text
~/Projects/DAP-Manual/
    compose.yml
    Dockerfile
    docs/
    scripts/
```

### 3. Keep important config in Git

Keep these in Git where safe:

- Compose files
- Documentation
- Non-secret config templates
- Scripts
- Systemd units
- Notes and playbooks

Do **not** commit secrets:

- API keys
- tokens
- passwords
- private keys
- `.env` files containing secrets

### 4. Read logs before restarting everything

Do not fire the restart cannon first.

Use:

```bash
docker logs container-name --tail=100
docker compose logs --tail=100
```

Then decide what to do.

## Standard Compose Pattern

A simple service usually looks like this:

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

## Traefik Pattern

For services exposed through Traefik:

```yaml
services:
  example:
    image: example/example:latest
    container_name: example
    restart: unless-stopped
    networks:
      - saltbox
    labels:
      - "traefik.enable=true"
      - "traefik.docker.network=saltbox"
      - "traefik.http.routers.example.rule=Host(`example.daponline.com`)"
      - "traefik.http.routers.example.entrypoints=websecure"
      - "traefik.http.routers.example.tls=true"
      - "traefik.http.routers.example.tls.certresolver=cloudflare"
      - "traefik.http.services.example.loadbalancer.server.port=8000"

networks:
  saltbox:
    external: true
```

### Why `traefik.docker.network` matters

If a container is attached to more than one Docker network, Traefik may choose the wrong one.

This can cause confusing failures where:

- the container is running
- local access works
- Docker looks fine
- the public URL fails

The fix is to tell Traefik exactly which Docker network to use:

```yaml
- "traefik.docker.network=saltbox"
```

## Local Port Pattern

Sometimes it is useful to keep a local test port as well as Traefik.

Example:

```yaml
ports:
  - "8008:8000"
```

This allows:

```bash
curl -I http://127.0.0.1:8008
```

If this returns `HTTP 200`, the application and container are alive.

## Essential Commands

### List containers

```bash
docker ps
```

### Show all containers

```bash
docker ps -a
```

### View logs

```bash
docker logs container-name --tail=100
docker compose logs --tail=100
```

### Follow logs

```bash
docker logs -f container-name
docker compose logs -f
```

### Restart a service

```bash
docker compose restart service-name
```

### Rebuild and start

```bash
docker compose up -d --build
```

### Stop a stack

```bash
docker compose down
```

### Pull updates

```bash
docker compose pull
docker compose up -d
```

## Networking Commands

### List networks

```bash
docker network ls
```

### Inspect a network

```bash
docker network inspect saltbox
```

### Inspect container networks

```bash
docker inspect container-name --format '{{json .NetworkSettings.Networks}}' | jq
```

### Inspect labels

```bash
docker inspect container-name --format '{{json .Config.Labels}}' | jq
```

## Port Diagnostics

### See host listening ports

```bash
ss -tulpn
```

### See Docker port mapping

```bash
docker port container-name
```

### See exposed ports

```bash
docker ps --format "table {{.Names}}\t{{.Ports}}"
```

## Health Checks

If a container has a health check:

```bash
docker ps --format "table {{.Names}}\t{{.Status}}"
```

A healthy container shows:

```text
Up 3 hours (healthy)
```

An unhealthy container needs logs first, not blind restarts.

## Volumes and Data

### List volumes

```bash
docker volume ls
```

### Inspect a volume

```bash
docker volume inspect volume-name
```

### Bind mounts

Bind mounts are easier to understand during recovery because they point to normal host paths.

Example:

```yaml
volumes:
  - ./config:/config
  - /mnt/unionfs/Media:/media
```

## Troubleshooting Flow

### Symptom: Service page will not load

Check in this order:

1. Is the container running?
2. Is the application listening inside the container?
3. Is the port mapped locally?
4. Is Traefik routing to the right container?
5. Is DNS pointing to the right place?
6. Is Cloudflare reaching the origin?

Commands:

```bash
docker ps --filter name=service-name
docker logs service-name --tail=100
docker port service-name
curl -I http://127.0.0.1:PORT
docker logs traefik --tail=150 | grep -i service-name
curl -I https://service.daponline.com
```

### Symptom: Local works but public URL fails

If this works:

```bash
curl -I http://127.0.0.1:8008
```

but this fails:

```bash
curl -I https://manual.daponline.com
```

then the container is probably fine.

Move outward:

1. Docker network
2. Traefik labels
3. Traefik logs
4. Firewall/NAT
5. Cloudflare DNS/proxy

### Symptom: Container keeps restarting

Use:

```bash
docker logs container-name --tail=200
docker inspect container-name
```

Look for:

- bad environment variables
- missing mounted files
- permissions errors
- port conflicts
- database connection failures

### Symptom: YAML error

Run:

```bash
docker compose config
```

This validates the Compose file before starting the stack.

Common causes:

- bad indentation
- tabs instead of spaces
- missing quotes
- pasted text on the wrong line
- `:` characters in unquoted values

## DAP Manual Incident: MkDocs Container

### What happened

The DAP Manual container built and ran locally, but the public URL initially returned a Cloudflare error.

### What proved Docker was fine

```bash
curl -I http://127.0.0.1:8008
```

returned:

```text
HTTP/1.0 200 OK
```

The container also showed:

```text
0.0.0.0:8008->8000/tcp
```

### Lesson

If localhost returns HTTP 200, do not keep attacking the app container.

Move outward to:

- Traefik
- Docker network selection
- DNS
- Cloudflare
- firewall/NAT

## Backup Notes

For Docker services, back up:

- Compose files
- `.env` files
- bind-mounted config folders
- databases
- custom scripts
- documentation

Do not rely on container images as backups. Images can be pulled again. Configuration and data cannot always be recreated.

## Quick Reference

```bash
docker ps
docker ps -a
docker logs NAME --tail=100
docker compose logs --tail=100
docker compose config
docker compose up -d
docker compose up -d --build
docker compose pull
docker compose down
docker network ls
docker network inspect saltbox
docker inspect NAME
docker port NAME
ss -tulpn
```

## DAP Rules

!!! warning "Do not restart the world"
    Restart the smallest thing that could fix the problem. If you restart ten services at once, you may hide the actual cause.

!!! tip "Local first"
    Always test locally before blaming DNS, Cloudflare, or Traefik.

!!! tip "Logs before guesses"
    Logs are the crime scene. Look before sweeping it up.
