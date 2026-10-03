#include "script_component.hpp"
// Author: Joncantplay
params ["_source"];
private _active = _source getVariable [QGVAR(active), []];
if (count _active < 2) exitWith {};
_active params ["_id", "_url"];
if ([_source] call FUNC(isMuted) || {!([_url] call FUNC(canHear))}) exitWith {};
private _status = GVAR(status) getOrDefault [_id, ["", -10]];
if (_status#0 == "loading" && {diag_tickTime - (_status#1) < 10}) exitWith {};
[_id] call FUNC(destroyLocal);
GVAR(sourcesTitles) deleteAt _id;
[_id] call FUNC(syncPlayback);
