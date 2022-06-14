# my-i3status

My custom i3 status bar that displays the current date, time, how light it is outside, and a list of daily todos (inspired by [Loop habit tracker](https://loophabits.org/)).

To use it, download the executable in releases and replace your status command in the i3 config with this. Replace the path and your coordinates with the correct values.
```sh
status_command /path/to/my-i3status lat long
```

If your longitude is in degrees west, your longitude needs to be negative
```sh
# Berlin:
status_command /path/to/my-i3status 52 13

# New York:
status_command /path/to/my-i3status 40 -74
```

If your location is in the southern hemisphere, your latitude needs to be negative
```sh
# Santa:
status_command /path/to/my-i3status 90 0

# Evil santa:
status_command /path/to/my-i3status -90 0
```

Note: Using your exact location isn't neccesary, and doing so would also be a security risk if you send screenshots, since someone could deduce your location from the time, date, and daylight indicator displayed by the status bar

Daily todos are defined by a `todos.json` file in your .i3 directory, with this format:

```json
{
    "todos": [
        {
            "name": "Meditate", // The name displayed in the block
            "color": "#00aeff", // The color of the block
            "interval": 2, // Should be done once every `interval` days
            "done_today": "2022-06-14", // Did you do it today? (automatically set by the program)
            "last_completed": "2022-06-12" // When's the last time you did it? (automatically set by the program)
        }
    ]
}
```
