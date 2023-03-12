#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPTM_CNTCFR"]
    pub gptm_cntcfr: GPTM_CNTCFR,
    #[doc = "0x04 - GPTM_MDCFR"]
    pub gptm_mdcfr: GPTM_MDCFR,
    #[doc = "0x08 - GPTM_TRCFR"]
    pub gptm_trcfr: GPTM_TRCFR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - GPTM_CTR"]
    pub gptm_ctr: GPTM_CTR,
    _reserved4: [u8; 0x0c],
    #[doc = "0x20 - GPTM_CH0ICFR"]
    pub gptm_ch0icfr: GPTM_CH0ICFR,
    #[doc = "0x24 - GPTM_CH1ICFR"]
    pub gptm_ch1icfr: GPTM_CH1ICFR,
    #[doc = "0x28 - GPTM_CH2ICFR"]
    pub gptm_ch2icfr: GPTM_CH2ICFR,
    #[doc = "0x2c - GPTM_CH3ICFR"]
    pub gptm_ch3icfr: GPTM_CH3ICFR,
    _reserved8: [u8; 0x10],
    #[doc = "0x40 - GPTM_CH0OCFR"]
    pub gptm_ch0ocfr: GPTM_CH0OCFR,
    #[doc = "0x44 - GPTM_CH1OCFR"]
    pub gptm_ch1ocfr: GPTM_CH1OCFR,
    #[doc = "0x48 - GPTM_CH2OCFR"]
    pub gptm_ch2ocfr: GPTM_CH2OCFR,
    #[doc = "0x4c - GPTM_CH3OCFR"]
    pub gptm_ch3ocfr: GPTM_CH3OCFR,
    #[doc = "0x50 - GPTM_CHCTR"]
    pub gptm_chctr: GPTM_CHCTR,
    #[doc = "0x54 - GPTM_CHPOLR"]
    pub gptm_chpolr: GPTM_CHPOLR,
    _reserved14: [u8; 0x1c],
    #[doc = "0x74 - GPTM_DICTR"]
    pub gptm_dictr: GPTM_DICTR,
    #[doc = "0x78 - GPTM_EVGR"]
    pub gptm_evgr: GPTM_EVGR,
    #[doc = "0x7c - GPTM_INTSR"]
    pub gptm_intsr: GPTM_INTSR,
    #[doc = "0x80 - GPTM_CNTR"]
    pub gptm_cntr: GPTM_CNTR,
    #[doc = "0x84 - GPTM_PSCR"]
    pub gptm_pscr: GPTM_PSCR,
    #[doc = "0x88 - GPTM_CRR"]
    pub gptm_crr: GPTM_CRR,
    _reserved20: [u8; 0x04],
    #[doc = "0x90 - GPTM_CH0CCR"]
    pub gptm_ch0ccr: GPTM_CH0CCR,
    #[doc = "0x94 - GPTM_CH1CCR"]
    pub gptm_ch1ccr: GPTM_CH1CCR,
    #[doc = "0x98 - GPTM_CH2CCR"]
    pub gptm_ch2ccr: GPTM_CH2CCR,
    #[doc = "0x9c - GPTM_CH3CCR"]
    pub gptm_ch3ccr: GPTM_CH3CCR,
    #[doc = "0xa0 - GPTM_CH0ACR"]
    pub gptm_ch0acr: GPTM_CH0ACR,
    #[doc = "0xa4 - GPTM_CH1ACR"]
    pub gptm_ch1acr: GPTM_CH1ACR,
    #[doc = "0xa8 - GPTM_CH2ACR"]
    pub gptm_ch2acr: GPTM_CH2ACR,
    #[doc = "0xac - GPTM_CH3ACR"]
    pub gptm_ch3acr: GPTM_CH3ACR,
}
#[doc = "GPTM_CNTCFR (rw) register accessor: an alias for `Reg<GPTM_CNTCFR_SPEC>`"]
pub type GPTM_CNTCFR = crate::Reg<gptm_cntcfr::GPTM_CNTCFR_SPEC>;
#[doc = "GPTM_CNTCFR"]
pub mod gptm_cntcfr;
#[doc = "GPTM_MDCFR (rw) register accessor: an alias for `Reg<GPTM_MDCFR_SPEC>`"]
pub type GPTM_MDCFR = crate::Reg<gptm_mdcfr::GPTM_MDCFR_SPEC>;
#[doc = "GPTM_MDCFR"]
pub mod gptm_mdcfr;
#[doc = "GPTM_TRCFR (rw) register accessor: an alias for `Reg<GPTM_TRCFR_SPEC>`"]
pub type GPTM_TRCFR = crate::Reg<gptm_trcfr::GPTM_TRCFR_SPEC>;
#[doc = "GPTM_TRCFR"]
pub mod gptm_trcfr;
#[doc = "GPTM_CTR (rw) register accessor: an alias for `Reg<GPTM_CTR_SPEC>`"]
pub type GPTM_CTR = crate::Reg<gptm_ctr::GPTM_CTR_SPEC>;
#[doc = "GPTM_CTR"]
pub mod gptm_ctr;
#[doc = "GPTM_CH0ICFR (rw) register accessor: an alias for `Reg<GPTM_CH0ICFR_SPEC>`"]
pub type GPTM_CH0ICFR = crate::Reg<gptm_ch0icfr::GPTM_CH0ICFR_SPEC>;
#[doc = "GPTM_CH0ICFR"]
pub mod gptm_ch0icfr;
#[doc = "GPTM_CH1ICFR (rw) register accessor: an alias for `Reg<GPTM_CH1ICFR_SPEC>`"]
pub type GPTM_CH1ICFR = crate::Reg<gptm_ch1icfr::GPTM_CH1ICFR_SPEC>;
#[doc = "GPTM_CH1ICFR"]
pub mod gptm_ch1icfr;
#[doc = "GPTM_CH2ICFR (rw) register accessor: an alias for `Reg<GPTM_CH2ICFR_SPEC>`"]
pub type GPTM_CH2ICFR = crate::Reg<gptm_ch2icfr::GPTM_CH2ICFR_SPEC>;
#[doc = "GPTM_CH2ICFR"]
pub mod gptm_ch2icfr;
#[doc = "GPTM_CH3ICFR (rw) register accessor: an alias for `Reg<GPTM_CH3ICFR_SPEC>`"]
pub type GPTM_CH3ICFR = crate::Reg<gptm_ch3icfr::GPTM_CH3ICFR_SPEC>;
#[doc = "GPTM_CH3ICFR"]
pub mod gptm_ch3icfr;
#[doc = "GPTM_CH0OCFR (rw) register accessor: an alias for `Reg<GPTM_CH0OCFR_SPEC>`"]
pub type GPTM_CH0OCFR = crate::Reg<gptm_ch0ocfr::GPTM_CH0OCFR_SPEC>;
#[doc = "GPTM_CH0OCFR"]
pub mod gptm_ch0ocfr;
#[doc = "GPTM_CH1OCFR (rw) register accessor: an alias for `Reg<GPTM_CH1OCFR_SPEC>`"]
pub type GPTM_CH1OCFR = crate::Reg<gptm_ch1ocfr::GPTM_CH1OCFR_SPEC>;
#[doc = "GPTM_CH1OCFR"]
pub mod gptm_ch1ocfr;
#[doc = "GPTM_CH2OCFR (rw) register accessor: an alias for `Reg<GPTM_CH2OCFR_SPEC>`"]
pub type GPTM_CH2OCFR = crate::Reg<gptm_ch2ocfr::GPTM_CH2OCFR_SPEC>;
#[doc = "GPTM_CH2OCFR"]
pub mod gptm_ch2ocfr;
#[doc = "GPTM_CH3OCFR (rw) register accessor: an alias for `Reg<GPTM_CH3OCFR_SPEC>`"]
pub type GPTM_CH3OCFR = crate::Reg<gptm_ch3ocfr::GPTM_CH3OCFR_SPEC>;
#[doc = "GPTM_CH3OCFR"]
pub mod gptm_ch3ocfr;
#[doc = "GPTM_CHCTR (rw) register accessor: an alias for `Reg<GPTM_CHCTR_SPEC>`"]
pub type GPTM_CHCTR = crate::Reg<gptm_chctr::GPTM_CHCTR_SPEC>;
#[doc = "GPTM_CHCTR"]
pub mod gptm_chctr;
#[doc = "GPTM_CHPOLR (rw) register accessor: an alias for `Reg<GPTM_CHPOLR_SPEC>`"]
pub type GPTM_CHPOLR = crate::Reg<gptm_chpolr::GPTM_CHPOLR_SPEC>;
#[doc = "GPTM_CHPOLR"]
pub mod gptm_chpolr;
#[doc = "GPTM_DICTR (rw) register accessor: an alias for `Reg<GPTM_DICTR_SPEC>`"]
pub type GPTM_DICTR = crate::Reg<gptm_dictr::GPTM_DICTR_SPEC>;
#[doc = "GPTM_DICTR"]
pub mod gptm_dictr;
#[doc = "GPTM_EVGR (rw) register accessor: an alias for `Reg<GPTM_EVGR_SPEC>`"]
pub type GPTM_EVGR = crate::Reg<gptm_evgr::GPTM_EVGR_SPEC>;
#[doc = "GPTM_EVGR"]
pub mod gptm_evgr;
#[doc = "GPTM_INTSR (rw) register accessor: an alias for `Reg<GPTM_INTSR_SPEC>`"]
pub type GPTM_INTSR = crate::Reg<gptm_intsr::GPTM_INTSR_SPEC>;
#[doc = "GPTM_INTSR"]
pub mod gptm_intsr;
#[doc = "GPTM_CNTR (rw) register accessor: an alias for `Reg<GPTM_CNTR_SPEC>`"]
pub type GPTM_CNTR = crate::Reg<gptm_cntr::GPTM_CNTR_SPEC>;
#[doc = "GPTM_CNTR"]
pub mod gptm_cntr;
#[doc = "GPTM_PSCR (rw) register accessor: an alias for `Reg<GPTM_PSCR_SPEC>`"]
pub type GPTM_PSCR = crate::Reg<gptm_pscr::GPTM_PSCR_SPEC>;
#[doc = "GPTM_PSCR"]
pub mod gptm_pscr;
#[doc = "GPTM_CRR (rw) register accessor: an alias for `Reg<GPTM_CRR_SPEC>`"]
pub type GPTM_CRR = crate::Reg<gptm_crr::GPTM_CRR_SPEC>;
#[doc = "GPTM_CRR"]
pub mod gptm_crr;
#[doc = "GPTM_CH0CCR (rw) register accessor: an alias for `Reg<GPTM_CH0CCR_SPEC>`"]
pub type GPTM_CH0CCR = crate::Reg<gptm_ch0ccr::GPTM_CH0CCR_SPEC>;
#[doc = "GPTM_CH0CCR"]
pub mod gptm_ch0ccr;
#[doc = "GPTM_CH1CCR (rw) register accessor: an alias for `Reg<GPTM_CH1CCR_SPEC>`"]
pub type GPTM_CH1CCR = crate::Reg<gptm_ch1ccr::GPTM_CH1CCR_SPEC>;
#[doc = "GPTM_CH1CCR"]
pub mod gptm_ch1ccr;
#[doc = "GPTM_CH2CCR (rw) register accessor: an alias for `Reg<GPTM_CH2CCR_SPEC>`"]
pub type GPTM_CH2CCR = crate::Reg<gptm_ch2ccr::GPTM_CH2CCR_SPEC>;
#[doc = "GPTM_CH2CCR"]
pub mod gptm_ch2ccr;
#[doc = "GPTM_CH3CCR (rw) register accessor: an alias for `Reg<GPTM_CH3CCR_SPEC>`"]
pub type GPTM_CH3CCR = crate::Reg<gptm_ch3ccr::GPTM_CH3CCR_SPEC>;
#[doc = "GPTM_CH3CCR"]
pub mod gptm_ch3ccr;
#[doc = "GPTM_CH0ACR (rw) register accessor: an alias for `Reg<GPTM_CH0ACR_SPEC>`"]
pub type GPTM_CH0ACR = crate::Reg<gptm_ch0acr::GPTM_CH0ACR_SPEC>;
#[doc = "GPTM_CH0ACR"]
pub mod gptm_ch0acr;
#[doc = "GPTM_CH1ACR (rw) register accessor: an alias for `Reg<GPTM_CH1ACR_SPEC>`"]
pub type GPTM_CH1ACR = crate::Reg<gptm_ch1acr::GPTM_CH1ACR_SPEC>;
#[doc = "GPTM_CH1ACR"]
pub mod gptm_ch1acr;
#[doc = "GPTM_CH2ACR (rw) register accessor: an alias for `Reg<GPTM_CH2ACR_SPEC>`"]
pub type GPTM_CH2ACR = crate::Reg<gptm_ch2acr::GPTM_CH2ACR_SPEC>;
#[doc = "GPTM_CH2ACR"]
pub mod gptm_ch2acr;
#[doc = "GPTM_CH3ACR (rw) register accessor: an alias for `Reg<GPTM_CH3ACR_SPEC>`"]
pub type GPTM_CH3ACR = crate::Reg<gptm_ch3acr::GPTM_CH3ACR_SPEC>;
#[doc = "GPTM_CH3ACR"]
pub mod gptm_ch3acr;
