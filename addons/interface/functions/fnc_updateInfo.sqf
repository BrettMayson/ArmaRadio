#include "script_component.hpp"
/*
 * Edited by: Joncantplay
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

params [["_display", displayNull]];
if (isNull _display) exitWith {};

private _object = _display getVariable QGVAR(object);

private _ctrlList = _display displayCtrl IDC_LIST;
(_ctrlList getVariable [str lbCurSel _ctrlList, ["", ""]]) params ["_name", "_picture"];

if (_name == "") then {_name = _object getVariable [QGVAR(activeStationName), ""]};
private _ctrlName = _display displayCtrl IDC_NAME;
_ctrlName ctrlSetText _name;
_ctrlName ctrlSetTooltip _name;

private _ctrlDescription = _display displayCtrl IDC_DESCRIPTION;
private _activeID = _object getVariable [QEGVAR(manager,active), []] param [0, ""];
private _description = EGVAR(manager,sourcesTitles) getOrDefault [_activeID, ""];
private _visibleDescription = [_description, localize "STR_Live_Radio_Interface_NoTrackInfo"] select (_description == "");
_ctrlDescription ctrlSetText _visibleDescription;
_ctrlDescription ctrlSetTooltip _visibleDescription;

private _ctrlCopyTitle = _display displayCtrl IDC_COPY_TITLE;
_ctrlCopyTitle setVariable [QGVAR(copyTitle), _description];
_ctrlCopyTitle ctrlEnable (_description != "");
_ctrlCopyTitle ctrlSetTooltip (["", localize "STR_Live_Radio_Interface_CopyTitleTooltip"] select (_description != ""));

if ((_ctrlList getVariable [str lbCurSel _ctrlList, []] param [2, ""]) == (_object getVariable [QEGVAR(manager,active), []] param [1, ""])) then {
    _picture = EGVAR(manager,sourcesAlbumArt) getOrDefault [_activeID, _picture];
};
private _ctrlPicture = _display displayCtrl IDC_PICTURE;
_ctrlPicture ctrlSetText _picture;

private _ctrlPictureDefault = _display displayCtrl IDC_PICTURE_DEFAULT;
_ctrlPictureDefault ctrlShow (_picture == "");
[_display] call FUNC(updateStatus);
