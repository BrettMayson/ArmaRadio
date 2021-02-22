#include "script_component.hpp"

params ["_source", "_gain"];

private _sources = (keys GVAR(sources));
private _index = _sources findIf { (GVAR(sources) get _x) isEqualTo _source };
if (_index == -1) exitWith {};

private _id = _sources select _index;

_source setVariable [QGVAR(volume), _gain, true];

EXT callExtension ["gain", [_id, _gain]];
