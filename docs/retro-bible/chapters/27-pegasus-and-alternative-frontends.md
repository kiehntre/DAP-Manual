# Pegasus and Alternative Frontends

Not every setup needs the same frontend.

This draft chapter will compare Pegasus and other alternatives as practical tools, not as team jerseys.

## Historical Context

- Explain why frontend choice became its own category.
- Cover desktop, handheld, arcade cabinet and TV use cases.
- Mention that frontends often outlive individual emulator choices.

## Concepts

| Concept | Meaning | Draft note |
| --- | --- | --- |
| Frontend | Launcher and library interface. | Does not emulate by itself. |
| Theme | Visual presentation layer. | Can affect readability. |
| Metadata file | Local game information. | Format differs by frontend. |
| Launcher command | How a game is started. | The most important part. |

## Practical Setup

- Start from the display and controller use case.
- Check metadata format.
- Check launch command flexibility.
- Check Linux packaging.
- Test suspend, resume and streaming behaviour.

## Recommended Comparison

| Frontend | Strength | Watch for |
| --- | --- | --- |
| ES-DE | Familiar controller interface. | System config structure. |
| Pegasus | Flexible metadata and themes. | More manual setup. |
| Steam | Streaming and controller layer. | Library scale and metadata. |
| RomM | Web access and management. | Not a couch frontend by itself. |

## Real-World DAP Setup

- Document which frontend is primary.
- Document any secondary use.
- Record path sharing rules.

> **DAP Tip**
>
> Pick the frontend that matches the room. A web browser, arcade cabinet and handheld do not need to behave the same way.

## Common Mistakes

- Choosing a frontend only because screenshots look nice.
- Ignoring controller navigation.
- Forgetting metadata portability.
- Keeping several frontends half-configured.

## Troubleshooting

### Frontend shows games but launches nothing

- Check command templates.
- Check emulator paths.
- Check file quoting.
- Check current working directory.

### Theme looks poor on TV

- Check font size.
- Check contrast.
- Check overscan.
- Check controller focus indicators.

## Key Points

- Frontends are workflow choices.
- Launch reliability matters more than theme polish.
- Multiple frontends can coexist if storage remains sane.

## Further Reading

- Add Pegasus documentation.
- Add frontend comparison references.
