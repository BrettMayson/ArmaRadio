#include "script_component.hpp"
/*
 * Author: mharis001
 * Handles selecting an entry in the radio stations list.
 *
 * Arguments:
 * 0: List <CONTROL>
 * 1: Index <NUMBER>
 *
 * Return Value:
 * None
 *
 * Example:
 * [CONTROL, 0] call live_radiointerface_fnc_handleListSelect
 *
 * Public: No
 */

params ["_ctrlList", "_index"];

// Exit if the list is currently locked
// List selection changed due to clearing/adding entries or using lbSetCurSel command
if (_ctrlList getVariable [QGVAR(locked), false]) exitWith {};

// Update the display with the station's information
// Delayed to allow the metadata to be received by the extension
private _display = ctrlParent _ctrlList;
[{
    _this call FUNC(updateInfo);
}, [_display], 0.3] call CBA_fnc_waitAndExecute;

// Change the station if the radio is powered on
if (_display getVariable QGVAR(powered)) then {
    private _url = (_ctrlList getVariable str _index) param [2, ""];
    private _object = _display getVariable QGVAR(object);
    [_object, _url] call EFUNC(manager,play);
};
