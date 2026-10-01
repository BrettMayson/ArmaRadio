#include "script_component.hpp"
// Author: Joncantplay

params ["_control", ["_button", 1]];
if (_button != 1) exitWith {};

private _title = _control getVariable [QGVAR(copyTitle), ""];
if (_title == "") exitWith {};

copyToClipboard _title;
hint format [localize "STR_Live_Radio_Interface_TitleCopied", _title];
