#include "script_component.hpp"

if (hasInterface) then {
    [QGVAR(start), {
        params ["_id", "_url", "_source"];
        // Be idempotent if an event and the JIP scan discover the same radio.
        if ((GVAR(sourceURLs) getOrDefault [_id, ""]) != _url) then {
            if (GVAR(playingSources) getOrDefault [_id, false]) then {
                EXT callExtension ["source:destroy", [_id]];
                GVAR(playingSources) deleteAt _id;
            };
            GVAR(sourcesTitles) deleteAt _id;
            GVAR(sourcesAlbumArt) deleteAt _id;
        };
        GVAR(sources) set [_id, _source];
        GVAR(sourceURLs) set [_id, _url];
        GVAR(sourceVolumes) set [_id, _source getVariable [QGVAR(volume), 1]];
        [_id] call FUNC(syncPlayback);
    }] call CBA_fnc_addEventHandler;

    [QGVAR(stop), {
        params ["_id"];
        if (GVAR(playingSources) getOrDefault [_id, false]) then {
            EXT callExtension ["source:destroy", [_id]];
        };
        GVAR(playingSources) deleteAt _id;
        GVAR(sources) deleteAt _id;
        GVAR(status) deleteAt _id;
        GVAR(sourceURLs) deleteAt _id;
        GVAR(sourceVolumes) deleteAt _id;
        GVAR(sourcesTitles) deleteAt _id;
            GVAR(sourcesAlbumArt) deleteAt _id;
    }] call CBA_fnc_addEventHandler;

    [QGVAR(volume), {
        params ["_id", "_gain"];
        // Remember changes while blocked, without creating a local audio source.
        if (_id in GVAR(sourceURLs)) then {
            GVAR(sourceVolumes) set [_id, _gain];
        };
        if (GVAR(playingSources) getOrDefault [_id, false]) then {
            EXT callExtension ["source:gain", [_id, _gain]];
        };
    }] call CBA_fnc_addEventHandler;

    [FUNC(tick), 0.05] call CBA_fnc_addPerFrameHandler;
    [FUNC(heartbeat), 0.75] call CBA_fnc_addPerFrameHandler;

    if (missionNamespace getVariable ["cba_settings_ready", false]) then {
        GVAR(playbackReady) = true;
    };

    {
        private _active = _x getVariable [QGVAR(active), []];
        if (count _active >= 2) then {
            [QGVAR(start), [_active#0, _active#1, _x]] call CBA_fnc_localEvent;
        };
    } forEach allMissionObjects "";
};

addMissionEventHandler ["EntityKilled", {
    [_this#0] call FUNC(stopEntity);
}];
addMissionEventHandler ["ExtensionCallback", {
    params ["_name", "_function", "_data"];

    if ((toLower _name) isEqualTo "live_radio_log") exitWith {
        private _entry = parseSimpleArray _data;
        private _level = toUpper (_entry param [0, "", [""]]);
        private _message = _entry param [1, "", [""]];
        [toLower _function, _level, _message] call FUNC(observeLog);
        if (!([toLower _function, _level, _message] call FUNC(shouldLog))) exitWith {};
        LOG_SYS(_function,_data);
    };
    if ((toLower _name) isNotEqualTo "live_radio") exitWith {};
    switch (_function) do {
        case "error": {
            (parseSimpleArray _data) params ["_id", "_message"];
            if (GVAR(playingSources) getOrDefault [_id, false]) then {
                private _previous = GVAR(status) getOrDefault [_id, []];
                if ((_previous param [0, ""]) == "error" && {(_previous param [2, ""]) != ""}) exitWith {};
                if ((_previous param [2, ""]) != _message) then {
                    diag_log format ["[LIVE_RADIO] Stream %1 failed: %2", _id, _message];
                };
                GVAR(status) set [_id, ["error", diag_tickTime, _message select [0, 512]]];
            };
        };
        case "album_art": {
            (parseSimpleArray _data) params ["_id", "_path"];
            if (GVAR(playingSources) getOrDefault [_id, false]) then {
                GVAR(sourcesAlbumArt) set [_id, _path];
                [QGVAR(albumArtUpdated), [_id, _path]] call CBA_fnc_localEvent;
            };
        };

        case "title": {
            (parseSimpleArray _data) params ["_id", "_title"];
            // Ignore callbacks still queued by a source that has been blocked/stopped.
            if (GVAR(playingSources) getOrDefault [_id, false]) then {
                private _previousTitle = GVAR(sourcesTitles) getOrDefault [_id, ""];
                GVAR(sourcesTitles) set [_id, _title];
                [QGVAR(metadataUpdated), [_id, _title]] call CBA_fnc_localEvent;
                // Use the same plain-text hint as title copying, once per title change.
                // Remote radios are also decoded: only notify for the listener's vehicle
                // or a nearby radio, never for every radio across the mission.
                private _source = GVAR(sources) getOrDefault [_id, objNull];
                private _listener = call CBA_fnc_currentUnit;
                private _url = GVAR(sourceURLs) getOrDefault [_id, ""];
                if (hasInterface && {_title != ""} && {_title != _previousTitle} &&
                    {alive _listener} && {!isNull _source} && {alive _source} &&
                    {(GVAR(sourceVolumes) getOrDefault [_id, 0]) > 0} &&
                    {!([_source] call FUNC(isMuted))} && {[_url] call FUNC(canHear)} &&
                    {vehicle _listener == _source || {_listener distance _source <= 5}}
                ) then {
                    hint format [localize "STR_Live_Radio_Interface_NowPlaying", _title];
                };
            };
        };
    };
}];
