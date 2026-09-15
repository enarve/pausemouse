# Pausemouse
```
 _  _
(o)(o)--.
 \../ (  )hjw
 m\/m--m'`--.
```
Pausemouse reminds you to take short breaks while working on your desktop.

Change work duration and break duration settings from system tray menu or from config file. Don't forget to take a break, when message appears on the screen.

## Build
```sh
$ cargo build
```

On macOS optionally use `bundle.sh` to create `.app`:
```sh
$ chmod +x bundle.sh
$ ./bundle.sh
```
It will appear in `target/bundle/osx`.

Config is stored at `/pausemause/config.toml` in your system's default config directory.