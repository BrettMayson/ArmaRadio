// COMPONENT should be defined in the script_component.hpp and included BEFORE this hpp

#define MAINPREFIX dynulo
#define PREFIX radio

#define EXT "dynulo_radio"

#include "script_version.hpp"

#define VERSION     MAJOR.MINOR
#define VERSION_STR MAJOR.MINOR.PATCH.BUILD
#define VERSION_AR  MAJOR,MINOR,PATCH,BUILD

// MINIMAL required version for the Mod. Components can specify others..
#define REQUIRED_VERSION 1.92

#ifdef COMPONENT_BEAUTIFIED
    #define COMPONENT_NAME QUOTE(Dynulo Radio - COMPONENT_BEAUTIFIED)
#else
    #define COMPONENT_NAME QUOTE(Dynulo Radio - COMPONENT)
#endif
