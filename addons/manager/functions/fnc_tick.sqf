#include "script_component.hpp"

private _player = call CBA_fnc_currentUnit;
private _inZeus = !(isNull (findDisplay 312));


private _data = if (_inZeus) then {
    private _d = vectorDir curatorCamera;
    _d append vectorUp curatorCamera;
    _d
} else {
    private _d = eyeDirection _player;
    _d append vectorUp _player;
    _d
};
EXT callExtension ["listener:dir", _data];

{
    if (alive _y) then {
        private _pos = getPosASL _y;
        private _data = [_x, 0, 0, 0];
        if (_inZeus || {!(_y isEqualTo vehicle _player)}) then {
            private _ppos = eyePos _player;
            if (_inZeus) then {
                _ppos = getPosASL curatorCamera;
            };
            _data = [
                _x,
                (_pos#0 - _ppos#0) toFixed 2,
                (_pos#1 - _ppos#1) toFixed 2,
                (_pos#2 - _ppos#2) toFixed 2
            ];
        };
        EXT callExtension ["source:pos", _data];
    } else {
        [QGVAR(stop), [_x]] call CBA_fnc_localEvent;
    };
} forEach GVAR(sources);
