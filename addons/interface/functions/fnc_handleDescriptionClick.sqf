#include "script_component.hpp"
/*
 * Author: BrettMayson
 * Handles clicking the currently playing track title, copying it to the clipboard.
 *
 * Arguments:
 * 0: Description Control <CONTROL>
 * 1: Button <NUMBER>
 *
 * Return Value:
 * None
 *
 * Example:
 * [CONTROL, 0] call live_radio_interface_fnc_handleDescriptionClick
 *
 * Public: No
 */

params ["_ctrlDescription", "_button"];

if (_button != 1) exitWith {};

private _title = ctrlText _ctrlDescription;
if (_title == "") exitWith {};

copyToClipboard _title;
hint (LLSTRING(CopiedToClipboard));
