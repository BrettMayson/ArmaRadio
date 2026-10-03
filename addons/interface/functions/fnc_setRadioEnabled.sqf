#include "script_component.hpp"
/*
 * Author: Joncantplay
 * Enable radio interactions on a mission object, including late-joining clients.
 * Example: [this, true] call live_radio_interface_fnc_setRadioEnabled;
 * Public: Yes
 */
params [["_object", objNull, [objNull]], ["_enabled", true, [true]]];
if (isNull _object) exitWith {false};
_object setVariable [QGVAR(enabled), _enabled, true];
[QGVAR(radioEnabled), [_object]] call CBA_fnc_globalEvent;
true
