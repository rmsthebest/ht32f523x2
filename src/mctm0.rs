#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCTM_CNTCFR"]
    pub mctm_cntcfr: MCTM_CNTCFR,
    #[doc = "0x04 - MCTM_MDCFR"]
    pub mctm_mdcfr: MCTM_MDCFR,
    #[doc = "0x08 - MCTM_TRCFR"]
    pub mctm_trcfr: MCTM_TRCFR,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - MCTM_CTR"]
    pub mctm_ctr: MCTM_CTR,
    _reserved1: [u8; 12usize],
    #[doc = "0x20 - MCTM_CH0ICFR"]
    pub mctm_ch0icfr: MCTM_CH0ICFR,
    #[doc = "0x24 - MCTM_CH1ICFR"]
    pub mctm_ch1icfr: MCTM_CH1ICFR,
    #[doc = "0x28 - MCTM_CH2ICFR"]
    pub mctm_ch2icfr: MCTM_CH2ICFR,
    #[doc = "0x2c - MCTM_CH3ICFR"]
    pub mctm_ch3icfr: MCTM_CH3ICFR,
    _reserved2: [u8; 16usize],
    #[doc = "0x40 - MCTM_CH0OCFR"]
    pub mctm_ch0ocfr: MCTM_CH0OCFR,
    #[doc = "0x44 - MCTM_CH1OCFR"]
    pub mctm_ch1ocfr: MCTM_CH1OCFR,
    #[doc = "0x48 - MCTM_CH2OCFR"]
    pub mctm_ch2ocfr: MCTM_CH2OCFR,
    #[doc = "0x4c - MCTM_CH3OCFR"]
    pub mctm_ch3ocfr: MCTM_CH3OCFR,
    #[doc = "0x50 - MCTM_CHCTR"]
    pub mctm_chctr: MCTM_CHCTR,
    #[doc = "0x54 - MCTM_CHPOLR"]
    pub mctm_chpolr: MCTM_CHPOLR,
    _reserved3: [u8; 20usize],
    #[doc = "0x6c - MCTM_CHBRKCFR"]
    pub mctm_chbrkcfr: MCTM_CHBRKCFR,
    #[doc = "0x70 - MCTM_CHBRKCTR"]
    pub mctm_chbrkctr: MCTM_CHBRKCTR,
    #[doc = "0x74 - MCTM_DICTR"]
    pub mctm_dictr: MCTM_DICTR,
    #[doc = "0x78 - MCTM_EVGR"]
    pub mctm_evgr: MCTM_EVGR,
    #[doc = "0x7c - MCTM_INTSR"]
    pub mctm_intsr: MCTM_INTSR,
    #[doc = "0x80 - MCTM_CNTR"]
    pub mctm_cntr: MCTM_CNTR,
    #[doc = "0x84 - MCTM_PSCR"]
    pub mctm_pscr: MCTM_PSCR,
    #[doc = "0x88 - MCTM_CRR"]
    pub mctm_crr: MCTM_CRR,
    #[doc = "0x8c - MCTM_REPR"]
    pub mctm_repr: MCTM_REPR,
    #[doc = "0x90 - MCTM_CH0CCR"]
    pub mctm_ch0ccr: MCTM_CH0CCR,
    #[doc = "0x94 - MCTM_CH1CCR"]
    pub mctm_ch1ccr: MCTM_CH1CCR,
    #[doc = "0x98 - MCTM_CH2CCR"]
    pub mctm_ch2ccr: MCTM_CH2CCR,
    #[doc = "0x9c - MCTM_CH3CCR"]
    pub mctm_ch3ccr: MCTM_CH3CCR,
    #[doc = "0xa0 - MCTM_CH0ACR"]
    pub mctm_ch0acr: MCTM_CH0ACR,
    #[doc = "0xa4 - MCTM_CH1ACR"]
    pub mctm_ch1acr: MCTM_CH1ACR,
    #[doc = "0xa8 - MCTM_CH2ACR"]
    pub mctm_ch2acr: MCTM_CH2ACR,
    #[doc = "0xac - MCTM_CH3ACR"]
    pub mctm_ch3acr: MCTM_CH3ACR,
}
#[doc = "MCTM_CNTCFR"]
pub struct MCTM_CNTCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_CNTCFR"]
pub mod mctm_cntcfr;
#[doc = "MCTM_MDCFR"]
pub struct MCTM_MDCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_MDCFR"]
pub mod mctm_mdcfr;
#[doc = "MCTM_TRCFR"]
pub struct MCTM_TRCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_TRCFR"]
pub mod mctm_trcfr;
#[doc = "MCTM_CTR"]
pub struct MCTM_CTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_CTR"]
pub mod mctm_ctr;
#[doc = "MCTM_CH0ICFR"]
pub struct MCTM_CH0ICFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_CH0ICFR"]
pub mod mctm_ch0icfr;
#[doc = "MCTM_CH1ICFR"]
pub struct MCTM_CH1ICFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_CH1ICFR"]
pub mod mctm_ch1icfr;
#[doc = "MCTM_CH2ICFR"]
pub struct MCTM_CH2ICFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_CH2ICFR"]
pub mod mctm_ch2icfr;
#[doc = "MCTM_CH3ICFR"]
pub struct MCTM_CH3ICFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_CH3ICFR"]
pub mod mctm_ch3icfr;
#[doc = "MCTM_CH0OCFR"]
pub struct MCTM_CH0OCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_CH0OCFR"]
pub mod mctm_ch0ocfr;
#[doc = "MCTM_CH1OCFR"]
pub struct MCTM_CH1OCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_CH1OCFR"]
pub mod mctm_ch1ocfr;
#[doc = "MCTM_CH2OCFR"]
pub struct MCTM_CH2OCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_CH2OCFR"]
pub mod mctm_ch2ocfr;
#[doc = "MCTM_CH3OCFR"]
pub struct MCTM_CH3OCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_CH3OCFR"]
pub mod mctm_ch3ocfr;
#[doc = "MCTM_CHCTR"]
pub struct MCTM_CHCTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_CHCTR"]
pub mod mctm_chctr;
#[doc = "MCTM_CHPOLR"]
pub struct MCTM_CHPOLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_CHPOLR"]
pub mod mctm_chpolr;
#[doc = "MCTM_CHBRKCFR"]
pub struct MCTM_CHBRKCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_CHBRKCFR"]
pub mod mctm_chbrkcfr;
#[doc = "MCTM_CHBRKCTR"]
pub struct MCTM_CHBRKCTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_CHBRKCTR"]
pub mod mctm_chbrkctr;
#[doc = "MCTM_DICTR"]
pub struct MCTM_DICTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_DICTR"]
pub mod mctm_dictr;
#[doc = "MCTM_EVGR"]
pub struct MCTM_EVGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_EVGR"]
pub mod mctm_evgr;
#[doc = "MCTM_INTSR"]
pub struct MCTM_INTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_INTSR"]
pub mod mctm_intsr;
#[doc = "MCTM_CNTR"]
pub struct MCTM_CNTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_CNTR"]
pub mod mctm_cntr;
#[doc = "MCTM_PSCR"]
pub struct MCTM_PSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_PSCR"]
pub mod mctm_pscr;
#[doc = "MCTM_CRR"]
pub struct MCTM_CRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_CRR"]
pub mod mctm_crr;
#[doc = "MCTM_REPR"]
pub struct MCTM_REPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_REPR"]
pub mod mctm_repr;
#[doc = "MCTM_CH0CCR"]
pub struct MCTM_CH0CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_CH0CCR"]
pub mod mctm_ch0ccr;
#[doc = "MCTM_CH1CCR"]
pub struct MCTM_CH1CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_CH1CCR"]
pub mod mctm_ch1ccr;
#[doc = "MCTM_CH2CCR"]
pub struct MCTM_CH2CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_CH2CCR"]
pub mod mctm_ch2ccr;
#[doc = "MCTM_CH3CCR"]
pub struct MCTM_CH3CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_CH3CCR"]
pub mod mctm_ch3ccr;
#[doc = "MCTM_CH0ACR"]
pub struct MCTM_CH0ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_CH0ACR"]
pub mod mctm_ch0acr;
#[doc = "MCTM_CH1ACR"]
pub struct MCTM_CH1ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_CH1ACR"]
pub mod mctm_ch1acr;
#[doc = "MCTM_CH2ACR"]
pub struct MCTM_CH2ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_CH2ACR"]
pub mod mctm_ch2acr;
#[doc = "MCTM_CH3ACR"]
pub struct MCTM_CH3ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCTM_CH3ACR"]
pub mod mctm_ch3acr;
