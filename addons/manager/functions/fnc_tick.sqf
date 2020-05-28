#include "script_component.hpp"

private _d = vectorDir player;
private _u = vectorUp player;
EXT callExtension ["orientation", [_d#0, _d#1, _d#2, _u#0, _u#1, _u#2]];

{
	private _source = GVAR(active) getVariable _x;
	if (alive _source) then {
		private _pos = getPosASL _source;
		private _ppos = getPosASL ACE_player;
		private _data = [
			_x,
			(_pos#0 - _ppos#0) toFixed 2,
			(_pos#1 - _ppos#1) toFixed 2,
			(_pos#2 - _ppos#2) toFixed 2
		];
		EXT callExtension ["pos", _data];
	} else {
		EXT callExtension ["destroy", [_x]];
		GVAR(active) setVariable [_x, nil];
		systemChat format ["%1 destroy %2", EXT, [_x]];
	};
} forEach allVariables GVAR(active);
