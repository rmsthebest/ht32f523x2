#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART_URDR"]
    pub uart_urdr: UART_URDR,
    #[doc = "0x04 - UART_URCR"]
    pub uart_urcr: UART_URCR,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - UART_URIER"]
    pub uart_urier: UART_URIER,
    #[doc = "0x10 - UART_URSIFR"]
    pub uart_ursifr: UART_URSIFR,
    _reserved4: [u8; 0x10],
    #[doc = "0x24 - UART_URDLR"]
    pub uart_urdlr: UART_URDLR,
    #[doc = "0x28 - UART_URTSTR"]
    pub uart_urtstr: UART_URTSTR,
}
#[doc = "UART_URDR (rw) register accessor: an alias for `Reg<UART_URDR_SPEC>`"]
pub type UART_URDR = crate::Reg<uart_urdr::UART_URDR_SPEC>;
#[doc = "UART_URDR"]
pub mod uart_urdr;
#[doc = "UART_URCR (rw) register accessor: an alias for `Reg<UART_URCR_SPEC>`"]
pub type UART_URCR = crate::Reg<uart_urcr::UART_URCR_SPEC>;
#[doc = "UART_URCR"]
pub mod uart_urcr;
#[doc = "UART_URIER (rw) register accessor: an alias for `Reg<UART_URIER_SPEC>`"]
pub type UART_URIER = crate::Reg<uart_urier::UART_URIER_SPEC>;
#[doc = "UART_URIER"]
pub mod uart_urier;
#[doc = "UART_URSIFR (rw) register accessor: an alias for `Reg<UART_URSIFR_SPEC>`"]
pub type UART_URSIFR = crate::Reg<uart_ursifr::UART_URSIFR_SPEC>;
#[doc = "UART_URSIFR"]
pub mod uart_ursifr;
#[doc = "UART_URDLR (rw) register accessor: an alias for `Reg<UART_URDLR_SPEC>`"]
pub type UART_URDLR = crate::Reg<uart_urdlr::UART_URDLR_SPEC>;
#[doc = "UART_URDLR"]
pub mod uart_urdlr;
#[doc = "UART_URTSTR (rw) register accessor: an alias for `Reg<UART_URTSTR_SPEC>`"]
pub type UART_URTSTR = crate::Reg<uart_urtstr::UART_URTSTR_SPEC>;
#[doc = "UART_URTSTR"]
pub mod uart_urtstr;
