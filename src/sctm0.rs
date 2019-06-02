#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CNTCFR"]
    pub cntcfr: CNTCFR,
    #[doc = "0x04 - GPTM_MDCFR"]
    pub gptm_mdcfr: GPTM_MDCFR,
    #[doc = "0x08 - GPTM_TRCFR"]
    pub gptm_trcfr: GPTM_TRCFR,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - GPTM_CTR"]
    pub gptm_ctr: GPTM_CTR,
    _reserved1: [u8; 12usize],
    #[doc = "0x20 - GPTM_CH0ICFR"]
    pub gptm_ch0icfr: GPTM_CH0ICFR,
    _reserved2: [u8; 32usize],
    #[doc = "0x44 - GPTM_CH1OCFR"]
    pub gptm_ch1ocfr: GPTM_CH1OCFR,
    _reserved3: [u8; 8usize],
    #[doc = "0x50 - GPTM_CHCTR"]
    pub gptm_chctr: GPTM_CHCTR,
    #[doc = "0x54 - GPTM_CHPOLR"]
    pub gptm_chpolr: GPTM_CHPOLR,
    _reserved4: [u8; 28usize],
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
    _reserved5: [u8; 4usize],
    #[doc = "0x90 - GPTM_CH0CCR"]
    pub gptm_ch0ccr: GPTM_CH0CCR,
}
#[doc = "CNTCFR"]
pub struct CNTCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNTCFR"]
pub mod cntcfr;
#[doc = "GPTM_MDCFR"]
pub struct GPTM_MDCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_MDCFR"]
pub mod gptm_mdcfr;
#[doc = "GPTM_TRCFR"]
pub struct GPTM_TRCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_TRCFR"]
pub mod gptm_trcfr;
#[doc = "GPTM_CTR"]
pub struct GPTM_CTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CTR"]
pub mod gptm_ctr;
#[doc = "GPTM_CH0ICFR"]
pub struct GPTM_CH0ICFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CH0ICFR"]
pub mod gptm_ch0icfr;
#[doc = "GPTM_CH1OCFR"]
pub struct GPTM_CH1OCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CH1OCFR"]
pub mod gptm_ch1ocfr;
#[doc = "GPTM_CHCTR"]
pub struct GPTM_CHCTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CHCTR"]
pub mod gptm_chctr;
#[doc = "GPTM_CHPOLR"]
pub struct GPTM_CHPOLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CHPOLR"]
pub mod gptm_chpolr;
#[doc = "GPTM_DICTR"]
pub struct GPTM_DICTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_DICTR"]
pub mod gptm_dictr;
#[doc = "GPTM_EVGR"]
pub struct GPTM_EVGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_EVGR"]
pub mod gptm_evgr;
#[doc = "GPTM_INTSR"]
pub struct GPTM_INTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_INTSR"]
pub mod gptm_intsr;
#[doc = "GPTM_CNTR"]
pub struct GPTM_CNTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CNTR"]
pub mod gptm_cntr;
#[doc = "GPTM_PSCR"]
pub struct GPTM_PSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_PSCR"]
pub mod gptm_pscr;
#[doc = "GPTM_CRR"]
pub struct GPTM_CRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CRR"]
pub mod gptm_crr;
#[doc = "GPTM_CH0CCR"]
pub struct GPTM_CH0CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CH0CCR"]
pub mod gptm_ch0ccr;
