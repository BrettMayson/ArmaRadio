#include "script_component.hpp"
// Author: Joncantplay
/*
 * Build a local allowlist from addon, campaign and mission station configs.
 * Only numeric noCopyright = 1 grants permission. Missing/invalid flags deny it.
 * Duplicate URLs: the last loaded entry wins; personal presets are applied last.
 */
params [["_roots", [configFile, campaignConfigFile, missionConfigFile]]];

private _policy = createHashMap;
{
    {
        private _url = getText (_x >> "url");
        if (_url != "") then {
            private _approved = isNumber (_x >> "noCopyright") && {
                getNumber (_x >> "noCopyright") isEqualTo 1
            };
            _policy set [_url, _approved];
        };
    } forEach configProperties [_x >> "CfgRadioStations", "isClass _x"];
} forEach _roots;

{
    _x params ["_name", "_url", "_approved"];
    _policy set [_url, _approved];
} forEach (
    (missionNamespace getVariable [QEGVAR(interface,globalStationRows), []]) +
    (missionNamespace getVariable [QEGVAR(interface,personalStationRows), []])
);

_policy
