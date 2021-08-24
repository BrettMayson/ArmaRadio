#include "script_component.hpp"
/*
 * Author: mharis001
 * Updates the station info controls based on the currently selected list item.
 *
 * Arguments:
 * 0: Display <DISPLAY>
 *
 * Return Value:
 * None
 *
 * Example:
 * [DISPLAY] call live_radiointerface_fnc_updateInfo
 *
 * Public: No
 */

params ["_display"];

private _object = _display getVariable QGVAR(object);

private _ctrlList = _display displayCtrl IDC_LIST;
(_ctrlList getVariable str lbCurSel _ctrlList) params ["_name", "_picture"];

private _ctrlName = _display displayCtrl IDC_NAME;
_ctrlName ctrlSetText _name;

private _ctrlDescription = _display displayCtrl IDC_DESCRIPTION;
private _activeID = _object getVariable [QEGVAR(manager,active), []] param [0, ""];
_ctrlDescription ctrlSetText (EGVAR(manager,sourcesTitles) getOrDefault [_activeID, ""]);

private _ctrlPicture = _display displayCtrl IDC_PICTURE;
_ctrlPicture ctrlSetText _picture;

private _ctrlPictureDefault = _display displayCtrl IDC_PICTURE_DEFAULT;
_ctrlPictureDefault ctrlShow (_picture == "");
