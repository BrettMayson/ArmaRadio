#include "script_component.hpp"

if (hasInterface) then {
    [QGVAR(start), {
        params ["_id", "_url", "_source"];
        private _gain = _source getVariable [QGVAR(volume), 1];
        if (missionNamespace getVariable [QGVAR(streamerMode), false] && {!([_url] call FUNC(isCopyrightSafe))}) then {
            _gain = 0;
        };
        EXT callExtension ["source:new", [_id, _url, _gain]];
        GVAR(sources) set [_id, _source];
        [QGVAR(metadataUpdated), [_id, ""]] call CBA_fnc_localEvent;
    }] call CBA_fnc_addEventHandler;

    [QGVAR(stop), {
        params ["_id"];
        EXT callExtension ["source:destroy", [_id]];
        GVAR(sources) deleteAt _id;
        GVAR(sourcesTitles) deleteAt _id;
        GVAR(sourcesAlbumArt) deleteAt _id;
    }] call CBA_fnc_addEventHandler;

    [QGVAR(volume), {
        params ["_id", "_gain"];
        private _source = GVAR(sources) getOrDefault [_id, objNull];
        private _url = _source getVariable [QGVAR(active), []] param [1, ""];
        if (missionNamespace getVariable [QGVAR(streamerMode), false] && {!([_url] call FUNC(isCopyrightSafe))}) then {
            _gain = 0;
        };
        EXT callExtension ["source:gain", [_id, _gain]];
    }] call CBA_fnc_addEventHandler;

    [FUNC(tick)] call CBA_fnc_addPerFrameHandler;
    [FUNC(heartbeat), 0.75] call CBA_fnc_addPerFrameHandler;

    {
        private _active = _x getVariable [QGVAR(active), []];
        if (_active isNotEqualTo []) then {
            [QGVAR(start), [_active#0, _active#1, _x]] call CBA_fnc_localEvent;
        };
    } forEach allMissionObjects "";
};

addMissionEventHandler ["ExtensionCallback", {
    params ["_name", "_function", "_data"];

    private _lname = toLower _name;
    if (_lname isEqualTo "live_radio_log") exitWith {
        LOG_SYS(_function,_data);
    };
    if (_lname isNotEqualTo "live_radio") exitWith {};
    switch (_function) do {
        case "title": {
            (parseSimpleArray _data) params ["_id", "_title"];
            GVAR(sourcesTitles) set [_id, _title];
            [QGVAR(metadataUpdated), [_id, _title]] call CBA_fnc_localEvent;
        };
        case "album_art": {
            (parseSimpleArray _data) params ["_id", "_path"];
            GVAR(sourcesAlbumArt) set [_id, _path];
            [QGVAR(albumArtUpdated), [_id, _path]] call CBA_fnc_localEvent;
        };
    };
}];
