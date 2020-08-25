#include "script_component.hpp"
/*
 * Author: mharis001
 * Handles the mouse button down event for the volume bar.
 *
 * Arguments:
 * 0: Volume Bar Mouse <CONTROL>
 * 1: Button <NUMBER>
 *
 * Return Value:
 * None
 *
 * Example:
 * [CONTROL, 0] call radio_interface_fnc_handleVolumeButtonDown
 *
 * Public: No
 */

params ["_ctrlVolumeBarMouse", "_button"];

if (_button == 0) then {
    _ctrlVolumeBarMouse setVariable [QGVAR(moving), true];
};
