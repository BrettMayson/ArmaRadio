#include "script_component.hpp"
/*
 * Author: Joncantplay
 * Checks whether the given object has a radio.
 *
 * Arguments:
 * 0: Object <OBJECT>
 *
 * Return Value:
 * Boolean
 *
 * Example:
 * [CONTROL, 0] call live_radio_interface_fnc_hasRadio
 *
 * Public: No
 */

params [["_object", objNull, [objNull]]];
!isNull _object && {
    (getNumber (configOf _object >> QGVAR(hasRadio)) == 1)
    || { _object getVariable [QGVAR(enabled), false] }
}
