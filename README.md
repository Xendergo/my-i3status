# my-i3status

My custom i3 status bar that displays the current date, time, and how light it is outside.

To use it, download the executable in releases and replace your status command in the i3 config with this. Replace the path and your coordinates with the correct values.
```
status_command /path/to/my-i3status lat long
```

If your longitude is in degrees west, your longitude needs to be negative
```
# Berlin:
status_command /path/to/my-i3status 52 13

# New York:
status_command /path/to/my-i3status 40 -74
```

If your location is in the southern hemisphere, your latitude needs to be negative
```
# Santa:
status_command /path/to/my-i3status 90 0

# Evil santa:
status_command /path/to/my-i3status -90 0
```

Note: Using your exact location isn't neccesary, and doing so would also be a security risk if you send screenshots, since someone could deduce your location from the time, date, and daylight indicator displayed by the status bar
