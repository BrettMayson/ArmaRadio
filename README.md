# Live Radio

**Bring real internet radio into Arma 3.** Choose a station, adjust the volume and listen together from vehicles or the FM Radio object.

## Features
- Radio support for cars, aircraft, boats, ships and the FM Radio object.
- Multiplayer synchronization of each radio's station, power and volume.
- Optional control from outside vehicles, only with an empty driver seat and Driver and Commander Only disabled.
- Modern and Classic interface styles with a searchable station list.
- Track information, album artwork when available, and click-to-copy track titles.
- Personal **Streamer Mode**, **Mute for me** and **Mute all radios**.
- Loading and error feedback, plus a local **Try again** button.
- Option to disable bundled stations while keeping custom stations.
- Personal station lists that can be edited in game and saved across missions.
- Global station lists configured through CBA server settings and shared with all players, alongside their personal stations.
- Zeus/Eden module to enable radio interactions on additional objects.
- Optional 3D audio inside cars and ships; audio inside aircraft remains centered.
- Configurable debug logging, CBA settings and optional key bindings.
- MP3 playback and AAC/AAC+/HLS audio decoding through the bundled decoder.

Requires **CBA_A3**. **ACE3** is supported but optional. Windows 64-bit only.

> Development version: BattlEye approval for the new binaries is not confirmed.

## Multiplayer and Streamer Mode

Each radio has its own shared station, power and volume. Personal mute controls and Streamer Mode affect only your audio.

Streamer Mode only plays stations marked as approved with `noCopyright = 1;`. Other stations remain selected for everyone but are not loaded or played locally for you. This flag does not verify a station's licensing; only use it after checking that the station is suitable for streaming or recording.


## Add your own radio stations

For one mission, add this section to its `description.ext`. For stations available across missions, place it in an expansion mod's `config.cpp` and require `live_radio_main` in that addon's `CfgPatches`.

```cpp
class CfgRadioStations {
    class MyStation {
        name = "My Radio FM";
        url = "https://example.com/live";
        condition = "true";
        noCopyright = 0;
    };
};
```

The URL must point to a **direct audio stream**, not the station's website.

| Property | Purpose |
| --- | --- |
| `name` | Station name shown in the interface. |
| `url` | Direct stream URL. |
| `picture` | Optional station image, used when album artwork is unavailable. |
| `condition` | Optional SQF expression controlling whether the station appears. Defaults to `true`. |
| `noCopyright` | Optional Streamer Mode approval. Set to `1` only for an approved station; omitted or `0` means unapproved. |

For a personal list, click **Stations** beside the search field to add, edit or delete stations. The list is saved locally across missions. Playing one of these stations shares its stream URL with other players using that radio; their personal mute and Streamer Mode settings still apply.

## Enable a radio on another object

In Zeus, place **Live Radio → Enable FM Radio on object** on the target. In Eden, synchronize the module with the target objects.

Alternatively, use an object's init field:

```sqf
[this, true] call live_radio_interface_fnc_setRadioEnabled;
```
This explicitly enables radio support on the object.



[GitHub](https://github.com/BrettMayson/ArmaRadio) · [Steam Workshop](https://steamcommunity.com/sharedfiles/filedetails/?id=2172022102)

For duplicate stream URLs, the last loaded approval wins: addon configuration, campaign, mission, then personal presets. Within each source, the last entry wins. Personal approval only affects your own Streamer Mode; other players keep their own approval settings.
