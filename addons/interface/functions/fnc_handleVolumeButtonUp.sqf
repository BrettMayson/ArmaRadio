#include "script_component.hpp"
/*
 * Edited by: Joncantplay
 * Author: mharis001
 * Handles the mouse button up event for the volume bar.
 *
 * Arguments:
 * 0: Volume Bar Mouse <CONTROL>
 * 1: Button <NUMBER>
 *
 * Return Value:
 * None
 *
 * Example:
 * [CONTROL, 0] call live_radiointerface_fnc_handleVolumeButtonUp
 *
 * Public: No
 */

params ["_ctrlVolumeBarMouse", "_button"];
if !([(ctrlParent _ctrlVolumeBarMouse) getVariable QGVAR(object)] call FUNC(canOpen)) exitWith {};

if (_button == 0) then {
    _ctrlVolumeBarMouse setVariable [QGVAR(moving), false];
    private _volume = _ctrlVolumeBarMouse getVariable [QGVAR(pendingVolume), -1];
    if (_volume >= 0) then {
        [(ctrlParent _ctrlVolumeBarMouse) getVariable QGVAR(object), _volume] call EFUNC(manager,volume);
    };
};
