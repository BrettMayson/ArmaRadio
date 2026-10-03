#include "script_component.hpp"
// Author: Joncantplay
// Load configured stations and persistent client presets without executing preset text.
GVAR(stations) = [];
GVAR(disabledStationURLs) = [];
private _customURLs = [];
{
    private _root = _x;
    {
        private _default = [_x, _root] call FUNC(isDefaultStation);
        private _url = getText (_x >> "url");
        if (_default && {missionNamespace getVariable [QGVAR(hideDefaultStations), false]}) then {
            GVAR(disabledStationURLs) pushBackUnique _url;
            continue;
        };
        _customURLs pushBackUnique _url;
        private _condition = getText (_x >> "condition");
        if (_condition == "") then {_condition = "true"};
        GVAR(stations) pushBack [
            getText (_x >> "name"), getText (_x >> "picture"), getText (_x >> "url"),
            compile _condition, getNumber (_x >> "noCopyright") == 1,
            _default
        ];
    } forEach configProperties [_x >> "CfgRadioStations", "isClass _x"];
} forEach [configFile, campaignConfigFile, missionConfigFile];

// Apply shared stations first; personal approval overrides remain local.
{
    _x params ["_setting", "_storage", "_error"];
    private _text = missionNamespace getVariable [_setting, "[]"];
    private _rows = parseSimpleArray _text;
    private _compact = toString ((toArray _text) select {!(_x in [9, 10, 13, 32])});
    private _valid = _rows isEqualType [] && {count _rows <= 100} && {_rows isNotEqualTo [] || {_compact == "[]"}};
    private _presets = [];
    if (_valid) then {
        {
            if !(_x isEqualType [] && {count _x == 3}) exitWith {_valid = false};
            _x params ["_name", "_url", "_approved"];
            if !(_name isEqualType "" && {_url isEqualType ""} && {_approved isEqualType true}) exitWith {_valid = false};
            private _lower = toLower _url;
            if (_name == "" || {count _name > 128} || {count _url > 2048} || {
                _lower find "http://" != 0 && {_lower find "https://" != 0}
            }) exitWith {_valid = false};
            _presets pushBack [_name, _url, _approved];
        } forEach _rows;
    };
    if (_valid) then {
        missionNamespace setVariable [_storage, _presets];
    } else {
        if (hasInterface) then {hint localize _error};
    };
    {
        _x params ["_name", "_url", "_approved"];
        _customURLs pushBackUnique _url;
        GVAR(stations) pushBack [_name, "", _url, {true}, _approved, false];
    } forEach (missionNamespace getVariable [_storage, []]);
} forEach [
    [QGVAR(globalStations), QGVAR(globalStationRows), "STR_Live_Radio_Interface_GlobalStationsInvalid"],
    [QGVAR(personalStations), QGVAR(personalStationRows), "STR_Live_Radio_Interface_PersonalStationsInvalid"]
];
GVAR(disabledStationURLs) = GVAR(disabledStationURLs) - _customURLs;
GVAR(stations) sort true;
EGVAR(manager,stationPolicy) = [] call EFUNC(manager,loadStationPolicy);
if (missionNamespace getVariable [QEGVAR(manager,playbackReady), false]) then {
    call EFUNC(manager,refreshPlayback);
};
[uiNamespace getVariable [QGVAR(display), displayNull]] call FUNC(updateList);
