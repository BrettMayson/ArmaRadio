#include "script_component.hpp"

private _player = call CBA_fnc_currentUnit;
private _vehicle = vehicle _player;
if (_vehicle isNotEqualTo GVAR(listenerVehicle)) then {
    GVAR(listenerVehicle) = _vehicle;
    GVAR(retryDue) = createHashMap;
    {
        if (((GVAR(status) getOrDefault [_x, [""]])#0) in ["loading", "error", "ended"] && {
            (GVAR(attemptVehicle) getOrDefault [_x, objNull]) isNotEqualTo _vehicle
        }) then {
            [_x] call FUNC(destroyLocal);
            GVAR(status) set [_x, ["ended", diag_tickTime]];
        };
    } forEach (keys GVAR(nativeIDs));
};
private _listenerAlive = alive _player;
if (_listenerAlive isNotEqualTo GVAR(listenerAlive)) then {
    GVAR(listenerAlive) = _listenerAlive;
    call FUNC(refreshPlayback);
};

// No listener/position extension traffic when no sources are registered.
if (count GVAR(sources) == 0) exitWith {};
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
if (count GVAR(playingSources) > 0) then {EXT callExtension ["listener:dir", _data]};

{
    private _source = GVAR(sources) get _x;
    if (alive _source) then {
        if ((GVAR(playingSources) getOrDefault [_x, false]) && {!(((GVAR(status) getOrDefault [_x, ["loading", 0]])#0) in ["ended", "error"])}) then {
            private _ppos = if (_inZeus) then {getPosASL curatorCamera} else {eyePos _player};
            private _inside = !_inZeus && {vehicle _player isEqualTo _source};
            private _centered = _inside && {
                (_source isKindOf "Air") || {
                    !(missionNamespace getVariable [QGVAR(inside3DAudio), false])
                }
            };
            private _relative = if (_centered) then {
                [0, 0, 0]
            } else {
                (getPosASL _source) vectorDiff _ppos
            };
            private _data = [
                GVAR(nativeIDs) get _x,
                (_relative#0) toFixed 2,
                (_relative#1) toFixed 2,
                (_relative#2) toFixed 2
            ];
            EXT callExtension ["source:pos", _data];
        };
    } else {
        [QGVAR(stop), [_x]] call CBA_fnc_localEvent;
    };
} forEach (keys GVAR(sources));
