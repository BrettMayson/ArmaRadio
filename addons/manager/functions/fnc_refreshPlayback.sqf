#include "script_component.hpp"
// Author: Joncantplay
/* Re-evaluate existing radios immediately when the local setting changes. */
if (!hasInterface) exitWith {};

{
    [_x] call FUNC(syncPlayback);
} forEach (keys GVAR(sources));
