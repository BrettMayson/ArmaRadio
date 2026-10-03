#include "script_component.hpp"
ADDON = false;
#include "XEH_PREP.hpp"
ADDON = true;

GVAR(status) = createHashMap;
GVAR(failureLog) = createHashMap;
GVAR(nativeLogTimes) = createHashMap;
GVAR(nativeIDs) = createHashMap;
GVAR(attemptSources) = createHashMap;
GVAR(autoAttempts) = createHashMap;
GVAR(attemptVehicle) = createHashMap;
GVAR(retryDue) = createHashMap;
GVAR(attemptSerial) = 0;
GVAR(sources) = createHashMap;
GVAR(sourcesTitles) = createHashMap;
GVAR(sourcesAlbumArt) = createHashMap;
GVAR(sourceURLs) = createHashMap;
GVAR(sourceVolumes) = createHashMap;
GVAR(playingSources) = createHashMap;
GVAR(listenerAlive) = true;
GVAR(listenerVehicle) = objNull;
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
    localize "STR_Live_Radio_Manager_VolumeMultiplier",
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

// Keep the previous interface-only preference when migrating to CBA settings.
private _savedStreamerMode = profileNamespace getVariable [QGVAR(streamerMode), false];
[
    QGVAR(streamerMode), "CHECKBOX",
    [localize "STR_Live_Radio_Manager_StreamerMode", localize "STR_Live_Radio_Interface_StreamerTooltip"],
    "Live Radio", _savedStreamerMode, 2,
    {
        // CBA updates the missionNamespace value and persists the client setting.
        if (GVAR(playbackReady)) then {call FUNC(refreshPlayback)};
        [QGVAR(streamerModeChanged), []] call CBA_fnc_localEvent;
    }, false
] call CBA_fnc_addSetting;
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

[
    QGVAR(autoRetry), "CHECKBOX",
    [localize "STR_Live_Radio_Manager_AutoRetry", localize "STR_Live_Radio_Manager_AutoRetryTooltip"],
    "Live Radio", false, 2,
    {
        GVAR(autoRetry) = _this;
        // No delayed closures or in-flight automatic reconnects survive disable.
        GVAR(retryDue) = createHashMap;
        if (!_this) then {
            {
                if ((GVAR(autoAttempts) get _x) == (GVAR(nativeIDs) getOrDefault [_x, ""])) then {
                    [_x] call FUNC(destroyLocal);
                    GVAR(status) set [_x, ["ended", diag_tickTime]];
                };
            } forEach (keys GVAR(autoAttempts));
        } else {
            // Re-enabling is a new user request, not revival of an old timer.
            if (GVAR(playbackReady)) then {
                {
                    if ((GVAR(nativeIDs) getOrDefault [_x, ""]) == "" && {
                        ((GVAR(status) getOrDefault [_x, [""]])#0) in ["error", "ended"]
                    }) then {
                        [_x] call FUNC(syncPlayback);
                        GVAR(autoAttempts) set [_x, GVAR(nativeIDs) getOrDefault [_x, ""]];
                    };
                } forEach (keys GVAR(sources));
            };
        };
    }, false
] call CBA_fnc_addSetting;
