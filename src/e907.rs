#[macro_export]
macro_rules! xcsr_addr {
    (mxstatus) => {
        0x7C0
    };
    (mhcr) => {
        0x7C1
    };
    (mhint) => {
        0x7C5
    };
    (mraddr) => {
        0x7E0
    };
    (mexstatus) => {
        0x7E1
    };
    (mnmicause) => {
        0x7E2
    };
    (mnmipc) => {
        0x7E3
    };
    (mcpuid) => {
        0xFC0
    };
    (fxcr) => {
        0x800
    };
}

pub const XCSR_ADDR_MXSTATUS: usize = xcsr_addr!(mxstatus);
pub const XCSR_ADDR_MHCR: usize = xcsr_addr!(mhcr);
pub const XCSR_ADDR_MHINT: usize = xcsr_addr!(mhint);
pub const XCSR_ADDR_MRADDR: usize = xcsr_addr!(mraddr);
pub const XCSR_ADDR_MEXSTATUS: usize = xcsr_addr!(mexstatus);
pub const XCSR_ADDR_MNMICAUSE: usize = xcsr_addr!(mnmicause);
pub const XCSR_ADDR_MNMIPC: usize = xcsr_addr!(mnmipc);
pub const XCSR_ADDR_CPUID: usize = xcsr_addr!(mcpuid);
pub const XCSR_ADDR_FXCR: usize = xcsr_addr!(fxcr);
