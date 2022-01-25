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
		EXT callExtension ["listener:gain", [_this]];
		// [{
			private _sources = keys GVAR(sources);
			{
				// Reset the volume to apply the multiplier
				EXT callExtension ["source:gain", [_x, (GVAR(sources) getOrDefault [_x, objNull]) getVariable [QGVAR(volume), 0.5]]];
			} forEach _sources;
		// }, [], 0.1] call CBA_fnc_waitAndExecute;
	}
] call CBA_fnc_addSetting;
