#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CNTCFR"]
    pub cntcfr: CNTCFR,
    #[doc = "0x04 - MDCFR"]
    pub mdcfr: MDCFR,
    #[doc = "0x08 - TRCFR"]
    pub trcfr: TRCFR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - CTR"]
    pub ctr: CTR,
    _reserved4: [u8; 0x0c],
    #[doc = "0x20 - CH0ICFR"]
    pub ch0icfr: CH0ICFR,
    #[doc = "0x24 - CH1ICFR"]
    pub ch1icfr: CH1ICFR,
    #[doc = "0x28 - CH2ICFR"]
    pub ch2icfr: CH2ICFR,
    #[doc = "0x2c - CH3ICFR"]
    pub ch3icfr: CH3ICFR,
    _reserved8: [u8; 0x10],
    #[doc = "0x40 - CH0OCFR"]
    pub ch0ocfr: CH0OCFR,
    #[doc = "0x44 - CH1OCFR"]
    pub ch1ocfr: CH1OCFR,
    #[doc = "0x48 - CH2OCFR"]
    pub ch2ocfr: CH2OCFR,
    #[doc = "0x4c - CH3OCFR"]
    pub ch3ocfr: CH3OCFR,
    #[doc = "0x50 - CHCTR"]
    pub chctr: CHCTR,
    #[doc = "0x54 - CHPOLR"]
    pub chpolr: CHPOLR,
    _reserved14: [u8; 0x14],
    #[doc = "0x6c - CHBRKCFR"]
    pub chbrkcfr: CHBRKCFR,
    #[doc = "0x70 - CHBRKCTR"]
    pub chbrkctr: CHBRKCTR,
    #[doc = "0x74 - DICTR"]
    pub dictr: DICTR,
    #[doc = "0x78 - EVGR"]
    pub evgr: EVGR,
    #[doc = "0x7c - INTSR"]
    pub intsr: INTSR,
    #[doc = "0x80 - CNTR"]
    pub cntr: CNTR,
    #[doc = "0x84 - PSCR"]
    pub pscr: PSCR,
    #[doc = "0x88 - CRR"]
    pub crr: CRR,
    #[doc = "0x8c - REPR"]
    pub repr: REPR,
    #[doc = "0x90 - CH0CCR"]
    pub ch0ccr: CH0CCR,
    #[doc = "0x94 - CH1CCR"]
    pub ch1ccr: CH1CCR,
    #[doc = "0x98 - CH2CCR"]
    pub ch2ccr: CH2CCR,
    #[doc = "0x9c - CH3CCR"]
    pub ch3ccr: CH3CCR,
    #[doc = "0xa0 - CH0ACR"]
    pub ch0acr: CH0ACR,
    #[doc = "0xa4 - CH1ACR"]
    pub ch1acr: CH1ACR,
    #[doc = "0xa8 - CH2ACR"]
    pub ch2acr: CH2ACR,
    #[doc = "0xac - CH3ACR"]
    pub ch3acr: CH3ACR,
}
#[doc = "CNTCFR (rw) register accessor: an alias for `Reg<CNTCFR_SPEC>`"]
pub type CNTCFR = crate::Reg<cntcfr::CNTCFR_SPEC>;
#[doc = "CNTCFR"]
pub mod cntcfr;
#[doc = "MDCFR (rw) register accessor: an alias for `Reg<MDCFR_SPEC>`"]
pub type MDCFR = crate::Reg<mdcfr::MDCFR_SPEC>;
#[doc = "MDCFR"]
pub mod mdcfr;
#[doc = "TRCFR (rw) register accessor: an alias for `Reg<TRCFR_SPEC>`"]
pub type TRCFR = crate::Reg<trcfr::TRCFR_SPEC>;
#[doc = "TRCFR"]
pub mod trcfr;
#[doc = "CTR (rw) register accessor: an alias for `Reg<CTR_SPEC>`"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "CTR"]
pub mod ctr;
#[doc = "CH0ICFR (rw) register accessor: an alias for `Reg<CH0ICFR_SPEC>`"]
pub type CH0ICFR = crate::Reg<ch0icfr::CH0ICFR_SPEC>;
#[doc = "CH0ICFR"]
pub mod ch0icfr;
#[doc = "CH1ICFR (rw) register accessor: an alias for `Reg<CH1ICFR_SPEC>`"]
pub type CH1ICFR = crate::Reg<ch1icfr::CH1ICFR_SPEC>;
#[doc = "CH1ICFR"]
pub mod ch1icfr;
#[doc = "CH2ICFR (rw) register accessor: an alias for `Reg<CH2ICFR_SPEC>`"]
pub type CH2ICFR = crate::Reg<ch2icfr::CH2ICFR_SPEC>;
#[doc = "CH2ICFR"]
pub mod ch2icfr;
#[doc = "CH3ICFR (rw) register accessor: an alias for `Reg<CH3ICFR_SPEC>`"]
pub type CH3ICFR = crate::Reg<ch3icfr::CH3ICFR_SPEC>;
#[doc = "CH3ICFR"]
pub mod ch3icfr;
#[doc = "CH0OCFR (rw) register accessor: an alias for `Reg<CH0OCFR_SPEC>`"]
pub type CH0OCFR = crate::Reg<ch0ocfr::CH0OCFR_SPEC>;
#[doc = "CH0OCFR"]
pub mod ch0ocfr;
#[doc = "CH1OCFR (rw) register accessor: an alias for `Reg<CH1OCFR_SPEC>`"]
pub type CH1OCFR = crate::Reg<ch1ocfr::CH1OCFR_SPEC>;
#[doc = "CH1OCFR"]
pub mod ch1ocfr;
#[doc = "CH2OCFR (rw) register accessor: an alias for `Reg<CH2OCFR_SPEC>`"]
pub type CH2OCFR = crate::Reg<ch2ocfr::CH2OCFR_SPEC>;
#[doc = "CH2OCFR"]
pub mod ch2ocfr;
#[doc = "CH3OCFR (rw) register accessor: an alias for `Reg<CH3OCFR_SPEC>`"]
pub type CH3OCFR = crate::Reg<ch3ocfr::CH3OCFR_SPEC>;
#[doc = "CH3OCFR"]
pub mod ch3ocfr;
#[doc = "CHCTR (rw) register accessor: an alias for `Reg<CHCTR_SPEC>`"]
pub type CHCTR = crate::Reg<chctr::CHCTR_SPEC>;
#[doc = "CHCTR"]
pub mod chctr;
#[doc = "CHPOLR (rw) register accessor: an alias for `Reg<CHPOLR_SPEC>`"]
pub type CHPOLR = crate::Reg<chpolr::CHPOLR_SPEC>;
#[doc = "CHPOLR"]
pub mod chpolr;
#[doc = "CHBRKCFR (rw) register accessor: an alias for `Reg<CHBRKCFR_SPEC>`"]
pub type CHBRKCFR = crate::Reg<chbrkcfr::CHBRKCFR_SPEC>;
#[doc = "CHBRKCFR"]
pub mod chbrkcfr;
#[doc = "CHBRKCTR (rw) register accessor: an alias for `Reg<CHBRKCTR_SPEC>`"]
pub type CHBRKCTR = crate::Reg<chbrkctr::CHBRKCTR_SPEC>;
#[doc = "CHBRKCTR"]
pub mod chbrkctr;
#[doc = "DICTR (rw) register accessor: an alias for `Reg<DICTR_SPEC>`"]
pub type DICTR = crate::Reg<dictr::DICTR_SPEC>;
#[doc = "DICTR"]
pub mod dictr;
#[doc = "EVGR (rw) register accessor: an alias for `Reg<EVGR_SPEC>`"]
pub type EVGR = crate::Reg<evgr::EVGR_SPEC>;
#[doc = "EVGR"]
pub mod evgr;
#[doc = "INTSR (rw) register accessor: an alias for `Reg<INTSR_SPEC>`"]
pub type INTSR = crate::Reg<intsr::INTSR_SPEC>;
#[doc = "INTSR"]
pub mod intsr;
#[doc = "CNTR (rw) register accessor: an alias for `Reg<CNTR_SPEC>`"]
pub type CNTR = crate::Reg<cntr::CNTR_SPEC>;
#[doc = "CNTR"]
pub mod cntr;
#[doc = "PSCR (rw) register accessor: an alias for `Reg<PSCR_SPEC>`"]
pub type PSCR = crate::Reg<pscr::PSCR_SPEC>;
#[doc = "PSCR"]
pub mod pscr;
#[doc = "CRR (rw) register accessor: an alias for `Reg<CRR_SPEC>`"]
pub type CRR = crate::Reg<crr::CRR_SPEC>;
#[doc = "CRR"]
pub mod crr;
#[doc = "REPR (rw) register accessor: an alias for `Reg<REPR_SPEC>`"]
pub type REPR = crate::Reg<repr::REPR_SPEC>;
#[doc = "REPR"]
pub mod repr;
#[doc = "CH0CCR (rw) register accessor: an alias for `Reg<CH0CCR_SPEC>`"]
pub type CH0CCR = crate::Reg<ch0ccr::CH0CCR_SPEC>;
#[doc = "CH0CCR"]
pub mod ch0ccr;
#[doc = "CH1CCR (rw) register accessor: an alias for `Reg<CH1CCR_SPEC>`"]
pub type CH1CCR = crate::Reg<ch1ccr::CH1CCR_SPEC>;
#[doc = "CH1CCR"]
pub mod ch1ccr;
#[doc = "CH2CCR (rw) register accessor: an alias for `Reg<CH2CCR_SPEC>`"]
pub type CH2CCR = crate::Reg<ch2ccr::CH2CCR_SPEC>;
#[doc = "CH2CCR"]
pub mod ch2ccr;
#[doc = "CH3CCR (rw) register accessor: an alias for `Reg<CH3CCR_SPEC>`"]
pub type CH3CCR = crate::Reg<ch3ccr::CH3CCR_SPEC>;
#[doc = "CH3CCR"]
pub mod ch3ccr;
#[doc = "CH0ACR (rw) register accessor: an alias for `Reg<CH0ACR_SPEC>`"]
pub type CH0ACR = crate::Reg<ch0acr::CH0ACR_SPEC>;
#[doc = "CH0ACR"]
pub mod ch0acr;
#[doc = "CH1ACR (rw) register accessor: an alias for `Reg<CH1ACR_SPEC>`"]
pub type CH1ACR = crate::Reg<ch1acr::CH1ACR_SPEC>;
#[doc = "CH1ACR"]
pub mod ch1acr;
#[doc = "CH2ACR (rw) register accessor: an alias for `Reg<CH2ACR_SPEC>`"]
pub type CH2ACR = crate::Reg<ch2acr::CH2ACR_SPEC>;
#[doc = "CH2ACR"]
pub mod ch2acr;
#[doc = "CH3ACR (rw) register accessor: an alias for `Reg<CH3ACR_SPEC>`"]
pub type CH3ACR = crate::Reg<ch3acr::CH3ACR_SPEC>;
#[doc = "CH3ACR"]
pub mod ch3acr;
