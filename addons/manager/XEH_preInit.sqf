#include "script_component.hpp"
ADDON = false;
#include "XEH_PREP.hpp"
ADDON = true;

GVAR(sources) = createHashMap;
GVAR(sourcesTitles) = createHashMap;

// Make sure the extension has been loaded once
EXT callExtension "";

GVAR(copyrightSafe) = createHashMap;
{
    private _configSource = _x;
    {
        GVAR(copyrightSafe) set [getText (_x >> "url"), getNumber (_x >> "noCopyright") == 1];
    } forEach (configProperties [_configSource >> "CfgRadioStations", "isClass _x"]);
} forEach [configFile, campaignConfigFile, missionConfigFile];

[
    QGVAR(volumeMultiplier),
    "SLIDER",
    "Volume Multiplier",
    "Live Radio",
    [0.1, 1, 0.5, 2, true],
    0,
    {
        EXT callExtension ["source:global_gain", [_this]];
    }
] call CBA_fnc_addSetting;

[
    QGVAR(streamerMode),
    "CHECKBOX",
    "Streamer Mode",
    "Live Radio",
    false,
    0,
    {
        call FUNC(refreshStreamerMode);
    }
] call CBA_fnc_addSetting;
