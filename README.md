# jiggle - A Tiny Program to Jiggle Your Mouse

Tired of keeping your laptop unlocked while you do your boring job, where you have nothing to do?
Tired of your manager telling you that you went "away" for far too long?
Worry no longer! `jiggle` is here to empower you to do other things when you're "working" from
home!

*Jokes apart*, jiggle is a tiny program to help you keep your laptop awake when you don't want the
screen to suspend. This is very useful when you're keeping an eye on a dashboard perhaps, or you
want to somehow keep your system unlocked for some reason.

## Usage

Run `jiggle` to run this command every 10 seconds. If you'd like to adjust the time setting, you
can use `jiggle <SECONDS>`, such as `jiggle 60` to run the command every 60s.

What Jiggle does is it moves your mouse a *tiny* bit every few seconds. That's enough to keep your
computer from sleeping.

## Installation

Download the binary from the releases, and run the command whenever you want.

Note that you should install `libxdo-dev`, `xdotool`, or `libxdo-devel` if you're trying to build
this using `cargo` on Linux.

This should work on MacOs, Windows and Linux.

Raise issues [here](https://github.com/stonecharioteer/wfh-jiggle).
