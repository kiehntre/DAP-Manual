# Sunshine Host Configuration

Sunshine is the doorway between the retro host and the screens around the house.

This draft chapter will cover host setup, app entries and the small configuration choices that make streaming feel clean instead of fragile.

## Historical Context

- Explain game streaming in the post-GameStream world.
- Position Sunshine as the host side of the setup.
- Note why local network quality matters.

## Concepts

| Concept | Meaning | Draft note |
| --- | --- | --- |
| Host | Machine running the game. | Encodes video and receives input. |
| App entry | Launchable item in Sunshine. | Can start Steam, ES-DE or scripts. |
| Encoder | Hardware or software video compression. | Affects quality and latency. |
| Pairing | Trust relationship with client. | Needs care on shared networks. |

## Practical Setup

- Install and secure Sunshine.
- Set host display behaviour.
- Create app entries for frontends and Steam.
- Test one client before adding many.
- Record encoder settings.

## Recommended Layout

```text
/mnt/games/streaming/sunshine/
/mnt/games/streaming/scripts/
/mnt/games/streaming/notes/
```

## Real-World DAP Setup

- Document Sunshine install method.
- Document service status.
- Document app entries.
- Record display and resolution strategy.

> **DAP Warning**
>
> Streaming problems are often host, network and client combined. Change one thing at a time or the trail goes cold.

## Common Mistakes

- Leaving app entries unnamed or unclear.
- Ignoring firewall rules.
- Testing only on a desktop monitor.
- Forgetting audio device behaviour.
- Changing resolution without checking frontend scaling.

## Troubleshooting

### Client cannot connect

- Check Sunshine service.
- Check firewall.
- Check host IP.
- Check pairing status.

### Stream starts but app does not launch

- Check command path.
- Check user permissions.
- Check working directory.
- Check display session.

## Key Points

- Sunshine should have clear, named launch entries.
- Host display and encoder settings affect every client.
- Streaming configuration needs notes, not guesswork.

## Further Reading

- Add Sunshine documentation.
- Add encoder and firewall references.
