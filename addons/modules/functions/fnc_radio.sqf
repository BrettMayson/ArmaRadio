#include "..\script_component.hpp"
/*
 * Author: Joncantplay
 *
 * Handles the radio module functionality.
 *
 * Arguments:
 * 0: Logic - The radio module logic object
 * 1: Array - Units affected by the radio module
 * 2: Boolean - Whether the module is activated
 *
 * Return Value:
 * Boolean
 *
 * Public: No
 */
params ["_logic", ["_units", []], ["_activated", true]];

if (!_activated || {!local _logic}) exitWith {};

private _curatorPlaced = _logic getVariable ["BIS_fnc_moduleInit_isCuratorPlaced", false];
private _targets = +_units;
_targets append synchronizedObjects _logic;

private _attached = attachedTo _logic;

if (isNull _attached) then {
    _attached = _logic getVariable ["bis_fnc_curatorAttachObject_object", objNull];
};
if (!isNull _attached) then {_targets pushBackUnique _attached};
if (_curatorPlaced) then { deleteVehicle _logic };

_targets = _targets select {!isNull _x && {!(_x isKindOf "Logic")} && {!(_x isKindOf "CAManBase")}};
if (_targets isEqualTo []) exitWith {
    WARNING_1("No valid targets for the radio module %1",_logic);
    if (_curatorPlaced) then { 
        [objNull, LLSTRING(Feedback_NothingSelected)] call BIS_fnc_showCuratorFeedbackMessage;
    };
};

{
    [QEGVAR(interface,radioEnabled), [_x]] call CBA_fnc_globalEventJIP;
} forEach (_targets arrayIntersect _targets);
