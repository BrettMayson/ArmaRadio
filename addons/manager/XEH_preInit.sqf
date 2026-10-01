#include "script_component.hpp"
ADDON = false;
#include "XEH_PREP.hpp"
ADDON = true;

GVAR(status) = createHashMap;
GVAR(sources) = createHashMap;
GVAR(sourcesTitles) = createHashMap;
GVAR(sourcesAlbumArt) = createHashMap;
GVAR(sourceURLs) = createHashMap;
GVAR(sourceVolumes) = createHashMap;
GVAR(playingSources) = createHashMap;
GVAR(listenerAlive) = true;
GVAR(cleanupLogReported) = false;
GVAR(stationPolicy) = [] call FUNC(loadStationPolicy);
// Do not start any audio until CBA has applied the saved personal settings.
GVAR(playbackReady) = false;

// Make sure the extension has been loaded once
EXT callExtension "";
[
    QGVAR(debugMessages),
    "CHECKBOX",
    [localize "STR_Live_Radio_Manager_DebugMessages", localize "STR_Live_Radio_Manager_DebugMessagesTooltip"],
    "Live Radio",
    false,
    0,
    {},
    false
] call CBA_fnc_addSetting;

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
    QGVAR(inside3DAudio), "CHECKBOX",
    [localize "STR_Live_Radio_Manager_Inside3DAudio", localize "STR_Live_Radio_Manager_Inside3DAudioTooltip"],
    "Live Radio", false, 2, {}, false
] call CBA_fnc_addSetting;

// Streamer Mode is owned by the interface and saved directly in the local profile.
if (isNil {profileNamespace getVariable QGVAR(streamerMode)}) then {
    private _oldHash = profileNamespace getVariable ["cba_settings_hash", []];
    private _previous = [false, 0];
    if (_oldHash isEqualType [] && {count _oldHash >= 3}) then {
        _previous = [_oldHash, toLower QGVAR(streamerMode), [false, 0]] call CBA_fnc_hashGet;
    };
    profileNamespace setVariable [QGVAR(streamerMode), _previous param [0, false, [false]]];
    saveProfileNamespace;
};
GVAR(streamerMode) = profileNamespace getVariable [QGVAR(streamerMode), false];
[
    QGVAR(muteAll), "CHECKBOX",
    [localize "STR_Live_Radio_Manager_MuteAll", localize "STR_Live_Radio_Manager_MuteAllTooltip"],
    "Live Radio", false, 2,
    {
        GVAR(muteAll) = _this;
        if (GVAR(playbackReady)) then {call FUNC(refreshPlayback)};
        [QGVAR(streamerModeChanged), []] call CBA_fnc_localEvent;
    }, false
] call CBA_fnc_addSetting;

["CBA_settingsInitialized", {
    GVAR(playbackReady) = true;
    call FUNC(refreshPlayback);
}] call CBA_fnc_addEventHandler;
