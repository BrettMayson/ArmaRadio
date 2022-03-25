#include "script_component.hpp"

params ["_source", "_url"];

private _ret = "";

private _existing = _source getVariable [QGVAR(active), []];
if !(_existing isEqualTo []) then {
	if ((_existing select 1) isEqualTo _url) then {
		_ret = _existing select 0;
	} else {
		[QGVAR(stop), [_existing select 0]] call CBA_fnc_globalEvent;
	};
};

if !(_ret isEqualTo "") exitWith {};
if (_url isEqualTo "") exitWith {
	_source setVariable [QGVAR(active), nil, true];
};

private _id = EXT callExtension ["id", []] select 0;

_source setVariable [QGVAR(active), [_id, _url], true];

[QGVAR(start), [_id, _url, _source]] call CBA_fnc_globalEvent;

_id
