#include "script_component.hpp"

params ["_source", "_gain"];

private _sources = (allVariables GVAR(active));
private _id = _sources select (_sources findIf { (GVAR(active) getVariable _x) isEqualTo _source });

EXT callExtension ["gain", [_id, _gain]];
