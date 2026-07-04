# Remote Gaming Design

Remote gaming is not only a streaming app. It is a design choice for the whole retro setup.

This chapter is a skeleton for thinking about where games run, where they are displayed and how the player actually gets back to the menu when the fun is done.

## Historical Context

Home streaming used to feel like a trick. Now it can be the main way a library is used. Sunshine, Moonlight, Steam Remote Play and handheld PCs have made the gaming machine less tied to one physical screen.

## Practical Advice

- Decide whether the host is a desktop, server or living-room machine.
- Keep launch targets simple.
- Make exit hotkeys consistent.
- Match stream resolution to the client.
- Test from the actual sofa, not only beside the router.

## Linux-First Recommendations

Linux hosts should document:

- display server;
- GPU and encoder;
- audio device;
- Sunshine app entries;
- controller path;
- firewall rules;
- wake and sleep behaviour.

## DAP Gold Standard Setup

```text
/mnt/games/streaming/
  host-notes.md
  client-notes.md
  launch-scripts/
  test-results/
```

> **DAP Tip**
>
> A good remote setup has a boring exit path. If quitting a game needs a keyboard in another room, it is not finished.

## Common Mistakes

- Testing only one client.
- Ignoring TV latency settings.
- Streaming a desktop resolution that does not suit the client.
- Forgetting that controller order can change.

## Troubleshooting

### Stream is sharp but feels bad

Check latency, frame pacing, client decode, TV game mode and network jitter.

### Game launches behind the frontend

Check fullscreen settings, window focus, launch script timing and desktop environment behaviour.

## DAP Warning

Remote play hides complexity until it breaks. Keep host, client and network notes separate so troubleshooting has somewhere to start.

## What Comes Next

Next comes GPU passthrough: the heavier option for people who need a virtual machine to behave like a real gaming PC.
