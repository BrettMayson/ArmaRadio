#include "script_component.hpp"
// Author: Joncantplay
params [["_display", displayNull]];
if (isNull _display) exitWith {};
private _enabled = missionNamespace getVariable [QEGVAR(manager,streamerMode), false];
private _status = _display displayCtrl IDC_STREAMER_STATUS;
_status ctrlSetText format ["Streamer Mode enabled: %1", ["False", "True"] select _enabled];
_status ctrlSetTextColor ([[0.8, 0.83, 0.87, 1], [0.3, 0.85, 0.8, 1]] select _enabled);
private _toggle = _display displayCtrl IDC_STREAMER_TOGGLE;
_toggle ctrlSetText localize "STR_Live_Radio_Interface_ToggleStreamer";
_toggle ctrlSetTooltip localize "STR_Live_Radio_Interface_StreamerTooltip";
_toggle ctrlEnable true;
private _object = _display getVariable [QGVAR(object), objNull];
private _muted = _object getVariable [QEGVAR(manager,locallyMuted), false];
private _mute = _display displayCtrl IDC_MUTE;
_mute ctrlSetText localize (["STR_Live_Radio_Interface_Mute", "STR_Live_Radio_Interface_Unmute"] select _muted);
_mute ctrlSetTooltip localize "STR_Live_Radio_Interface_MuteTooltip";
