#include "script_component.hpp"
/*
 * Author: Brett Mayson,  matidp4
 * Checks if the player can open the interface
 *
 * Arguments:
 * 0: Object <OBJECT>
 * 1: Outside <BOOLEAN>
 *
 * Return Value:
 * BOOLEAN
 *
 * Example:
 * [_object] call live_radio_interface_fnc_canOpen
 *
 * Public: No
 */

params ["_object", "_outside"];

if (!_outside) then {
    if (GVAR(driverAndCommanderOnly)) exitWith {
        private _player = call CBA_fnc_currentUnit;

        (driver _object == _player) ||
        (commander _object == _player)
    };

    true
} else {
    if (!GVAR(interactOutsideVehicle)) exitWith {false};
    if (!isNull driver _object) exitWith {false};

    true
};
