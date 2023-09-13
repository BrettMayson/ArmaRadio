#include "script_component.hpp"

if (hasInterface) then {
    [QGVAR(start), {
        params ["_id", "_url", "_source"];
        EXT callExtension ["source:new", [_id, _url, _source getVariable [QGVAR(volume), 1]]];
        GVAR(sources) set [_id, _source];
        [QGVAR(metadataUpdated), [_id, ""]] call CBA_fnc_localEvent;
    }] call CBA_fnc_addEventHandler;

    [QGVAR(stop), {
        params ["_id"];
        EXT callExtension ["source:destroy", [_id]];
        GVAR(sources) deleteAt _id;
        GVAR(sourcesTitles) deleteAt _id;
    }] call CBA_fnc_addEventHandler;

    [QGVAR(volume), {
        params ["_id", "_gain"];
        EXT callExtension ["source:gain", [_id, _gain]];
    }] call CBA_fnc_addEventHandler;

    [FUNC(tick)] call CBA_fnc_addPerFrameHandler;
    [FUNC(heartbeat), 0.75] call CBA_fnc_addPerFrameHandler;

    {
        private _active = _x getVariable [QGVAR(active), []];
        if !(_action isEqualTo []) then {
            [QGVAR(start), [_active#0, _active#1, _x]] call CBA_fnc_localEvent;
        };
    } forEach allMissionObjects "";
};

addMissionEventHandler ["ExtensionCallback", {
    params ["_name", "_function", "_data"];

    if ((tolower _name) isEqualTo "live_radio_log") exitWith {
        LOG_SYS(_function,_data);
    };
    if !((tolower _name) isEqualTo "live_radio") exitWith {};
    switch (_function) do {
        case "title": {
            (parseSimpleArray _data) params ["_id", "_title"];
            GVAR(sourcesTitles) set [_id, _title];
            [QGVAR(metadataUpdated), [_id, _title]] call CBA_fnc_localEvent;
        };
    };
}];
