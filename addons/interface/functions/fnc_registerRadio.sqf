#include "script_component.hpp"
/*
 * Author: Joncantplay
 * Registers the radio interface actions for the given object.
 *
 * Arguments:
 * 0: Object - The object for which to register the radio interface actions
 *
 * Return Value:
 * None
 *
 * Example:
 * [_object] call live_radio_interface_fnc_registerRadio
 *
 * Public: No
 */

params [["_object", objNull, [objNull]]];

if (!hasInterface || {isNull _object} || {_object getVariable [QGVAR(actionsRegistered), false]}) exitWith {};

_object setVariable [QGVAR(actionsRegistered), true];
_object setVariable [QGVAR(enabled), true];

if !(isClass (configFile >> "CfgPatches" >> "ace_interact_menu")) exitWith {};

private _outside = [QGVAR(openOutside), LLSTRING(DisplayName), "",
    {[_target] call FUNC(open)},
    {[_target,true] call FUNC(canOpen)},
    {}, [], [0, 0, 0], 5
] call ace_interact_menu_fnc_createAction;
[_object, 0, ["ACE_MainActions"], _outside] call ace_interact_menu_fnc_addActionToObject;

private _inside = [QGVAR(openInside), LLSTRING(DisplayName), "",
    {[_target] call FUNC(open)},
    {[_target] call FUNC(canOpen)},
    {}, [], [0, 0, 0], 10, [false, true, false, false, true]
] call ace_interact_menu_fnc_createAction;
[_object, 1, ["ACE_SelfActions"], _inside] call ace_interact_menu_fnc_addActionToObject;
