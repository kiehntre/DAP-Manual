# Playbooks

Practical troubleshooting flows for the DAP homelab.

## Case 0001 - DAP Manual Docker Deployment

### Symptoms

- The `dap-manual` container was running.
- Local access worked.
- `curl -I http://127.0.0.1:8008` returned `HTTP 200`.
- The public domain initially returned a Cloudflare `522`.

### Diagnosis

Docker was not the problem.

Confirmed working:

- MkDocs built successfully.
- The Docker container started.
- Port mapping worked.
- Local HTTP access worked.
- Traefik routing was checked separately.

### Lesson

If localhost returns `HTTP 200`, do not waste time blaming the application container first.

Move outward:

1. Container
2. Docker network
3. Traefik
4. Firewall/NAT
5. Cloudflare/DNS

### Useful commands

    docker ps --filter name=dap-manual
    docker logs dap-manual --tail=80
    docker port dap-manual
    curl -I http://127.0.0.1:8008
    curl -I https://manual.daponline.com

### Final note

This was a good first real incident for the manual because it proved the value of documenting the actual diagnostic path instead of writing generic waffle.
