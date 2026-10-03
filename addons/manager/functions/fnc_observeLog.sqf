#include "script_component.hpp"
// Author: Joncantplay
// Interpret only source-specific, exact messages emitted by the bundled DLL.
// Metadata is deliberately not proof that audio playback started.
params ["_target", "_level", "_message"];
if (!hasInterface || {_target != "live_radio::source"}) exitWith {};
if (_message == "cleaning up sources") exitWith {
    {
        if ((GVAR(status) getOrDefault [_x, ["", 0]])#0 != "error") then {
            GVAR(status) set [_x, ["ended", diag_tickTime]];
        };
    } forEach (keys GVAR(playingSources));
};
if (_message find "Playing source for " != 0 && {_message find "Stream closed for " != 0} && {_message find "Stream receiver disconnected for " != 0} && {_message find "killing thread, error queueing buffer for " != 0}) exitWith {};
{
    private _id = _x;
    if (_message find format ["Playing source for %1, ", _id] == 0) then {
        GVAR(status) set [_id, ["started", diag_tickTime]];
    };
    if (_message == format ["Stream closed for %1", _id] || {_message == format ["Stream receiver disconnected for %1", _id]} || {_message find format ["killing thread, error queueing buffer for %1:", _id] == 0}) then {
        if ((GVAR(status) getOrDefault [_id, ["", 0]])#0 != "error") then {
            GVAR(status) set [_id, ["ended", diag_tickTime]];
        };
    };
} forEach (keys GVAR(playingSources));
