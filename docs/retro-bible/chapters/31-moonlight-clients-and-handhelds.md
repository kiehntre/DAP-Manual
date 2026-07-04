# Moonlight Clients and Handhelds

Moonlight is where the host setup meets real screens, real controllers and real sofas.

This draft chapter will cover client choices, handheld quirks and the practical checks that make streaming feel dependable.

## Historical Context

- Explain Moonlight as the client side of the Sunshine stack.
- Cover TVs, tablets, phones, handheld PCs and laptops.
- Note why each client may need different settings.

## Concepts

| Concept | Meaning | Draft note |
| --- | --- | --- |
| Client | Device receiving the stream. | Display and input vary widely. |
| Bitrate | Video data rate. | Needs network headroom. |
| Resolution | Stream size. | Should match the client when practical. |
| Latency | Input-to-display delay. | Affected by network, encode and display. |

## Practical Setup

- Pair each client deliberately.
- Choose resolution per device.
- Tune bitrate for network conditions.
- Test controller mapping.
- Record stable settings.

## Recommended Layout

```text
/mnt/games/streaming/moonlight/
/mnt/games/streaming/client-notes/
```

## Real-World DAP Setup

- Document known clients.
- Record recommended resolution and bitrate.
- Note controller behaviour per client.
- Note Wi-Fi and wired differences.

> **DAP Tip**
>
> A handheld does not need the same stream settings as a 4K television. Give each screen a setup that suits it.

## Common Mistakes

- Using one bitrate for every device.
- Ignoring client display aspect ratio.
- Forgetting controller order.
- Testing beside the router and assuming the whole house is fine.

## Troubleshooting

### Stream stutters

- Check Wi-Fi signal.
- Lower bitrate.
- Check host GPU load.
- Check client decode capability.

### Input feels delayed

- Check TV game mode.
- Check client frame pacing.
- Check encoder settings.
- Test wired network if possible.

## Key Points

- Moonlight tuning is client-specific.
- Network quality matters as much as emulator performance.
- Controller checks belong in the streaming test plan.

## Further Reading

- Add Moonlight documentation.
- Add network testing references.
