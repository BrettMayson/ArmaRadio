#include "script_component.hpp"

/*
 * Author: BrettMayson, matidp4, Joncantplay
 *
 * Checks whether the player can open the interface.
 *
 * Arguments:
 * 0: Unit - The player unit attempting to access the object
 * 1: Object - Object being accessed
 * 2: Boolean - Whether the object is being accessed from outside a vehicle
 *
 * Return Value:
 * Boolean
 *
 * Public: No
 */

params [
    ["_object", objNull, [objNull]],
    ["_outside", false, [true]]
];

private _unit = call CBA_fnc_currentUnit;

if (!_outside && { isNull objectParent _unit }) exitWith { false };

if (
    isNull _unit
    || {!alive _unit}
    || {isNull _object}
    || {!alive _object}
    || {!([_object] call FUNC(hasRadio))}
) exitWith { false };

if (_outside) exitWith {
    if (_object isKindOf "Static" || _object isKindOf "Thing") exitWith { true };

    if (GVAR(interactOutsideVehicle) && GVAR(driverAndCommanderOnly)) exitWith {
        (isNull driver _object) && (isNull commander _object)
    };

    GVAR(interactOutsideVehicle)
};

!GVAR(driverAndCommanderOnly)
|| {driver _object == _unit}
|| {commander _object == _unit}
