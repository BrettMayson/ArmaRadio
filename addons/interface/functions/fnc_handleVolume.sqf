#include "script_component.hpp"
/*
 * Author: mharis001
 * Handles updating the volume bar and icon to reflect the given volume level.
 *
 * Arguments:
 * 0: Display <DISPLAY>
 * 1: Volume <NUMBER>
 *
 * Return Value:
 * None
 *
 * Example:
 * [DISPLAY, 0.5] call live_radiointerface_fnc_handleVolume
 *
 * Public: No
 */

params ["_display", "_volume"];

// Convert volume to percentage of max
_volume = _volume / MAX_VOLUME;

// Update the volume icon
private _icon = switch (true) do {
    case (_volume >= 2/3): {
        ICON_VOLUME_HIGH
    };
    case (_volume >= 1/3): {
        ICON_VOLUME_MEDIUM
    };
    default {
        ICON_VOLUME_LOW
    };
};

private _ctrlVolumeIcon = _display displayCtrl IDC_VOLUME_ICON;
_ctrlVolumeIcon ctrlSetText _icon;

// Update the volume bar, setting the entire position instead of just the width
// in order to prevent issues with movingEnable = 1 config entry
private _ctrlVolumeBarMouse = _display displayCtrl IDC_VOLUME_BAR_MOUSE;
private _position = ctrlPosition _ctrlVolumeBarMouse;
_position set [2, _volume * (_position select 2)];

private _ctrlVolumeBarFill = _display displayCtrl IDC_VOLUME_BAR_FILL;
_ctrlVolumeBarFill ctrlSetPosition _position;
_ctrlVolumeBarFill ctrlCommit 0;
