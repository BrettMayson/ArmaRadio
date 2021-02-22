#include "script_component.hpp"

if (hasInterface) then {
	[QGVAR(start), {
		params ["_url", "_id", "_source"];
		EXT callExtension ["create", [_url, _id, _source getVariable [QGVAR(volume), 1]]];
		GVAR(sources) set [_id, _source];
	}] call CBA_fnc_addEventHandler;

	[QGVAR(stop), {
		params ["_id"];
		EXT callExtension ["destroy", [_id]];
		GVAR(sources) deleteAt _id;
	}] call CBA_fnc_addEventHandler;

	[QGVAR(volume), {
		params ["_target", "_gain"];
		[_target, _value] call FUNC(volume);
	}] call CBA_fnc_addEventHandler;

	[FUNC(tick)] call CBA_fnc_addPerFrameHandler;
	[FUNC(heartbeat), 0.75] call CBA_fnc_addPerFrameHandler;

	{
		private _active = _x getVariable [QGVAR(active), []];
		if !(_action isEqualTo []) then {
			[QGVAR(start), [_active # 0, _active # 1, _x]] call CBA_fnc_localEvent;
		};
	} forEach allMissionObjects "";
};

addMissionEventHandler ["ExtensionCallback", {
    params ["_name", "_function", "_data"];

    if ((tolower _name) isEqualTo "live_radio_log") exitWith {
		LOG_SYS(_function,_data);
	};
	if !((tolower _name) isEqualTo "live_radio") exitWith {};
	// systemChat format ["%1: %2", _function, _data];
}];
