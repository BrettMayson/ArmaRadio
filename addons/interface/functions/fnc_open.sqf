#include "script_component.hpp"
/*
 * Edited by: Joncantplay
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
 * [_object] call live_radiointerface_fnc_open
 *
 * Public: No
 */

params ["_object"];
if !([_object] call FUNC(canOpen)) exitWith {};

private _dialog = [QGVAR(display), QGVAR(displayClassic)] select (
    missionNamespace getVariable [QGVAR(interfaceStyle), 0] == 1
);
if (!createDialog _dialog) exitWith {};

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
private _ctrlStreamer = _display displayCtrl IDC_STREAMER_TOGGLE;
_ctrlStreamer ctrlAddEventHandler ["ButtonClick", {call FUNC(handleStreamer)}];
private _ctrlCopyTitle = _display displayCtrl IDC_COPY_TITLE;
_ctrlCopyTitle ctrlAddEventHandler ["MouseButtonClick", {call FUNC(handleCopyTitle)}];
(_display displayCtrl IDC_MUTE) ctrlAddEventHandler ["ButtonClick", {call FUNC(handleMute)}];
(_display displayCtrl IDC_RETRY) ctrlAddEventHandler ["ButtonClick", {
    params ["_control"];
    [(ctrlParent _control) getVariable QGVAR(object)] call EFUNC(manager,retry);
    [ctrlParent _control] call FUNC(updateStatus);
}];
[_display] call FUNC(updateStreamer);
[_display] call FUNC(updateStatus);

(_display displayCtrl IDC_PRESETS) ctrlAddEventHandler ["ButtonClick", {call FUNC(openPresets)}];
