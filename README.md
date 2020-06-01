# QuietMisdreavus Wallpaper Cycler

this is a personal tool i wrote to make it so that i could still have multiple wallpapers rotating
regularly on GNOME 3, even though it doesn't have a settings option to source wallpaper from a
folder. many assumptions are specific to the system i'm running it on.

the key trick in use here is that i can switch the wallpaper with the following command:

```
gsettings set org.gnome.desktop.background picture-uri file:///path/to/image.png
```

everything else is scaffolding to keep track of the current wallpaper (or at least, the wallpaper
most recently set by this command), and a folder to source wallpapers from.

to get the timer working, i decided to use a systemd timer, rather than turning the application into
a daemon (and starting it as a plain systemd service). the key reason for this is because systemd
timers let you specify wall-clock times to execute, meaning i can make sure that the times that the
wallpapers change is synchronized to the hour, and whatever multiple i desire (in this case, 10
minutes).

(for some reason, no other wallpaper organizer i've ever used on Linux has this property; i'm way
too used to it from Windows and macOS having this behavior.)

to set this up, use the following commands after cloning the repo:

```
cargo install --path .
qmwc --set-dir /path/to/wallpaper/directory
ln -s $(pwd)/qmwc.service $(pwd)/qmwc.timer ~/.config/systemd/user/
systemctl --user daemon-reload
systemctl --user enable qmwc.timer
systemctl --user start qmwc.timer
```
