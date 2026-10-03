#include "script_component.hpp"
params ["_ctrlSearchBar"];
private _token = (_ctrlSearchBar getVariable [QGVAR(searchToken), 0]) + 1;
_ctrlSearchBar setVariable [QGVAR(searchToken), _token];
[{
    params ["_control", "_token"];
    if (!isNull _control && {_control getVariable [QGVAR(searchToken), 0] == _token}) then {
        [ctrlParent _control] call FUNC(updateList);
    };
}, [_ctrlSearchBar, _token], 0.15] call CBA_fnc_waitAndExecute;
