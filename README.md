# jiggle - A Tiny Program to Jiggle Your Mouse

Tired of keeping your laptop unlocked while you do your boring job, where you have nothing to do?
Tired of your manager telling you that you went "away" for far too long?
Worry no longer! `jiggle` is here to empower you to do other things when you're "working" from
home!

*Jokes apart*, jiggle is a tiny program to help you keep your laptop awake when you don't want the
screen to suspend. This is very useful when you're keeping an eye on a dashboard perhaps, or you
want to somehow keep your system unlocked for some reason.

## Usage

Run `jiggle` to start with the default 10-second interval. A green circle appears in your menu bar.

```
jiggle          # jiggle every 10s
jiggle 30       # jiggle every 30s
JIGGLE_SLEEP=60 jiggle  # via environment variable
```

What Jiggle does is it moves your mouse a *tiny* bit and immediately moves it back. That's enough
to keep your computer from sleeping.

### Tray icon

Click the menu bar icon to:

- **Pause / Resume** — temporarily stop jiggling (icon turns gray while paused)
- **Quit Jiggle** — exit the app

## Installation

Download the binary from the releases, and run the command whenever you want.

Note that you should install `libxdo-dev`, `xdotool`, or `libxdo-devel` if you're trying to build
this using `cargo` on Linux.

This should work on macOS, Windows, and Linux.

Raise issues [here](https://github.com/stonecharioteer/wfh-jiggle).
