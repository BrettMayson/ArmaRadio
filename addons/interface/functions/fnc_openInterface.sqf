#include "script_component.hpp"

params ["_target"];

GVAR(target) = _target;
createDialog "A3R_RscStations";
