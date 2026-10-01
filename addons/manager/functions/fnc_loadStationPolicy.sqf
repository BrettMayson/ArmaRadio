#include "script_component.hpp"
// Author: Joncantplay
/*
 * Build a local allowlist from addon, campaign and mission station configs.
 * Only numeric noCopyright = 1 grants permission. Missing/invalid flags deny it.
 * Duplicate URLs must agree: one unapproved entry keeps the URL blocked.
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
            _policy set [_url, _approved && {_policy getOrDefault [_url, true]}];
        };
    } forEach configProperties [_x >> "CfgRadioStations", "isClass _x"];
} forEach _roots;

_policy
