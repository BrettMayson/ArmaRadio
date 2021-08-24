#include "script_component.hpp"

if (!isClass (configFile >> "CfgPatches" >> "ace_interaction")) then {
	[[
		"FM Radio",
		{
			[cursorTarget] call FUNC(open)
		},
		"", 1, true, true, "",
		'cursorTarget isKindOf "Land_FMradio_F"',
		5
	]] call CBA_fnc_addPlayerAction;
	[[
		"FM Radio",
		{
			[vehicle (call CBA_fnc_currentUnit)] call FUNC(open)
		},
		"", 1, true, true, "",
		'vehicle (call CBA_fnc_currentUnit) isKindOf "Car"',
		5
	]] call CBA_fnc_addPlayerAction;
};

[QEGVAR(manager,metadataUpdated), {
	[uiNamespace getVariable QGVAR(display)] call FUNC(updateInfo);
}] call CBA_fnc_addEventHandler;
