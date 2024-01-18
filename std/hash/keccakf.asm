machine KeccakF(??);

    pol constant ConnA, ConnB, ConnC, GateType, kGateType, kA, kB, kC;
    pol commit a[4], b[4], c[4];

    pol a44 = a[3] * 2**33 + a[2] * 2**22 + a[1] * 2**11 + a[0];
    pol b44 = b[3] * 2**33 + b[2] * 2**22 + b[1] * 2**11 + b[0];
    pol c44 = c[3] * 2**33 + c[2] * 2**22 + c[1] * 2**11 + c[0];

    {a44, b44, c44} connect {ConnA, ConnB, ConnC};

    { GateType, a[0], b[0], c[0] } in { kGateType, kA, kB, kC } ;
    { GateType, a[1], b[1], c[1] } in { kGateType, kA, kB, kC } ;
    { GateType, a[2], b[2], c[2] } in { kGateType, kA, kB, kC } ;
    { GateType, a[3], b[3], c[3] } in { kGateType, kA, kB, kC } ;

    Global.L1 * a44 = 0;
    Global.L1 * (2**44-1-b44) = 0;