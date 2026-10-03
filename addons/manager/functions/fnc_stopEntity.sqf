#include "script_component.hpp"
// Author: Joncantplay

params ["_entity"];

if (isServer) then {
    private _active = _entity getVariable [QGVAR(active), []];
    if (_active isNotEqualTo []) then {
        _entity setVariable [QGVAR(active), nil, true];
        [QGVAR(stop), [_active#0]] call CBA_fnc_globalEvent;
    };
};

if (hasInterface) then {
    if (_entity isEqualTo (call CBA_fnc_currentUnit)) then {
        GVAR(listenerAlive) = false;
        call FUNC(refreshPlayback);
    };
    {
        if ((GVAR(sources) get _x) isEqualTo _entity) then {
            [QGVAR(stop), [_x]] call CBA_fnc_localEvent;
        };
    } forEach (keys GVAR(sources));
};
