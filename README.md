A SFX application to be used on tabletop RPG sessions.

## Where files are stored

tt-sfx follows the OS conventions for application directories (via the `directories` crate). On Linux:

| Directory | Path | Contents |
|---|---|---|
| Data | `~/.local/share/TtSfx` | The host's sound files. This is the library the DM owns, so it should be kept. |
| Cache | `~/.cache/TtSfx` | Sound files a client received from a host. Safe to delete; they are re-synced when needed. |
| Config | `~/.config/TtSfx` | General user configuration. |

Paths differ on Windows and macOS.
