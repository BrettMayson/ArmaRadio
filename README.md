# Live Radio

Tune in to real, live internet radio stations from vehicles and radios in Arma 3 — with full 3D positional audio, live track metadata, and per-station copyright controls for streamers.

[GitHub](https://github.com/BrettMayson/ArmaRadio)

## Features

- **Real internet radio** — stream actual live stations, not looped audio files.
- **Works in vehicles and objects** — enabled for cars and the FM Radio object out of the box.
- **Live metadata** — currently playing track title and album art are displayed as the station broadcasts them.
- **Fully configurable stations** — add your own stations, complete with custom pictures and availability conditions.
- **Streamer Mode** — automatically mute stations locally that haven't been verified as copyright-safe, without affecting what other players hear.
- **Copyright Free Only** — server wide setting that restricts available stations to those marked as copyright-free.
- **ACE compatible**

## Multiplayer

All stations are synchronized across all players in multiplayer, ensuring everyone hears the same broadcast from the same source.

## Add Your Own Radio Station

Add this to your mod or mission's description.ext:

```cpp
class CfgRadioStations {
  class my_station {
    name = "My Radio FM";
    url = "http://my-radio.fm/live";
    condition = "true"; // SQF expression that determines if the station should be available
    noCopyright = 1;
  };
};
```

The URL must point directly to a supported audio stream. A normal website URL will not work.

- **name** — the station name shown in the list.
- **url** — direct link to the stream.
- **picture** *(optional)* — path to a station picture, shown until live album art is received.
- **condition** *(optional)* — an SQF expression evaluated to determine if the station should be shown; defaults to always available.
- **noCopyright** *(optional)* — marks the station as verified safe for Streamer Mode. See below.

## Streamer Mode

Streamer Mode only plays stations explicitly marked as copyright-safe. Other stations remain selected for everyone but are muted locally for the player using Streamer Mode.

Copyright-safe stations require:

```cpp
noCopyright = 1;
```

This flag should only be used when you have verified that the station is suitable for streaming or recording.

Remove `noCopyright = 1;` if the station has not been approved for Streamer Mode.
