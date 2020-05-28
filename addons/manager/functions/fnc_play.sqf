#include "script_component.hpp"

params ["_url", "_source"];

private _id = EXT callExtension "id";

private _jip = [QGVAR(start), [_url, _id, _source]] call CBA_fnc_globalEventJIP;

GVAR(jips) pushBack [_id, _jip];
publicVariable QGVAR(jips);
