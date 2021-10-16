pub enum MachineType {
    UNKNOWN = 0x0000,
    AM33 = 0x01D3,
    AMD64 = 0x8664,
    ARM = 0x01C0,
    ARM64 = 0xAA64,
    ARMNT = 0x01C4,
    EBC = 0x0EBC,
    I386 = 0x014C,
    IA64 = 0x0200,
    M32R = 0x9041,
    MIPS16 = 0x0266,
    MIPSFPU = 0x0366,
    MIPSFPU16 = 0x0466,
    POWERPC = 0x01F0,
    POWERPCFP = 0x01F1,
    R4000 = 0x0166,
    RISCV32 = 0x5032,
    RISCV64 = 0x5064,
    RISCV128 = 0x5128,
    SH3 = 0x01A2,
    SH3DSP = 0x01A3,
    SH4 = 0x01A6,
    SH5 = 0x01A8,
    THUMB = 0x01C2,
    WCEMIPSV2 = 0x0169,
}