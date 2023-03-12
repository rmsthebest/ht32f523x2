#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR"]
    pub cr: CR,
    #[doc = "0x04 - SDR"]
    pub sdr: SDR,
    #[doc = "0x08 - CSR"]
    pub csr: CSR,
    #[doc = "0x0c - DR"]
    pub dr: DR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR"]
pub mod cr;
#[doc = "SDR (rw) register accessor: an alias for `Reg<SDR_SPEC>`"]
pub type SDR = crate::Reg<sdr::SDR_SPEC>;
#[doc = "SDR"]
pub mod sdr;
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "CSR"]
pub mod csr;
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "DR"]
pub mod dr;
