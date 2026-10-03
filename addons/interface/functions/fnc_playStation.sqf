#include "script_component.hpp"
// Author: Joncantplay
params ["_object", "_url", ["_name", ""]];
if !([_object] call FUNC(canOpen)) exitWith {};
_object setVariable [QGVAR(activeStationName), _name, true];
[_object, _url] call EFUNC(manager,play);
