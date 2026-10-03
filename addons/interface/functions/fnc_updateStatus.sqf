#include "script_component.hpp"
// Author: Joncantplay
params [["_display", displayNull]];
if (isNull _display) exitWith {};
private _object = _display getVariable [QGVAR(object), objNull];
private _active = _object getVariable [QEGVAR(manager,active), []];
private _id = _active param [0, ""];
private _url = _active param [1, ""];
private _powered = _url != "";
_display setVariable [QGVAR(powered), _powered];
private _power = _display displayCtrl IDC_POWER;
_power ctrlSetTextColor ([[0.95,0.2,0.18,1], [0.3,0.85,0.8,1]] select _powered);
_power ctrlSetTooltip localize (["STR_Live_Radio_Interface_PowerOn", "STR_Live_Radio_Interface_PowerOff"] select _powered);
// Disabled controls turn grey. Keep this enabled to show red when powered off.
// handlePower ignores clicks until a station is selected.
_power ctrlEnable (!isNull _object && {alive _object});
private _state = EGVAR(manager,status) getOrDefault [_id, ["loading", diag_tickTime]];
private _elapsed = floor (diag_tickTime - (_state#1));
private _key = "Off";
private _color = [0.72,0.77,0.82,1];
private _retry = false;
if (_powered) then {
    _key = switch (true) do {
        case (missionNamespace getVariable [QEGVAR(manager,muteAll), false]): {"AllMuted"};
        case (_object getVariable [QEGVAR(manager,locallyMuted), false]): {"ObjectMuted"};
        case (_url in (missionNamespace getVariable [QGVAR(disabledStationURLs), []])): {"DefaultDisabled"};
        case (!([_url] call EFUNC(manager,canHear))): {"StreamerMuted"};
        default {
            _retry = _elapsed >= 3 || {_state#0 != "loading"};
            switch (_state#0) do {
                case "started": {_color = [0.3,0.85,0.8,1]; "Started"};
                case "error": {_color = [1,0.55,0.5,1]; "Error"};
                case "ended": {_color = [1,0.55,0.5,1]; "Ended"};
                default {_color = [1,0.79,0.4,1]; ["Loading", "Unconfirmed"] select (_elapsed >= 25)};
            }
        };
    };
};
private _text = switch (_key) do {
    case "DefaultDisabled": {localize "STR_Live_Radio_Interface_StatusDefaultDisabled"};
    case "Off": {localize "STR_Live_Radio_Interface_StatusOff"};
    case "AllMuted": {localize "STR_Live_Radio_Interface_StatusAllMuted"};
    case "ObjectMuted": {localize "STR_Live_Radio_Interface_StatusObjectMuted"};
    case "StreamerMuted": {localize "STR_Live_Radio_Interface_StatusStreamerMuted"};
    case "Started": {localize "STR_Live_Radio_Interface_StatusStarted"};
    case "Loading": {localize "STR_Live_Radio_Interface_StatusLoading"};
    case "Unconfirmed": {localize "STR_Live_Radio_Interface_StatusUnconfirmed"};
    case "Error": {localize "STR_Live_Radio_Interface_StatusError"};
    case "Ended": {localize "STR_Live_Radio_Interface_StatusEnded"};
    default {""};
};
if (_key == "Loading") then {_text = format [_text, _elapsed]};
if (_key == "Error") then {
    private _message = _state param [2, ""];
    private _category = (_message splitString ":") param [0, ""];
    private _errorKey = switch (_category) do {
        case "connect": {"Unreachable"};
        case "timeout": {"Timeout"};
        case "redirect": {"Redirect"};
        case "http": {"HTTP"};
        case "unsupported": {"Unsupported"};
        case "decode": {"Decode"};
        case "read": {"Interrupted"};
        default {"Error"};
    };
    _text = localize ("STR_Live_Radio_Interface_Status" + _errorKey);
    if (_message != "") then {_text = _text + endl + (_message select [(_message find ":") + 1, 180])};
};
private _ctrl = _display displayCtrl IDC_STATUS;
_ctrl ctrlSetText _text;
_ctrl ctrlSetTextColor _color;
_ctrl ctrlSetTooltip _text;
(_display displayCtrl IDC_RETRY) ctrlEnable (_retry && {!isNull _object} && {alive _object});
private _volumeMouse = _display displayCtrl IDC_VOLUME_BAR_MOUSE;
if !(_volumeMouse getVariable [QGVAR(moving), false]) then {
    [_display, _object getVariable [QEGVAR(manager,volume), DEFAULT_VOLUME]] call FUNC(handleVolume);
};
