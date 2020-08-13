#include "script_component.hpp"
/*
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
 * [CONTROL, true] call radio_interface_fnc_handlePower
 *
 * Public: No
 */

params ["_ctrlPower", ["_toggle", true]];

private _display = ctrlParent _ctrlPower;
private _powered = _display getVariable QGVAR(powered);

// Toggle the powered state if needed
if (_toggle) then {
    _powered = !_powered;

    // Start playing the selected station if the radio is powered on, otherwise turn off
    private _url = if (_powered) then {
        private _ctrlList = _display displayCtrl IDC_LIST;
        (_ctrlList getVariable str lbCurSel _ctrlList) param [3, ""]
    } else {
        ""
    };

    private _object = _display getVariable QGVAR(object);
    [_object, _url] call EFUNC(manager,play);

    _display setVariable [QGVAR(powered), _powered];
};

// Update visuals to reflect current state
private _color = [[1, 1, 1, 0.25], [1, 1, 1, 1]] select _powered;
_ctrlPower ctrlSetTextColor _color;

private _tooltip = [LSTRING(Off), LSTRING(On)] select _powered;
_ctrlPower ctrlSetTooltip localize _tooltip;
