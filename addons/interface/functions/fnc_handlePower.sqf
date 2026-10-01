#include "script_component.hpp"
/*
 * Edited by: Joncantplay
 * Author: mharis001
 * Handles clicking the power button.
 *
 * Arguments:
 * 0: Power Button <CONTROL>
 * 1: Toggle <BOOL> (default: true)
 *
 * Return Value:
 * None
 *
 * Example:
 * [CONTROL, true] call live_radiointerface_fnc_handlePower
 *
 * Public: No
 */

params ["_ctrlPower", ["_toggle", true]];
if !([(ctrlParent _ctrlPower) getVariable QGVAR(object)] call FUNC(canOpen)) exitWith {};

private _display = ctrlParent _ctrlPower;
private _powered = _display getVariable QGVAR(powered);

// Toggle the powered state if needed
if (_toggle) then {
    _powered = !_powered;

    // Start playing the selected station if the radio is powered on, otherwise turn off
    private _url = if (_powered) then {
        private _ctrlList = _display displayCtrl IDC_LIST;
        (_ctrlList getVariable str lbCurSel _ctrlList) param [2, ""]
    } else {
        ""
    };

    _powered = _url != "";
    private _object = _display getVariable QGVAR(object);
    [_object, _url] call EFUNC(manager,play);

    _display setVariable [QGVAR(powered), _powered];
};

// Update visuals to reflect current state
private _color = [[0.95, 0.2, 0.18, 1], [0.3, 0.85, 0.8, 1]] select _powered;
_ctrlPower ctrlSetTextColor _color;

private _tooltip = ["str_a3_rscdisplayconfigure_ca_mouseacceleration_off", "str_a3_rscdisplayconfigure_ca_mouseacceleration_on"] select _powered;
_ctrlPower ctrlSetTooltip localize _tooltip;

// Also refresh the mute notice when a blocked station is switched on or off.
[_display] call FUNC(updateInfo);
