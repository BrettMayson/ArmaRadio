#include "script_component.hpp"

[QGVAR(start), {
	params ["_url", "_id", "_source"];
	EXT callExtension ["create", [_url, _id]];
	GVAR(active) setVariable [_id, _source];
}] call CBA_fnc_addEventHandler;

[FUNC(tick), 0.05] call CBA_fnc_addPerFrameHandler;
