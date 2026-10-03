#include "script_component.hpp"

ADDON = false;

#include "XEH_PREP.hpp"

ADDON = true;

GVAR(stations) = [];
GVAR(groups) = [];

// Stations are classes with a "url", groups are classes without one that contain nested stations
private _fnc_addStation = {
    params ["_class", "_groupSetting", "_groupName"];

    GVAR(stations) pushBack [
        getText (_class >> "name"),
        getText (_class >> "picture"),
        getText (_class >> "url"),
        compile getText (_class >> "condition"),
        getNumber (_class >> "noCopyright") == 1,
        QGVAR(station_) + configName _class,
        _groupSetting,
        _groupName
    ];
};

{
    private _configFile = _x;

    {
        if (isText (_x >> "url")) then {
            [_x, "", ""] call _fnc_addStation;
        } else {
            private _groupSetting = QGVAR(group_) + configName _x;
            private _groupName = getText (_x >> "name");
            GVAR(groups) pushBackUnique [_groupSetting, _groupName];

            {
                [_x, _groupSetting, _groupName] call _fnc_addStation;
            } forEach configProperties [_x, "isClass _x"];
        };
    } forEach configProperties [_configFile >> "CfgRadioStations", "isClass _x"];
} forEach [configFile, campaignConfigFile, missionConfigFile];

GVAR(stations) sort true;

{
    _x params ["_groupSetting", "_groupName"];

    [
        _groupSetting,
        "CHECKBOX",
        _groupName,
        "Live Radio Stations",
        true,
        1
    ] call CBA_fnc_addSetting;
} forEach GVAR(groups);

{
    _x params ["_name", "", "", "", "", "_settingName", "_groupSetting", "_groupName"];

    private _category = if (_groupSetting == "") then {
        "Live Radio Stations"
    } else {
        ["Live Radio Stations", _groupName]
    };

    [
        _settingName,
        "CHECKBOX",
        _name,
        _category,
        true,
        1
    ] call CBA_fnc_addSetting;
} forEach GVAR(stations);


[
    QGVAR(driverAndCommanderOnly),
    "CHECKBOX",
    LSTRING(DriverCommanderOnly),
    "Live Radio",
    false,
    1
] call CBA_fnc_addSetting;

[
    QGVAR(interactOutsideVehicle),
    "CHECKBOX",
    LSTRING(InteractOutsideVehicle),
    "Live Radio",
    false,
    1
] call CBA_fnc_addSetting;

[
    QGVAR(copyrightFreeOnly),
    "CHECKBOX",
    LSTRING(CopyrightFreeOnly),
    "Live Radio",
    false,
    1
] call CBA_fnc_addSetting;
