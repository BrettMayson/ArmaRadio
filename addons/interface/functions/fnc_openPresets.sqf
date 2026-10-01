#include "script_component.hpp"
// Author: Joncantplay
if (!hasInterface) exitWith {};
if (!createDialog QGVAR(presetsDisplay)) exitWith {};
["refresh"] call FUNC(handlePresets);
