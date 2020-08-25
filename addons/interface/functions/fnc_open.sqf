#include "script_component.hpp"
/*
 * Author: mharis001
 * Opens the radio interface for the given object.
 *
 * Arguments:
 * 0: Object <OBJECT>
 *
 * Return Value:
 * None
 *
 * Example:
 * [_object] call radio_interface_fnc_open
 *
 * Public: No
 */

params ["_object"];

if (!createDialog QGVAR(display)) exitWith {};

private _display = uiNamespace getVariable QGVAR(display);
_display setVariable [QGVAR(object), _object];

// Initialize the stations list
// "-1" setVariable to correctly update station info when nothing is selected
private _ctrlList = _display displayCtrl IDC_LIST;
_ctrlList ctrlAddEventHandler ["LBSelChanged", {call FUNC(handleListSelect)}];
_ctrlList setVariable ["-1", ["", "", "", ""]];

[_display] call FUNC(updateList);

// Initialize the power button
private _ctrlPower = _display displayCtrl IDC_POWER;
_ctrlPower ctrlAddEventHandler ["ButtonClick", {call FUNC(handlePower)}];

private _activeURL = _object getVariable [QEGVAR(manager,active), []] param [1, ""];
_display setVariable [QGVAR(powered), _activeURL != ""];

[_ctrlPower, false] call FUNC(handlePower);

// Initialize the search bar and button
private _ctrlSearchBar = _display displayCtrl IDC_SEARCH_BAR;
_ctrlSearchBar ctrlAddEventHandler ["KeyUp", {call FUNC(handleSearchKeyUp)}];
_ctrlSearchBar ctrlAddEventHandler ["MouseButtonClick", {call FUNC(handleSearchClick)}];

private _ctrlSearchButton = _display displayCtrl IDC_SEARCH_BUTTON;
_ctrlSearchButton ctrlAddEventHandler ["ButtonClick", {call FUNC(handleSearchButton)}];

// Initialize the volume bar and icon
private _ctrlVolumeBarMouse = _display displayCtrl IDC_VOLUME_BAR_MOUSE;
_ctrlVolumeBarMouse ctrlAddEventHandler ["MouseMoving", {call FUNC(handleVolumeMouse)}];
_ctrlVolumeBarMouse ctrlAddEventHandler ["MouseHolding", {call FUNC(handleVolumeMouse)}];
_ctrlVolumeBarMouse ctrlAddEventHandler ["MouseButtonUp", {call FUNC(handleVolumeButtonUp)}];
_ctrlVolumeBarMouse ctrlAddEventHandler ["MouseButtonDown", {call FUNC(handleVolumeButtonDown)}];

private _volume = _object getVariable [QEGVAR(manager,volume), DEFAULT_VOLUME];
[_display, _volume] call FUNC(handleVolume);
