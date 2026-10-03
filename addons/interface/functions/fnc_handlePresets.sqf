#include "script_component.hpp"
// Author: Joncantplay
params ["_action"];
private _display = uiNamespace getVariable [QGVAR(presetsDisplay), displayNull];
if (isNull _display) exitWith {};
private _list = _display displayCtrl 300;
private _rows = +(missionNamespace getVariable [QGVAR(personalStationRows), []]);
if (_action == "select") exitWith {
    private _row = _rows param [lbCurSel _list, ["", "", false]];
    (_display displayCtrl 301) ctrlSetText (_row#0);
    (_display displayCtrl 302) ctrlSetText (_row#1);
    (_display displayCtrl 303) cbSetChecked (_row#2);
};
if (_action == "new") exitWith {
    _list lbSetCurSel -1;
    (_display displayCtrl 301) ctrlSetText "";
    (_display displayCtrl 302) ctrlSetText "";
    (_display displayCtrl 303) cbSetChecked false;
};
private _changed = false;
if (_action == "save") then {
    private _name = ctrlText (_display displayCtrl 301);
    private _url = ctrlText (_display displayCtrl 302);
    private _lower = toLower _url;
    if (_name == "" || {count _name > 128} || {count _url > 2048} || {
        _lower find "http://" != 0 && {_lower find "https://" != 0}
    }) exitWith {hint localize "STR_Live_Radio_Interface_PresetInvalid"};
    private _row = [_name, _url, cbChecked (_display displayCtrl 303)];
    private _index = lbCurSel _list;
    if (_index < 0) then {
        if (count _rows < 100) then {_rows pushBack _row; _changed = true} else {
            hint localize "STR_Live_Radio_Interface_PresetLimit";
        };
    } else {_rows set [_index, _row]; _changed = true};
};
if (_action == "delete" && {lbCurSel _list >= 0}) then {
    _rows deleteAt lbCurSel _list;
    _changed = true;
};
if (_changed) then {
    [QGVAR(personalStations), str _rows, 0, "client", true] call CBA_settings_fnc_set;
};
_list ctrlRemoveAllEventHandlers "LBSelChanged";
lbClear _list;
{_list lbAdd (_x#0)} forEach (missionNamespace getVariable [QGVAR(personalStationRows), []]);
_list ctrlAddEventHandler ["LBSelChanged", {["select"] call FUNC(handlePresets)}];
if (lbSize _list > 0) then {_list lbSetCurSel 0} else {["new"] call FUNC(handlePresets)};
