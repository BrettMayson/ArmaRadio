#include "script_component.hpp"
/*
 * Authors: BrettMayson, matidp4
 * Edited by: Joncantplay
 * Check supported objects, outside access and driver/commander permissions.
 */
params ["_object", ["_outside", isNull objectParent (call CBA_fnc_currentUnit)]];
private _player = call CBA_fnc_currentUnit;
if (!alive _object || {!([_object] call FUNC(isSupported))}) exitWith {false};
private _vehicle = _object isKindOf "LandVehicle" || {_object isKindOf "Air"} || {_object isKindOf "Ship"};
if (!_vehicle) exitWith {isNull objectParent _player && {_player distance _object <= 5}};
if (_outside) exitWith {
    !GVAR(driverAndCommanderOnly) && {isNull driver _object} && {GVAR(interactOutsideVehicle)} && {_player distance _object <= 5}
};
if (vehicle _player != _object) exitWith {false};
!GVAR(driverAndCommanderOnly) || {driver _object == _player} || {commander _object == _player}
