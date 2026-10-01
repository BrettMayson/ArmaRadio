#include "script_component.hpp"
// Author: Joncantplay
params ["_control"];
private _display = ctrlParent _control;
private _object = _display getVariable [QGVAR(object), objNull];
if (isNull _object) exitWith {};
// Keep this mute local to the player.
_object setVariable [QEGVAR(manager,locallyMuted), !(_object getVariable [QEGVAR(manager,locallyMuted), false])];
call EFUNC(manager,refreshPlayback);
[_display] call FUNC(updateList);
