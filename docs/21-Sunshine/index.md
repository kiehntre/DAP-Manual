# Sunshine

## Overview

Sunshine is the game streaming host. Moonlight is the client.

The goal is low-latency streaming from the homelab/gaming VM to laptops, TVs and handheld devices.

## Key Areas

- GPU encoder
- Display handling
- Audio capture
- Controller forwarding
- Network throughput
- Client bitrate

## Basic Checks

```bash
systemctl status sunshine
journalctl -u sunshine -n 100 --no-pager
nvidia-smi
ip addr
ss -tulpn | grep -E '47984|47989|48010'
```

## Common Symptoms

| Symptom | Likely Area |
|---|---|
| Black screen | Display/Xorg/Wayland/dummy plug |
| Stutter | Network, encoder, bitrate |
| No audio | PipeWire/PulseAudio sink |
| Controller not working | udev, Moonlight mapping, Steam Input |
| Poor image quality | Bitrate, codec, client decoder |

## Network Notes

For high-quality local streaming:

- Prefer wired Ethernet.
- Use 5 GHz/6 GHz Wi-Fi only if wired is not practical.
- Keep latency more important than raw speed.
- Check packet loss before blaming Sunshine.

## DAP Rule

!!! tip
    If Sunshine is bad but native desktop audio/video is fine, test network and encoder before reinstalling everything like a lunatic with a screwdriver.
