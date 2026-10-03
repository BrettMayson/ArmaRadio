#include "script_component.hpp"

ADDON = false;

#include "XEH_PREP.hpp"

ADDON = true;

GVAR(stations) = [];
GVAR(personalStationRows) = [];
GVAR(globalStationRows) = [];
call FUNC(reloadStations);

[QGVAR(hideDefaultStations), "CHECKBOX",
    [LSTRING(HideDefaultStations), LSTRING(HideDefaultStationsTooltip)],
    "Live Radio", false, 1, {
        call FUNC(reloadStations);
    }
] call CBA_fnc_addSetting;

// Global setting: CBA synchronizes the server/mission value, including JIP.
[QGVAR(globalStations), "EDITBOX",
    [LSTRING(GlobalStations), LSTRING(GlobalStationsTooltip)],
    "Live Radio", "[]", 1, {
        call FUNC(reloadStations);
    }
] call CBA_fnc_addSetting;

[QGVAR(personalStations), "EDITBOX",
    [LSTRING(PersonalStations), LSTRING(PersonalStationsTooltip)],
    "Live Radio", "[]", 2, {
        call FUNC(reloadStations);
    }
] call CBA_fnc_addSetting;

[
    QGVAR(driverAndCommanderOnly),
    "CHECKBOX",
    LSTRING(DriverCommanderOnly),
    "Live Radio",
    false,
    1
] call CBA_fnc_addSetting;

[QGVAR(interactOutsideVehicle), "CHECKBOX", [LSTRING(InteractOutsideVehicle), LSTRING(InteractOutsideVehicleTooltip)], "Live Radio", false, 1] call CBA_fnc_addSetting;

[QGVAR(interfaceStyle), "LIST",
    [LSTRING(InterfaceStyle), LSTRING(InterfaceStyleTooltip)],
    "Live Radio", [[0, 1], [LSTRING(StyleModern), LSTRING(StyleClassic)], 0], 2
] call CBA_fnc_addSetting;

// Context actions operate on the radio currently open in either interface style.
["Live Radio", QGVAR(toggleStreamerKey), LSTRING(ToggleStreamer),
    {["streamer"] call FUNC(handleShortcut)}, {}, [-1, [false, false, false]]
] call CBA_fnc_addKeybind;
["Live Radio", QGVAR(powerKey), LSTRING(TogglePower),
    {["power"] call FUNC(handleShortcut)}, {}, [-1, [false, false, false]]
] call CBA_fnc_addKeybind;
["Live Radio", QGVAR(retryKey), LSTRING(Retry),
    {["retry"] call FUNC(handleShortcut)}, {}, [-1, [false, false, false]]
] call CBA_fnc_addKeybind;
["Live Radio", QGVAR(copyTitleKey), LSTRING(CopyTitle),
    {["copy"] call FUNC(handleShortcut)}, {}, [-1, [false, false, false]]
] call CBA_fnc_addKeybind;
