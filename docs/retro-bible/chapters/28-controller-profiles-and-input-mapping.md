# Controller Profiles and Input Mapping

Controllers are where a tidy setup either feels invisible or falls apart.

This draft chapter will cover input mapping as a maintainable system, not a one-night fight with every emulator menu.

## Historical Context

- Explain why console layouts do not map cleanly to modern controllers.
- Cover XInput, SDL, evdev and Steam Input at a high level.
- Note arcade, light gun, wheel and keyboard exceptions.

## Concepts

| Concept | Meaning | Draft note |
| --- | --- | --- |
| Physical button | Button on the controller. | Label may not match target system. |
| Logical input | Input seen by emulator. | Depends on driver and mapping. |
| Hotkey | Button combo for emulator actions. | Needs consistency. |
| Profile | Saved mapping. | Should be backed up. |

## Practical Setup

- Define a DAP default controller.
- Map menu, quit and save hotkeys consistently.
- Create per-system profiles where needed.
- Document special controllers separately.
- Back up input configs.

## Recommended Layout

```text
/mnt/games/controllers/profiles/
/mnt/games/controllers/notes/
/mnt/games/backups/controllers/
```

## Real-World DAP Setup

- Record primary controllers.
- Record Steam Input policy.
- Record emulator profile paths.
- Note Moonlight client differences.

> **DAP Warning**
>
> Do not rely on memory for hotkeys. A forgotten quit combo is funny once and annoying forever.

## Common Mistakes

- Mapping by button label instead of in-game function.
- Using global hotkeys that conflict with games.
- Forgetting player order.
- Ignoring Bluetooth reconnect behaviour.
- Losing profiles during emulator cleanup.

## Troubleshooting

### Buttons are swapped

- Check controller mode.
- Check SDL mapping.
- Check Steam Input layer.
- Check emulator profile.

### Controller works locally but not over Moonlight

- Check client controller type.
- Check Sunshine input settings.
- Check Steam Input involvement.
- Check player order.

## Key Points

- Input mapping needs a house style.
- Hotkeys should be documented.
- Streaming adds another input layer.

## Further Reading

- Add SDL controller mapping references.
- Add Steam Input documentation.
- Add emulator input documentation.
