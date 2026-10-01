#include "script_component.hpp"
// Author: Joncantplay
params [["_object", objNull, [objNull]]];
!isNull _object && {
    _object isKindOf "Land_FMradio_F" || {_object isKindOf "Car"} || {
        _object isKindOf "Air"
    } || {_object isKindOf "Ship"} || {_object getVariable [QGVAR(enabled), false]}
}
