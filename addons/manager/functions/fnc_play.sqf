#include "script_component.hpp"

params ["_url", "_source"];

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
	_source setVariable [QGVAR(active), nil];
};

private _id = EXT callExtension "id";

GVAR(jips) pushBack _id;
publicVariable QGVAR(jips);

_source setVariable [QGVAR(active), [_id, _url]];

[QGVAR(start), [_url, _id, _source], _id] call CBA_fnc_globalEventJIP;

_id
