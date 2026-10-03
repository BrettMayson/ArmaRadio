#include "script_component.hpp"
// Author: Joncantplay
params ["_object"];
(missionNamespace getVariable [QGVAR(muteAll), false]) || {_object getVariable [QGVAR(locallyMuted), false]}
