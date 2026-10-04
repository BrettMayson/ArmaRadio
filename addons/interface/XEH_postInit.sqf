#include "script_component.hpp"

if (!hasInterface) exitWith {};
if (isClass (configFile >> "CfgPatches" >> "ace_interact_menu")) then {
    private _inside = [QGVAR(openInside), LLSTRING(DisplayName), "",
        {[_target] call FUNC(open)},
        {[_target] call FUNC(canOpen)},
        {}, [], [0, 0, 0], 10, [false, true, false, false, true]
    ] call ace_interact_menu_fnc_createAction;
    private _outside = [QGVAR(openOutside), LLSTRING(DisplayName), "",
        {[_target] call FUNC(open)},
        {[_target, true] call FUNC(canOpen)},
        {}, [], [0, 0, 0], 5
    ] call ace_interact_menu_fnc_createAction;
    ["Car", "Air", "Ship"] apply {
        [_x, 1, ["ACE_SelfActions"], _inside, true] call ace_interact_menu_fnc_addActionToClass;
        [_x, 0, ["ACE_MainActions"], _outside, true] call ace_interact_menu_fnc_addActionToClass;
    };
} else {
    [[
        LLSTRING(DisplayName),
        {
            [vehicle (call CBA_fnc_currentUnit)] call FUNC(open)
        },
        "", 1, true, true, "",
        QUOTE([vehicle (call CBA_fnc_currentUnit)] call FUNC(canOpen)),
        5
    ]] call CBA_fnc_addPlayerAction;
    [[
        LLSTRING(DisplayName),
        {
            [cursorObject] call FUNC(open)
        },
        "", 1, true, true, "",
        QUOTE([ARR_2(cursorObject,true)] call FUNC(canOpen)),
        5
    ]] call CBA_fnc_addPlayerAction;
};

[QEGVAR(manager,metadataUpdated), {
    [uiNamespace getVariable [QGVAR(display), displayNull]] call FUNC(updateInfo);
}] call CBA_fnc_addEventHandler;

[QEGVAR(manager,streamerModeChanged), {
    [uiNamespace getVariable [QGVAR(display), displayNull]] call FUNC(updateList);
}] call CBA_fnc_addEventHandler;

[QEGVAR(manager,albumArtUpdated), {
    [uiNamespace getVariable [QGVAR(display), displayNull]] call FUNC(updateInfo);
}] call CBA_fnc_addEventHandler;

[QGVAR(radioEnabled), {call FUNC(registerRadio)}] call CBA_fnc_addEventHandler;

[QGVAR(updateInfo), {
    private _display = uiNamespace getVariable [QGVAR(display), displayNull];
    if (!isNull _display) then {
        [_display] call FUNC(updateInfo);
    };
}] call CBA_fnc_addEventHandler;

[QEGVAR(manager,volume), { 
    private _display = uiNamespace getVariable [QGVAR(display), displayNull];
    if (isNull _display) exitWith {};
    private _object = _display getVariable [QGVAR(object), objNull];

    params ["", "_value", "_source"];

    if (isNull _object || _object != _source) exitWith {};
    [_display, _value] call live_radio_interface_fnc_handleVolume;
}] call CBA_fnc_addEventHandler;

[QEGVAR(manager,stop), {
    private _display = uiNamespace getVariable [QGVAR(display), displayNull];
    if (isNull _display) exitWith {};
    private _object = _display getVariable [QGVAR(object), objNull];

    params ["", "_value", "_source"];
    if (_value != "") exitWith {}; // We're about to start playing a different station anyway

    if (isNull _object || _object != _source) exitWith {};
    private _ctrl = _display displayCtrl IDC_POWER;
    _display setVariable [QGVAR(power), false];
    [_ctrl, false] call live_radio_interface_fnc_handlePower;
}] call CBA_fnc_addEventHandler;

[QEGVAR(manager,start), {
    private _display = uiNamespace getVariable [QGVAR(display), displayNull];
    if (isNull _display) exitWith {};
    private _object = _display getVariable [QGVAR(object), objNull];

    params ["", "_value", "_source"];

    if (isNull _object || _object != _source) exitWith {};
    private _ctrl = _display displayCtrl IDC_POWER;
    _display setVariable [QGVAR(power), true];
    [_ctrl, false] call live_radio_interface_fnc_handlePower;
}] call CBA_fnc_addEventHandler;
