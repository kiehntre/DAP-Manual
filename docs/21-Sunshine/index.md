# Sunshine

## Overview

Sunshine is the game streaming host. Moonlight is the client.

## Basic checks

```bash
systemctl status sunshine
journalctl -u sunshine -n 100 --no-pager
nvidia-smi
ss -tulpn | grep -E '47984|47989|48010'
```

## Common areas

- Display
- NVIDIA encoder
- Audio
- Controller forwarding
- Network latency
- Bitrate
