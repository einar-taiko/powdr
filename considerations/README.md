
```pil
include "global.pil";


namespace KeccakF(%N);

/// constant polynomials
    pol constant ConnA, ConnB, ConnC, GateType, kGateType, kA, kB, kC;
/// committed polys, 4 limbs each, each limb 11 bits.
    pol commit a[4], b[4], c[4];

/// composing and binding them to symbols
    pol a44 = a[3] * 2**33 + a[2] * 2**22 + a[1] * 2**11 + a[0];
    pol b44 = b[3] * 2**33 + b[2] * 2**22 + b[1] * 2**11 + b[0];
    pol c44 = c[3] * 2**33 + c[2] * 2**22 + c[1] * 2**11 + c[0];
/// a copy constraint
    {a44, b44, c44} connect {ConnA, ConnB, ConnC};
/// plookups
    { GateType, a[0], b[0], c[0] } in { kGateType, kA, kB, kC } ;
    { GateType, a[1], b[1], c[1] } in { kGateType, kA, kB, kC } ;
    { GateType, a[2], b[2], c[2] } in { kGateType, kA, kB, kC } ;
    { GateType, a[3], b[3], c[3] } in { kGateType, kA, kB, kC } ;

    Global.L1 * a44 = 0;
    Global.L1 * (2**44-1-b44) = 0;
```





https://github.com/0xPolygonHermez/zkevm-proverjs/blob/main/pil/main.pil

https://docs.polygon.technology/zkEVM/spec/pil/

seems I need to translate the
https://github.com/0xPolygonHermez/zkevm-proverjs/blob/main/pil/keccakf.pil

to  powdr-pil.

investigating what is needed to be done, if there already is a kecchak.pil


https://github.com/powdr-labs/powdr/tree/b7d89fbbefc76053f0e132344fe56229d70d8acd/riscv/tests/riscv_data/keccak


where is the design of powdr/test_data/polygon-hermez/keccakf.pil

unclear what the difference between keccak p and f variant is, mentioned here https://crates.io/crates/keccak


git log refers to this as keccak 44bits, although witness cols implies `3*4*32 = 384`.


https://polygon.technology/blog/zk-white-paper-efficient-zk-proofs-for-keccak#

what is `kGateType`?

https://docs.polygon.technology/zkEVM/architecture/zkprover/hashing-state-machines/keccakf-sm/


https://docs.polygon.technology/zkEVM/architecture/zkprover/hashing-state-machines/keccakf-sm/

`ConnA` is convention for constant pol A.
