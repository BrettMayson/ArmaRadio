#include "script_component.hpp"
// Author: Joncantplay
// Filter extension messages before CBA forwards them to the Arma RPT.
params ["_target", "_level", "_message"];
if (_level in ["DEBUG", "TRACE"] && {
    !(missionNamespace getVariable [QGVAR(debugMessages), false])
}) exitWith {false};
if (_target isEqualTo "live_radio::source" && {
    _level isEqualTo "DEBUG" && {_message isEqualTo "cleaning up sources"}
}) exitWith {
    // Retain the existing watchdog rate limit even when debugging is enabled.
    if (!hasInterface || {GVAR(cleanupLogReported)}) exitWith {false};
    GVAR(cleanupLogReported) = true;
    true
};
if (_level in ["ERROR", "WARN"] && {!GVAR(debugMessages)}) exitWith {
    private _key = _target + ":" + _message;
    private _last = GVAR(nativeLogTimes) getOrDefault [_key, -30];
    if (diag_tickTime - _last < 30) exitWith {false};
    if (count GVAR(nativeLogTimes) > 128) then {GVAR(nativeLogTimes) = createHashMap};
    GVAR(nativeLogTimes) set [_key, diag_tickTime];
    true
};
true
