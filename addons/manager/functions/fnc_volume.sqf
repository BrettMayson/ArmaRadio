#include "script_component.hpp"

params ["_source", "_gain"];

private _sources = (allVariables GVAR(active));
private _index = _sources findIf { (GVAR(active) getVariable _x) isEqualTo _source };
if (_index == -1) exitWith {};

private _id = _sources select _index;

_source setVariable [QGVAR(volume), _gain, true];

EXT callExtension ["gain", [_id, _gain]];
