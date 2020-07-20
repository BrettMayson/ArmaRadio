#include "script_component.hpp"

[QGVAR(start), {
	params ["_url", "_id", "_source"];
	if !(_id in GVAR(jips)) exitWith {};
	EXT callExtension ["create", [_url, _id]];
	GVAR(active) setVariable [_id, _source];
}] call CBA_fnc_addEventHandler;

[QGVAR(stop), {
	params ["_id"];
	EXT callExtension ["destroy", [_id]];
	GVAR(active) setVariable [_id, nil];
	GVAR(jips) = GVAR(jips) - [_id];
}] call CBA_fnc_addEventHandler;

[FUNC(tick)] call CBA_fnc_addPerFrameHandler;
[FUNC(heartbeat), 0.75] call CBA_fnc_addPerFrameHandler;
