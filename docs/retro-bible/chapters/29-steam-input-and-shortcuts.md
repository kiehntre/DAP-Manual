# Steam Input and Shortcuts

Steam can be a powerful glue layer for controllers, streaming and non-Steam games.

This draft chapter will explain how to use it without letting it quietly take over every input decision.

## Historical Context

- Explain Steam as more than a storefront.
- Cover Steam Input, Big Picture and non-Steam shortcuts.
- Place it beside Sunshine, Moonlight and standalone frontends.

## Concepts

| Concept | Meaning | Draft note |
| --- | --- | --- |
| Steam Input | Controller translation layer. | Useful, but can double-map inputs. |
| Non-Steam shortcut | Launch entry for external software. | Can point to frontends or emulators. |
| Big Picture | Controller-first Steam interface. | Useful on TVs. |
| Per-game layout | Shortcut-specific input config. | Needs naming discipline. |

## Practical Setup

- Decide when Steam launches the frontend.
- Decide when Steam launches individual games.
- Use clear shortcut names.
- Test controller behaviour with and without Steam Input.
- Back up shortcut notes where possible.

## Recommended Layout

```text
/mnt/games/steam-shortcuts/notes/
/mnt/games/controllers/steam-input/
```

## Real-World DAP Setup

- Document the main Steam launch path.
- Document Sunshine app entries that call Steam.
- Record any Steam Input templates.

> **DAP Tip**
>
> Steam Input is best treated as a deliberate layer. If it is enabled by accident, debugging controllers gets muddy fast.

## Common Mistakes

- Enabling Steam Input everywhere.
- Creating duplicate shortcuts with slightly different names.
- Forgetting working directory and launch options.
- Using Steam artwork as the only metadata record.

## Troubleshooting

### Controller input is doubled

- Check Steam Input.
- Check emulator input driver.
- Check frontend controller handling.
- Check Moonlight virtual controller.

### Shortcut opens the wrong thing

- Check target path.
- Check launch options.
- Check whether the emulator moved.
- Check Flatpak command format.

## Key Points

- Steam is a useful launch and input layer.
- It should be documented like any other part of the stack.
- Controller issues often come from stacked input translation.

## Further Reading

- Add Steam Input documentation.
- Add Steam shortcut backup references.
