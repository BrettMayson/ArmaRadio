#include "script_component.hpp"
// Author: Joncantplay

params ["_action"];
if (_action == "streamer") exitWith {call FUNC(handleStreamer); true};

private _display = uiNamespace getVariable [QGVAR(display), displayNull];
if (isNull _display) exitWith {false};

switch (_action) do {
    case "power": {
        [_display displayCtrl IDC_POWER] call FUNC(handlePower);
    };
    case "retry": {
        if (ctrlEnabled (_display displayCtrl IDC_RETRY)) then {
            [_display getVariable QGVAR(object)] call EFUNC(manager,retry);
            [_display] call FUNC(updateStatus);
        };
    };
    case "copy": {
        [_display displayCtrl IDC_COPY_TITLE] call FUNC(handleCopyTitle);
    };
};
true
