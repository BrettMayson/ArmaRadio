#include "script_component.hpp"

ADDON = false;

#include "XEH_PREP.hpp"

ADDON = true;

GVAR(stations) = [];

{
    private _stations = configProperties [_x >> "CfgRadioStations", "isClass _x"] apply {
        [getText (_x >> "name"), getText (_x >> "description"), getText (_x >> "picture"), getText (_x >> "url")]
    };

    GVAR(stations) append _stations;
} forEach [configFile, campaignConfigFile, missionConfigFile];

GVAR(stations) sort true;
