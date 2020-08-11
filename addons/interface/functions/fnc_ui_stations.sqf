#include "script_component.hpp"
/*
 * Arguments:
 * 0: stations controls group <CONTROL>
 *
 * Return Value:
 * None
 *
 * Example:
 * [CONTROL] call a3r_interface_stations;
 *
 * Public: No
 */

params ["_display"];

_display displayAddEventHandler ["Unload", {
    params ["_display"];
    private _slider = _display displayCtrl 13589;
    _control ctrlRemoveAllEventHandlers "SliderPosChanged"; 
}];

//Generic Init:
private _ctrlButtonOK = _display displayCtrl 1; //IDC_OK

//Specific on-load stuff:
private _slider = _display displayCtrl 13589;
_slider sliderSetRange [0, 2];
_slider sliderSetPosition (GVAR(target) getVariable [QEGVAR(manager,volume), 1]);
_slider ctrlAddEventHandler ["SliderPosChanged", {
    params ["_control", "_value"];
    GVAR(target) setVariable [QEGVAR(manager,volume), _value];
    [QEGVAR(manager,volume), [GVAR(target), _value]] call CBA_fnc_globalEvent;
}];

private _listbox = _display displayCtrl 16189;
private _selected = 0;

private _existing = (GVAR(target)) getVariable [QEGVAR(manager,active), ["", ""]] select 1;

_listbox lbAdd "Turn Off";
{
    private _index = _listbox lbAdd (_x select 0);
	_listbox lbSetData [_index, _x select 1];
    if ((_x select 1) isEqualTo _existing) then {
        _selected = _index;
    }
} forEach GVAR(stations);

_listbox lbSetCurSel _selected;

private _fnc_onConfirm = {
    params [["_ctrlButtonOK", controlNull, [controlNull]]];

    private _display = ctrlparent _ctrlButtonOK;
    if (isNull _display) exitWith {};

    private _lb = _display displayCtrl 16189;

    private _url = _lb lbData (lbCurSel _lb);
    [GVAR(target), _url] call EFUNC(manager,play);
};

_ctrlButtonOK ctrlAddEventHandler ["buttonclick", _fnc_onConfirm];
