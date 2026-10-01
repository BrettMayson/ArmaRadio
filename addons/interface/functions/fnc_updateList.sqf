#include "script_component.hpp"
/*
 * Edited by: Joncantplay
 * Author: mharis001
 * Updates/refreshes the stations list.
 *
 * Arguments:
 * 0: Display <DISPLAY>
 *
 * Return Value:
 * None
 *
 * Example:
 * [DISPLAY] call live_radiointerface_fnc_updateList
 *
 * Public: No
 */

params [["_display", displayNull]];
if (isNull _display) exitWith {};

// Get the currently playing station
private _object = _display getVariable QGVAR(object);
private _activeURL = _object getVariable [QEGVAR(manager,active), []] param [1, ""];

// Get the current search filter
private _ctrlSearchBar = _display displayCtrl IDC_SEARCH_BAR;
private _filter = toLower ctrlText _ctrlSearchBar;

// Prevent lbSetCurSel from trigger the LBSelChanged event
private _ctrlList = _display displayCtrl IDC_LIST;
private _selectedURL = (_ctrlList getVariable [str lbCurSel _ctrlList, []]) param [2, ""];
private _restoreURL = [_selectedURL, _activeURL] select (_activeURL != "");
_ctrlList setVariable [QGVAR(locked), true];

// Clear the list, manually setting selection so info controls
// are properly updated if no station is active
lbClear _ctrlList;
_ctrlList lbSetCurSel -1;

{
    _x params ["_name", "_picture", "_url", "_condition", "_noCopyright"];
    if (call _condition isEqualTo false) then {continue};
    if (GVAR(copyrightFreeOnly) && {!_noCopyright}) then {continue};

    private _isActive = _url isEqualTo _activeURL;

    // Add currently playing station regardless of filter
    if (_isActive || {_filter in toLower _name}) then {
        private _label = _name;
        private _approved = EGVAR(manager,stationPolicy) getOrDefault [_url, false];
        private _muted = ([_object] call EFUNC(manager,isMuted)) || {!_approved && {missionNamespace getVariable [QEGVAR(manager,streamerMode), false]}};
        if (_muted) then {
            _label = format ["[Muted] %1", _name];
        };
        private _index = _ctrlList lbAdd _label;
        _ctrlList lbSetTooltip [_index, _label];
        _ctrlList lbSetColor [_index, [[0.92, 0.94, 0.97, 1], [0.65, 0.69, 0.74, 1]] select _muted];
        _ctrlList setVariable [str _index, [_name, _picture, _url]];

        if (_url isEqualTo _restoreURL) then {
            _ctrlList lbSetCurSel _index;
        };
    };
} forEach GVAR(stations);

_ctrlList setVariable [QGVAR(locked), false];

(_display displayCtrl IDC_EMPTY) ctrlShow (lbSize _ctrlList == 0);
// Refresh the station info controls
[_display] call FUNC(updateInfo);

[_display] call FUNC(updateStreamer);
