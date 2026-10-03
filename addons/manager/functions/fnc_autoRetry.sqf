#include "script_component.hpp"
// Polling avoids uncancellable delayed closures. Each scheduled attempt carries
// the exact native generation, URL, entity and listener vehicle that created it.
if (!GVAR(playbackReady)) exitWith {};
{
    private _id = _x;
    private _status = GVAR(status) getOrDefault [_id, ["", 0]];
    private _native = GVAR(nativeIDs) getOrDefault [_id, ""];
    private _source = GVAR(sources) getOrDefault [_id, objNull];
    private _url = GVAR(sourceURLs) getOrDefault [_id, ""];
    private _vehicle = vehicle (call CBA_fnc_currentUnit);
    private _valid = GVAR(autoRetry) && {_native != ""} && {alive _source} &&
        {alive (call CBA_fnc_currentUnit)} &&
        {(_source getVariable [QGVAR(active), []]) isEqualTo [_id, _url]} &&
        {(GVAR(attemptVehicle) getOrDefault [_id, objNull]) isEqualTo _vehicle} &&
        {!([_source] call FUNC(isMuted))} && {[_url] call FUNC(canHear)};
    if (!_valid || {!((_status#0) in ["error", "ended"])}) then {
        GVAR(retryDue) deleteAt _id;
    } else {
        private _pending = GVAR(retryDue) getOrDefault [_id, []];
        if (_pending isEqualTo []) then {
            GVAR(retryDue) set [_id, [diag_tickTime + 2, _native, _url, _source, _vehicle]];
            if (GVAR(debugMessages)) then {diag_log format ["[LIVE_RADIO] Scheduling retry in 2 seconds: %1 (%2)", _native, _status]};
        } else {
            _pending params ["_due", "_generation", "_expectedURL", "_entity", "_expectedVehicle"];
            if (diag_tickTime >= _due) then {
                GVAR(retryDue) deleteAt _id;
                // Revalidate immediately before destroying/reopening. No wait or
                // scheduled thread can interleave these unscheduled PFH calls.
                if (GVAR(autoRetry) && {_native == _generation} && {_url == _expectedURL} &&
                    {_source isEqualTo _entity} && {_vehicle isEqualTo _expectedVehicle} &&
                    {(_source getVariable [QGVAR(active), []]) isEqualTo [_id, _url]}) then {
                    [_id] call FUNC(destroyLocal);
                    [_id] call FUNC(syncPlayback);
                    GVAR(autoAttempts) set [_id, GVAR(nativeIDs) getOrDefault [_id, ""]];
                };
            };
        };
    };
} forEach (keys GVAR(sources));
