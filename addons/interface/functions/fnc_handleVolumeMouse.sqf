#include "script_component.hpp"
/*
 * Author: mharis001
 * Handles the mouse moving and holding events for the volume bar.
 *
 * Arguments:
 * 0: Volume Bar Mouse <CONTROL>
 * 1: X Position <NUMBER>
 *
 * Return Value:
 * None
 *
 * Example:
 * [CONTROL, 0] call radio_interface_fnc_handleVolumeMouse
 *
 * Public: No
 */

params ["_ctrlVolumeBarMouse", "_mousePosX"];

if (_ctrlVolumeBarMouse getVariable [QGVAR(moving), false]) then {
    // Convert from mouse position to volume level
    ctrlPosition _ctrlVolumeBarMouse params ["_posX", "", "_posW"];
    private _volume = linearConversion [_posX, _posX + _posW, _mousePosX, MIN_VOLUME, MAX_VOLUME, true];

    // Update the volume bar and icon
    private _display = ctrlParent _ctrlVolumeBarMouse;
    [_display, _volume] call FUNC(handleVolume);

    // Update the radio's volume
    private _object = _display getVariable QGVAR(object);
    [_object, _volume] call EFUNC(manager,volume);
};
