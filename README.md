# Music Player Daemon Pulse Audio Sink Control

Allows you to control MPD to use a different Pulse Audio sink that your default one for MPD. Useful if you want your music playing from a different output device than all other audio.

This watches for all changes to mpd and make sure that it gets routed to your selected Pulse Audio Sink.

This tool was created because I have found no way currently to set one application to use a different default sink from everything else and I was having to always adjust the output every time I started playing with MPD.

## Manual installation
```
cargo build --release
```
It will build to `target/release/mpd-pulse` which can be copied anywhere you like.

Copy `system/mpd-pulse.service` to `~/.config/systemd/user/mpd-pulse.service` and adjust the path to your `mpd-pulse` binary.

## Arch Linux installation

Install from the [AUR](https://aur.archlinux.org/packages/mpd-pulse/) and then start with systemd.

## Starting with systemd

Start the service.
```
systemctl --user start mpd-pulse.service
```

Enable the service to start automatically.
```
systemctl --user enable mpd-pulse.service
```
