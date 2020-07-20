#include "script_component.hpp"
ADDON = false;
#include "XEH_PREP.hpp"
ADDON = true;

GVAR(stations) = [];

{
	GVAR(stations) pushBack [
		getText (_x >> "name"),
		getText (_x >> "url")
	];
} forEach configProperties [configFile >> "CfgRadioStations", "true", true];

{
	GVAR(stations) pushBack [
		getText (_x >> "name"),
		getText (_x >> "url")
	];
} forEach configProperties [missionConfigFile >> "CfgRadioStations", "true", true];
