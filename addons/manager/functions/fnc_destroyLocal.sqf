#include "script_component.hpp"
// Invalidate callbacks BEFORE asking the old native worker to stop.
params ["_id"];
private _native = GVAR(nativeIDs) getOrDefault [_id, ""];
GVAR(nativeIDs) deleteAt _id;
GVAR(attemptSources) deleteAt _native;
GVAR(retryDue) deleteAt _id;
GVAR(autoAttempts) deleteAt _id;
GVAR(playingSources) deleteAt _id;
if (_native != "") then {EXT callExtension ["source:destroy", [_native]]};
