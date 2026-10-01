#include "script_component.hpp"

ADDON = false;

#include "XEH_PREP.hpp"

ADDON = true;

GVAR(stations) = [];

{
    private _stations = configProperties [_x >> "CfgRadioStations", "isClass _x"] apply {
        [getText (_x >> "name"), getText (_x >> "picture"), getText (_x >> "url"), compile getText (_x >> "condition"), getNumber (_x >> "noCopyright") == 1]
    };

    GVAR(stations) append _stations;
} forEach [configFile, campaignConfigFile, missionConfigFile];

GVAR(stations) sort true;

[
    QGVAR(driverAndCommanderOnly),
    "CHECKBOX",
    LSTRING(DriverCommanderOnly),
    "Live Radio",
    false,
    1
] call CBA_fnc_addSetting;

[QGVAR(interactOutsideVehicle), "CHECKBOX", [LSTRING(InteractOutsideVehicle), LSTRING(InteractOutsideVehicleTooltip)], "Live Radio", false, 1] call CBA_fnc_addSetting;
[QGVAR(copyrightFreeOnly), "CHECKBOX", LSTRING(CopyrightFreeOnly), "Live Radio", false, 1, {
    [uiNamespace getVariable [QGVAR(display), displayNull]] call FUNC(updateList);
}] call CBA_fnc_addSetting;
