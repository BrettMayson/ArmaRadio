class CfgVehicles {
    class Module_F;
    class GVAR(Radio): Module_F {
        scope = 2;
        scopeCurator = 2;
        displayName = CSTRING(Radio);
        category = QGVAR(modules);
        function = QFUNC(radio);
        isGlobal = 1;
        isTriggerActivated = 0;
        isDisposable = 1;
        curatorCanAttach = 1;
        curatorInfoType = "";
        author = "Joncantplay";
    };
};
