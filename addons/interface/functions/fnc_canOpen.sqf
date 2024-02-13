#include "script_component.hpp"
/*
 * Author: Brett Mayson,  matidp4
 * Checks if the player can open the interface
 *
 * Arguments:
 * 0: Object <OBJECT>
 *
 * Return Value:
 * BOOLEAN
 *
 * Example:
 * [_object] call live_radio_interface_fnc_canOpen
 *
 * Public: No
 */

if !(missionNamespace getVariable [QGVAR(driverAndCommanderOnly), false]) exitWith {true};

params ["_object"];
private _player = call CBA_fnc_currentUnit;

if (driver _object == _player) exitWith {true};
if (commander _object == _player) exitWith {true};
false
