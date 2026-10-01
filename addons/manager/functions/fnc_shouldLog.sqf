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
true
