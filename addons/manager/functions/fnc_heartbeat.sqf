#include "script_component.hpp"

private _result = EXT callExtension ["heartbeat", []];
// A successful heartbeat ends the previous cleanup/logging episode.
if ((_result param [1, -1]) == 0 && {(_result param [2, -1]) == 0}) then {
    GVAR(cleanupLogReported) = false;
};
