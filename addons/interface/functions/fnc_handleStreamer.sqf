#include "script_component.hpp"
// Author: Joncantplay
params ["_control"];
EGVAR(manager,streamerMode) = !(missionNamespace getVariable [QEGVAR(manager,streamerMode), false]);
profileNamespace setVariable [QEGVAR(manager,streamerMode), EGVAR(manager,streamerMode)];
saveProfileNamespace;
call EFUNC(manager,refreshPlayback);
[QEGVAR(manager,streamerModeChanged), []] call CBA_fnc_localEvent;
