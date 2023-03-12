#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR"]
    pub cr: CR,
    #[doc = "0x04 - IER"]
    pub ier: IER,
    #[doc = "0x08 - ADDR"]
    pub addr: ADDR,
    #[doc = "0x0c - SR"]
    pub sr: SR,
    #[doc = "0x10 - SHPGR"]
    pub shpgr: SHPGR,
    #[doc = "0x14 - SLPGR"]
    pub slpgr: SLPGR,
    #[doc = "0x18 - DR"]
    pub dr: DR,
    #[doc = "0x1c - TAR"]
    pub tar: TAR,
    #[doc = "0x20 - ADDMR"]
    pub addmr: ADDMR,
    #[doc = "0x24 - ADDSR"]
    pub addsr: ADDSR,
    #[doc = "0x28 - TOUT"]
    pub tout: TOUT,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR"]
pub mod cr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "IER"]
pub mod ier;
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "ADDR"]
pub mod addr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "SR"]
pub mod sr;
#[doc = "SHPGR (rw) register accessor: an alias for `Reg<SHPGR_SPEC>`"]
pub type SHPGR = crate::Reg<shpgr::SHPGR_SPEC>;
#[doc = "SHPGR"]
pub mod shpgr;
#[doc = "SLPGR (rw) register accessor: an alias for `Reg<SLPGR_SPEC>`"]
pub type SLPGR = crate::Reg<slpgr::SLPGR_SPEC>;
#[doc = "SLPGR"]
pub mod slpgr;
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "DR"]
pub mod dr;
#[doc = "TAR (rw) register accessor: an alias for `Reg<TAR_SPEC>`"]
pub type TAR = crate::Reg<tar::TAR_SPEC>;
#[doc = "TAR"]
pub mod tar;
#[doc = "ADDMR (rw) register accessor: an alias for `Reg<ADDMR_SPEC>`"]
pub type ADDMR = crate::Reg<addmr::ADDMR_SPEC>;
#[doc = "ADDMR"]
pub mod addmr;
#[doc = "ADDSR (rw) register accessor: an alias for `Reg<ADDSR_SPEC>`"]
pub type ADDSR = crate::Reg<addsr::ADDSR_SPEC>;
#[doc = "ADDSR"]
pub mod addsr;
#[doc = "TOUT (rw) register accessor: an alias for `Reg<TOUT_SPEC>`"]
pub type TOUT = crate::Reg<tout::TOUT_SPEC>;
#[doc = "TOUT"]
pub mod tout;
