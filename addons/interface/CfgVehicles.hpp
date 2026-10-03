class CfgVehicles {
    // Vehicle actions are registered under ACE_SelfActions at runtime.
    class Items_base_F;
    class Land_FMradio_F: Items_base_F {
        class ACE_Actions {
            class ACE_MainActions {
                selection = "interaction_point";
                distance = 5;
                class GVAR(open) {
                    displayName = CSTRING(DisplayName);
                    statement = QUOTE(_target call FUNC(open));
                };
            };
        };
    };
    class Module_F;
    class GVAR(moduleRadio): Module_F {
        scope = 2;
        scopeCurator = 2;
        displayName = CSTRING(ModuleRadio);
        category = QGVAR(modules);
        function = QFUNC(moduleRadio);
        isGlobal = 1;
        isTriggerActivated = 0;
        isDisposable = 1;
        curatorCanAttach = 1;
        curatorInfoType = "";
        author = "Joncantplay";
    };
};
