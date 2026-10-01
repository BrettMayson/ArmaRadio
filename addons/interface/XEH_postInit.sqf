#include "script_component.hpp"

if (!hasInterface) exitWith {};
if (isClass (configFile >> "CfgPatches" >> "ace_interact_menu")) then {
    {
        private _inside = [QGVAR(openInside), localize "STR_Live_Radio_Interface_DisplayName", "",
            {[_target] call FUNC(open)},
            {alive _target && {vehicle _player == _target} && {[_target] call FUNC(canOpen)}},
            {}, [], [0, 0, 0], 10, [false, true, false, false, true]
        ] call ace_interact_menu_fnc_createAction;
        // Mounted ACE opens this branch directly. An empty parent path places
        // the action alongside the branch and makes it invisible inside.
        [_x, 1, ["ACE_SelfActions"], _inside, true] call ace_interact_menu_fnc_addActionToClass;
        private _outside = [QGVAR(openOutside), localize "STR_Live_Radio_Interface_DisplayName", "",
            {[_target] call FUNC(open)},
            {alive _target && {isNull objectParent _player} && {[_target] call FUNC(canOpen)}},
            {}, [], [0, 0, 0], 5
        ] call ace_interact_menu_fnc_createAction;
        [_x, 0, ["ACE_MainActions"], _outside, true] call ace_interact_menu_fnc_addActionToClass;
    } forEach ["Car", "Air", "Ship"];
} else {
    [[
        "FM Radio",
        {
            [cursorTarget] call FUNC(open)
        },
        "", 1, true, true, "",
        QUOTE(alive cursorTarget && {isNull objectParent player} && {cursorTarget isKindOf ""Land_FMradio_F"" || {cursorTarget isKindOf ""Car""} || {cursorTarget isKindOf ""Air""} || {cursorTarget isKindOf ""Ship""}} && {[cursorTarget] call FUNC(canOpen)}),
        5
    ]] call CBA_fnc_addPlayerAction;
    [[
        "FM Radio",
        {
            [vehicle (call CBA_fnc_currentUnit)] call FUNC(open)
        },
        "", 1, true, true, "",
        QUOTE((vehicle player isKindOf ""Car"" || {vehicle player isKindOf ""Air""} || {vehicle player isKindOf ""Ship""}) && {alive vehicle player} && {[vehicle player] call FUNC(canOpen)}),
        5
    ]] call CBA_fnc_addPlayerAction;
};

[{
    private _display = uiNamespace getVariable [QGVAR(display), displayNull];
    if (!isNull _display) then {
        private _object = _display getVariable [QGVAR(object), objNull];
        if !([_object] call FUNC(canOpen)) exitWith {_display closeDisplay 2};
        private _active = _object getVariable [QEGVAR(manager,active), []];
        if (_active isNotEqualTo (_display getVariable [QGVAR(lastActive), []])) then {
            _display setVariable [QGVAR(lastActive), +_active];
            [_display] call FUNC(updateList);
        };
        [_display] call FUNC(updateStatus);
    };
}, 0.5] call CBA_fnc_addPerFrameHandler;

[QEGVAR(manager,metadataUpdated), {
    [uiNamespace getVariable [QGVAR(display), displayNull]] call FUNC(updateInfo);
}] call CBA_fnc_addEventHandler;

[QEGVAR(manager,streamerModeChanged), {
    [uiNamespace getVariable [QGVAR(display), displayNull]] call FUNC(updateList);
}] call CBA_fnc_addEventHandler;

[QEGVAR(manager,albumArtUpdated), {
    [uiNamespace getVariable QGVAR(display)] call FUNC(updateInfo);
}] call CBA_fnc_addEventHandler;

[QEGVAR(manager,albumArtUpdated), {
    [uiNamespace getVariable [QGVAR(display), displayNull]] call FUNC(updateInfo);
}] call CBA_fnc_addEventHandler;
