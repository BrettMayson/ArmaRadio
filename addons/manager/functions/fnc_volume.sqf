#include "script_component.hpp"
params ["_source", "_gain"];
_gain = (_gain max 0) min 2;
if (abs ((_source getVariable [QGVAR(volume), 1]) - _gain) < 0.005) exitWith {};
_source setVariable [QGVAR(volume), _gain, true];
private _id = (_source getVariable [QGVAR(active), []]) param [0, ""];
if (_id != "") then {[QGVAR(volume), [_id, _gain]] call CBA_fnc_globalEvent};
