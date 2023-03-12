#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USART_USRDR"]
    pub usart_usrdr: USART_USRDR,
    #[doc = "0x04 - USART_USRCR"]
    pub usart_usrcr: USART_USRCR,
    #[doc = "0x08 - USART_USRFCR"]
    pub usart_usrfcr: USART_USRFCR,
    #[doc = "0x0c - USART_USRIER"]
    pub usart_usrier: USART_USRIER,
    #[doc = "0x10 - USART_USRSIFR"]
    pub usart_usrsifr: USART_USRSIFR,
    #[doc = "0x14 - USART_USRTPR"]
    pub usart_usrtpr: USART_USRTPR,
    #[doc = "0x18 - USART_IrDACR"]
    pub usart_ir_dacr: USART_IR_DACR,
    #[doc = "0x1c - USART_RS485CR"]
    pub usart_rs485cr: USART_RS485CR,
    #[doc = "0x20 - USART_SYNCR"]
    pub usart_syncr: USART_SYNCR,
    #[doc = "0x24 - USART_USRDLR"]
    pub usart_usrdlr: USART_USRDLR,
    #[doc = "0x28 - USART_USRTSTR"]
    pub usart_usrtstr: USART_USRTSTR,
}
#[doc = "USART_USRDR (rw) register accessor: an alias for `Reg<USART_USRDR_SPEC>`"]
pub type USART_USRDR = crate::Reg<usart_usrdr::USART_USRDR_SPEC>;
#[doc = "USART_USRDR"]
pub mod usart_usrdr;
#[doc = "USART_USRCR (rw) register accessor: an alias for `Reg<USART_USRCR_SPEC>`"]
pub type USART_USRCR = crate::Reg<usart_usrcr::USART_USRCR_SPEC>;
#[doc = "USART_USRCR"]
pub mod usart_usrcr;
#[doc = "USART_USRFCR (rw) register accessor: an alias for `Reg<USART_USRFCR_SPEC>`"]
pub type USART_USRFCR = crate::Reg<usart_usrfcr::USART_USRFCR_SPEC>;
#[doc = "USART_USRFCR"]
pub mod usart_usrfcr;
#[doc = "USART_USRIER (rw) register accessor: an alias for `Reg<USART_USRIER_SPEC>`"]
pub type USART_USRIER = crate::Reg<usart_usrier::USART_USRIER_SPEC>;
#[doc = "USART_USRIER"]
pub mod usart_usrier;
#[doc = "USART_USRSIFR (rw) register accessor: an alias for `Reg<USART_USRSIFR_SPEC>`"]
pub type USART_USRSIFR = crate::Reg<usart_usrsifr::USART_USRSIFR_SPEC>;
#[doc = "USART_USRSIFR"]
pub mod usart_usrsifr;
#[doc = "USART_USRTPR (rw) register accessor: an alias for `Reg<USART_USRTPR_SPEC>`"]
pub type USART_USRTPR = crate::Reg<usart_usrtpr::USART_USRTPR_SPEC>;
#[doc = "USART_USRTPR"]
pub mod usart_usrtpr;
#[doc = "USART_IrDACR (rw) register accessor: an alias for `Reg<USART_IR_DACR_SPEC>`"]
pub type USART_IR_DACR = crate::Reg<usart_ir_dacr::USART_IR_DACR_SPEC>;
#[doc = "USART_IrDACR"]
pub mod usart_ir_dacr;
#[doc = "USART_RS485CR (rw) register accessor: an alias for `Reg<USART_RS485CR_SPEC>`"]
pub type USART_RS485CR = crate::Reg<usart_rs485cr::USART_RS485CR_SPEC>;
#[doc = "USART_RS485CR"]
pub mod usart_rs485cr;
#[doc = "USART_SYNCR (rw) register accessor: an alias for `Reg<USART_SYNCR_SPEC>`"]
pub type USART_SYNCR = crate::Reg<usart_syncr::USART_SYNCR_SPEC>;
#[doc = "USART_SYNCR"]
pub mod usart_syncr;
#[doc = "USART_USRDLR (rw) register accessor: an alias for `Reg<USART_USRDLR_SPEC>`"]
pub type USART_USRDLR = crate::Reg<usart_usrdlr::USART_USRDLR_SPEC>;
#[doc = "USART_USRDLR"]
pub mod usart_usrdlr;
#[doc = "USART_USRTSTR (rw) register accessor: an alias for `Reg<USART_USRTSTR_SPEC>`"]
pub type USART_USRTSTR = crate::Reg<usart_usrtstr::USART_USRTSTR_SPEC>;
#[doc = "USART_USRTSTR"]
pub mod usart_usrtstr;
