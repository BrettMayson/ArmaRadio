#include "script_component.hpp"
// Author: Joncantplay
// Register local ACE actions once. Public object variables handle JIP availability.
params [["_object", objNull, [objNull]]];
if (!hasInterface || {isNull _object} || {_object getVariable [QGVAR(actionsRegistered), false]}) exitWith {};
if (_object isKindOf "Car" || {_object isKindOf "Air"} || {_object isKindOf "Ship"} || {
    _object isKindOf "Land_FMradio_F"
}) exitWith {};
if !(isClass (configFile >> "CfgPatches" >> "ace_interact_menu")) exitWith {};
_object setVariable [QGVAR(actionsRegistered), true];
private _outside = [QGVAR(openOutside), localize "STR_Live_Radio_Interface_DisplayName", "",
    {[_target] call FUNC(open)},
    {isNull objectParent _player && {[_target] call FUNC(canOpen)}},
    {}, [], [0, 0, 0], 5
] call ace_interact_menu_fnc_createAction;
[_object, 0, ["ACE_MainActions"], _outside] call ace_interact_menu_fnc_addActionToObject;
private _inside = [QGVAR(openInside), localize "STR_Live_Radio_Interface_DisplayName", "",
    {[_target] call FUNC(open)},
    {vehicle _player == _target && {[_target] call FUNC(canOpen)}},
    {}, [], [0, 0, 0], 10, [false, true, false, false, true]
] call ace_interact_menu_fnc_createAction;
[_object, 1, ["ACE_SelfActions"], _inside] call ace_interact_menu_fnc_addActionToObject;
