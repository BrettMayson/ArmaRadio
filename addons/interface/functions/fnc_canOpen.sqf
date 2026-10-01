#include "script_component.hpp"
/*
 * Edited by: Joncantplay
 * Author: Brett Mayson,  matidp4
 * Checks if the player can open the interface
 *
 * Arguments:
 * 0: Object <OBJECT>
 * 1: Outside interaction <BOOLEAN> (optional)
 *
 * Return Value:
 * BOOLEAN
 *
 * Example:
 * [_object] call live_radio_interface_fnc_canOpen
 *
 * Public: No
 */

params ["_object", ["_outside", isNull objectParent (call CBA_fnc_currentUnit)]];
private _player = call CBA_fnc_currentUnit;
if (isNull _object || {!alive _object}) exitWith {false};
if (_object isKindOf "Land_FMradio_F") exitWith {true};
if !(_object isKindOf "Car" || {_object isKindOf "Air"} || {_object isKindOf "Ship"}) exitWith {false};
if (_outside) exitWith {
    !GVAR(driverAndCommanderOnly) && {GVAR(interactOutsideVehicle)}
};
if (vehicle _player != _object) exitWith {false};
!GVAR(driverAndCommanderOnly) || {driver _object == _player} || {commander _object == _player}
