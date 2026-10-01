#include "script_component.hpp"
// Author: Joncantplay

params ["_control"];

private _title = _control getVariable [QGVAR(copyTitle), ""];
if (_title == "") exitWith {};

copyToClipboard _title;
systemChat format [localize "STR_Live_Radio_Interface_TitleCopied", _title];
