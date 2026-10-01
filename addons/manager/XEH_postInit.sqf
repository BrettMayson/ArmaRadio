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
                GVAR(sourcesTitles) set [_id, _title];
                [QGVAR(metadataUpdated), [_id, _title]] call CBA_fnc_localEvent;
            };
        };
    };
}];
