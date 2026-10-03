#include "script_component.hpp"
// Author: Joncantplay
params ["_logic", ["_units", []], ["_activated", true]];
if (!_activated || {!local _logic}) exitWith {};
private _curatorPlaced = _logic getVariable ["BIS_fnc_moduleInit_isCuratorPlaced", false];
private _targets = +_units;
_targets append synchronizedObjects _logic;
private _attached = attachedTo _logic;
if (isNull _attached) then {_attached = _logic getVariable ["bis_fnc_curatorAttachObject_object", objNull]};
if (!isNull _attached) then {_targets pushBackUnique _attached};
_targets = _targets select {!isNull _x && {!(_x isKindOf "Logic")} && {!(_x isKindOf "CAManBase")}};
if (_targets isEqualTo []) exitWith {
    if (hasInterface) then {hint localize "STR_Live_Radio_Interface_ModuleNoTarget"};
    if (_curatorPlaced) then {deleteVehicle _logic};
};
{
    [_x, true] call FUNC(setRadioEnabled);
} forEach (_targets arrayIntersect _targets);
if (_curatorPlaced) then {deleteVehicle _logic};
