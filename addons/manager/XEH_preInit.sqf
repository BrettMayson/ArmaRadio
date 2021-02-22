#include "script_component.hpp"
ADDON = false;
#include "XEH_PREP.hpp"
ADDON = true;

GVAR(sources) = createHashMap;

// Make sure the extension has been loaded once
EXT callExtension ""; 

[
    QGVAR(volumeMultiplier),
	"SLIDER",
	"Volume Multiplier",
	"Live Radio",
	[0, 1, 0.5, 2, true],
	0,
	{
		EXT callExtension ["gain_multiplier", [_this]];
		private _sources = keys GVAR(sources);
		{
			// Reset the volume to apply the multiplier
			EXT callExtension ["gain", [_x, (GVAR(sources) getOrDefault [_x, objNull]) getVariable [QGVAR(volume), _gain, true]]];
		} forEach _sources;
	}
] call CBA_fnc_addSetting;