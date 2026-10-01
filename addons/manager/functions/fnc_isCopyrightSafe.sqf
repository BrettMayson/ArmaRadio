#include "script_component.hpp"
// Original helper by BrettMayson; uses the shared station policy maintained by Joncantplay.
params ["_url"];
GVAR(stationPolicy) getOrDefault [_url, false]
