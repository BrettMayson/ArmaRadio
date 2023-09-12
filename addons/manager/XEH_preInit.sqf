#include "script_component.hpp"
ADDON = false;
#include "XEH_PREP.hpp"
ADDON = true;

GVAR(sources) = createHashMap;
GVAR(sourcesTitles) = createHashMap;

// Make sure the extension has been loaded once
EXT callExtension "";

[
    QGVAR(volumeMultiplier),
    "SLIDER",
    "Volume Multiplier",
    "Live Radio",
    [0.1, 1, 0.5, 2, true],
    0,
    {
        EXT callExtension ["source:global_gain", [_this]];
    }
] call CBA_fnc_addSetting;
