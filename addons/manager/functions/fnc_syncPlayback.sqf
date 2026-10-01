#include "script_component.hpp"
// Author: Joncantplay
/*
 * The only source:new call in the mod. Keep shared radio state separate from
 * local audio, so changing a personal setting never stops another player's radio.
 */
params ["_id"];
if (!hasInterface) exitWith {};

private _source = GVAR(sources) getOrDefault [_id, objNull];
private _url = GVAR(sourceURLs) getOrDefault [_id, ""];
private _playing = GVAR(playingSources) getOrDefault [_id, false];
private _allowed = GVAR(playbackReady) && {alive (call CBA_fnc_currentUnit)} && {!isNull _source} && {alive _source} && {
    !([_source] call FUNC(isMuted)) && {[_url] call FUNC(canHear)}
};

if (!_allowed) exitWith {
    if (_playing) then {
        EXT callExtension ["source:destroy", [_id]];
        GVAR(playingSources) deleteAt _id;
        GVAR(status) deleteAt _id;
        GVAR(sourcesTitles) deleteAt _id;
        [QGVAR(metadataUpdated), [_id, ""]] call CBA_fnc_localEvent;
    };
};

if (!_playing) then {
    private _gain = GVAR(sourceVolumes) getOrDefault [_id, 1];
    private _result = EXT callExtension ["source:new", [_id, _url, _gain]];
    if ((_result param [1, -1]) == 0 && {(_result param [2, -1]) == 0}) then {
        GVAR(playingSources) set [_id, true];
        GVAR(status) set [_id, ["loading", diag_tickTime]];
    } else {
        GVAR(status) set [_id, ["error", diag_tickTime]];
    };
    [QGVAR(metadataUpdated), [_id, ""]] call CBA_fnc_localEvent;
};
