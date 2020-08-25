#include "script_component.hpp"
/*
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
 * [DISPLAY] call radio_interface_fnc_updateList
 *
 * Public: No
 */

params ["_display"];

// Get the currently playing station
private _object = _display getVariable QGVAR(object);
private _activeURL = _object getVariable [QEGVAR(manager,active), []] param [1, ""];

// Get the current search filter
private _ctrlSearchBar = _display displayCtrl IDC_SEARCH_BAR;
private _filter = toLower ctrlText _ctrlSearchBar;

// Prevent lbSetCurSel from trigger the LBSelChanged event
private _ctrlList = _display displayCtrl IDC_LIST;
_ctrlList setVariable [QGVAR(locked), true];

// Clear the list, manually setting selection so info controls
// are properly updated if no station is active
lbClear _ctrlList;
_ctrlList lbSetCurSel -1;

{
    _x params ["_name", "_description", "_picture", "_url"];

    private _isActive = _url isEqualTo _activeURL;

    // Add currently playing station regardless of filter
    if (_isActive || {_filter in toLower _name}) then {
        private _index = _ctrlList lbAdd _name;
        _ctrlList setVariable [str _index, [_name, _description, _picture, _url]];

        if (_isActive) then {
            _ctrlList lbSetCurSel _index;
        };
    };
} forEach GVAR(stations);

_ctrlList setVariable [QGVAR(locked), false];

// Refresh the station info controls
[_display] call FUNC(updateInfo);
