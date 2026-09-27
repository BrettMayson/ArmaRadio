#include "script_component.hpp"
/*
 * Author: BrettMayson
 * Re-applies the local gain for all active sources after Streamer Mode is toggled.
 *
 * Arguments:
 * None
 *
 * Return Value:
 * None
 *
 * Example:
 * [] call live_radio_manager_fnc_refreshStreamerMode
 *
 * Public: No
 */

{
    private _id = _x;
    private _source = _y;

    private _gain = _source getVariable [QGVAR(volume), 1];
    private _url = _source getVariable [QGVAR(active), []] param [1, ""];

    if (missionNamespace getVariable [QGVAR(streamerMode), false] && {!([_url] call FUNC(isCopyrightSafe))}) then {
        _gain = 0;
    };

    EXT callExtension ["source:gain", [_id, _gain]];
} forEach GVAR(sources);
