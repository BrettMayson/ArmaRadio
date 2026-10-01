#include "script_component.hpp"
// Author: Joncantplay
/* The policy is checked on the listening client, never supplied by the sender. */
params [["_url", "", [""]]];

if (_url == "") exitWith {false};
if !(missionNamespace getVariable [QGVAR(streamerMode), false]) exitWith {true};

GVAR(stationPolicy) getOrDefault [_url, false]
