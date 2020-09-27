#include "script_component.hpp"

if (hasInterface) then {
	[QGVAR(start), {
		params ["_url", "_id", "_source"];
		if !(_id in GVAR(jips)) exitWith {};
		EXT callExtension ["create", [_url, _id, _source getVariable [QGVAR(volume), 1]]];
		GVAR(active) setVariable [_id, _source];
	}] call CBA_fnc_addEventHandler;

	[QGVAR(stop), {
		params ["_id"];
		EXT callExtension ["destroy", [_id]];
		GVAR(active) setVariable [_id, nil];
		GVAR(jips) = GVAR(jips) - [_id];
		[_id] call CBA_fnc_removeGlobalEventJIP;
	}] call CBA_fnc_addEventHandler;

	[QGVAR(volume), {
		params ["_target", "_gain"];
		[_target, _value] call FUNC(volume);
	}] call CBA_fnc_addEventHandler;

	[FUNC(tick)] call CBA_fnc_addPerFrameHandler;
	[FUNC(heartbeat), 0.75] call CBA_fnc_addPerFrameHandler;
};

addMissionEventHandler ["ExtensionCallback", {
    params ["_name", "_function", "_data"];

    if ((tolower _name) isEqualTo "arma_radio_log") exitWith {
		LOG_SYS(_function,_data);
	};
	if !((tolower _name) isEqualTo "arma_radio") exitWith {};
	// systemChat format ["%1: %2", _function, _data];
}];
