#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CFGR0"]
    pub cfgr0: CFGR0,
    #[doc = "0x04 - CFGR1"]
    pub cfgr1: CFGR1,
    #[doc = "0x08 - CFGR2"]
    pub cfgr2: CFGR2,
    #[doc = "0x0c - CFGR3"]
    pub cfgr3: CFGR3,
    #[doc = "0x10 - CFGR4"]
    pub cfgr4: CFGR4,
    #[doc = "0x14 - CFGR5"]
    pub cfgr5: CFGR5,
    #[doc = "0x18 - CFGR6"]
    pub cfgr6: CFGR6,
    #[doc = "0x1c - CFGR7"]
    pub cfgr7: CFGR7,
    #[doc = "0x20 - CFGR8"]
    pub cfgr8: CFGR8,
    #[doc = "0x24 - CFGR9"]
    pub cfgr9: CFGR9,
    #[doc = "0x28 - CFGR10"]
    pub cfgr10: CFGR10,
    #[doc = "0x2c - CFGR11"]
    pub cfgr11: CFGR11,
    #[doc = "0x30 - CFGR12"]
    pub cfgr12: CFGR12,
    #[doc = "0x34 - CFGR13"]
    pub cfgr13: CFGR13,
    #[doc = "0x38 - CFGR14"]
    pub cfgr14: CFGR14,
    #[doc = "0x3c - CFGR15"]
    pub cfgr15: CFGR15,
    #[doc = "0x40 - CR"]
    pub cr: CR,
    #[doc = "0x44 - EDGEFLGR"]
    pub edgeflgr: EDGEFLGR,
    #[doc = "0x48 - EDGESR"]
    pub edgesr: EDGESR,
    #[doc = "0x4c - SSCR"]
    pub sscr: SSCR,
    #[doc = "0x50 - WAKUPCR"]
    pub wakupcr: WAKUPCR,
    #[doc = "0x54 - WAKUPPOLR"]
    pub wakuppolr: WAKUPPOLR,
    #[doc = "0x58 - WAKUPFLG"]
    pub wakupflg: WAKUPFLG,
}
#[doc = "CFGR0 (rw) register accessor: an alias for `Reg<CFGR0_SPEC>`"]
pub type CFGR0 = crate::Reg<cfgr0::CFGR0_SPEC>;
#[doc = "CFGR0"]
pub mod cfgr0;
#[doc = "CFGR1 (rw) register accessor: an alias for `Reg<CFGR1_SPEC>`"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
#[doc = "CFGR1"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "CFGR2"]
pub mod cfgr2;
#[doc = "CFGR3 (rw) register accessor: an alias for `Reg<CFGR3_SPEC>`"]
pub type CFGR3 = crate::Reg<cfgr3::CFGR3_SPEC>;
#[doc = "CFGR3"]
pub mod cfgr3;
#[doc = "CFGR4 (rw) register accessor: an alias for `Reg<CFGR4_SPEC>`"]
pub type CFGR4 = crate::Reg<cfgr4::CFGR4_SPEC>;
#[doc = "CFGR4"]
pub mod cfgr4;
#[doc = "CFGR5 (rw) register accessor: an alias for `Reg<CFGR5_SPEC>`"]
pub type CFGR5 = crate::Reg<cfgr5::CFGR5_SPEC>;
#[doc = "CFGR5"]
pub mod cfgr5;
#[doc = "CFGR6 (rw) register accessor: an alias for `Reg<CFGR6_SPEC>`"]
pub type CFGR6 = crate::Reg<cfgr6::CFGR6_SPEC>;
#[doc = "CFGR6"]
pub mod cfgr6;
#[doc = "CFGR7 (rw) register accessor: an alias for `Reg<CFGR7_SPEC>`"]
pub type CFGR7 = crate::Reg<cfgr7::CFGR7_SPEC>;
#[doc = "CFGR7"]
pub mod cfgr7;
#[doc = "CFGR8 (rw) register accessor: an alias for `Reg<CFGR8_SPEC>`"]
pub type CFGR8 = crate::Reg<cfgr8::CFGR8_SPEC>;
#[doc = "CFGR8"]
pub mod cfgr8;
#[doc = "CFGR9 (rw) register accessor: an alias for `Reg<CFGR9_SPEC>`"]
pub type CFGR9 = crate::Reg<cfgr9::CFGR9_SPEC>;
#[doc = "CFGR9"]
pub mod cfgr9;
#[doc = "CFGR10 (rw) register accessor: an alias for `Reg<CFGR10_SPEC>`"]
pub type CFGR10 = crate::Reg<cfgr10::CFGR10_SPEC>;
#[doc = "CFGR10"]
pub mod cfgr10;
#[doc = "CFGR11 (rw) register accessor: an alias for `Reg<CFGR11_SPEC>`"]
pub type CFGR11 = crate::Reg<cfgr11::CFGR11_SPEC>;
#[doc = "CFGR11"]
pub mod cfgr11;
#[doc = "CFGR12 (rw) register accessor: an alias for `Reg<CFGR12_SPEC>`"]
pub type CFGR12 = crate::Reg<cfgr12::CFGR12_SPEC>;
#[doc = "CFGR12"]
pub mod cfgr12;
#[doc = "CFGR13 (rw) register accessor: an alias for `Reg<CFGR13_SPEC>`"]
pub type CFGR13 = crate::Reg<cfgr13::CFGR13_SPEC>;
#[doc = "CFGR13"]
pub mod cfgr13;
#[doc = "CFGR14 (rw) register accessor: an alias for `Reg<CFGR14_SPEC>`"]
pub type CFGR14 = crate::Reg<cfgr14::CFGR14_SPEC>;
#[doc = "CFGR14"]
pub mod cfgr14;
#[doc = "CFGR15 (rw) register accessor: an alias for `Reg<CFGR15_SPEC>`"]
pub type CFGR15 = crate::Reg<cfgr15::CFGR15_SPEC>;
#[doc = "CFGR15"]
pub mod cfgr15;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR"]
pub mod cr;
#[doc = "EDGEFLGR (rw) register accessor: an alias for `Reg<EDGEFLGR_SPEC>`"]
pub type EDGEFLGR = crate::Reg<edgeflgr::EDGEFLGR_SPEC>;
#[doc = "EDGEFLGR"]
pub mod edgeflgr;
#[doc = "EDGESR (rw) register accessor: an alias for `Reg<EDGESR_SPEC>`"]
pub type EDGESR = crate::Reg<edgesr::EDGESR_SPEC>;
#[doc = "EDGESR"]
pub mod edgesr;
#[doc = "SSCR (rw) register accessor: an alias for `Reg<SSCR_SPEC>`"]
pub type SSCR = crate::Reg<sscr::SSCR_SPEC>;
#[doc = "SSCR"]
pub mod sscr;
#[doc = "WAKUPCR (rw) register accessor: an alias for `Reg<WAKUPCR_SPEC>`"]
pub type WAKUPCR = crate::Reg<wakupcr::WAKUPCR_SPEC>;
#[doc = "WAKUPCR"]
pub mod wakupcr;
#[doc = "WAKUPPOLR (rw) register accessor: an alias for `Reg<WAKUPPOLR_SPEC>`"]
pub type WAKUPPOLR = crate::Reg<wakuppolr::WAKUPPOLR_SPEC>;
#[doc = "WAKUPPOLR"]
pub mod wakuppolr;
#[doc = "WAKUPFLG (rw) register accessor: an alias for `Reg<WAKUPFLG_SPEC>`"]
pub type WAKUPFLG = crate::Reg<wakupflg::WAKUPFLG_SPEC>;
#[doc = "WAKUPFLG"]
pub mod wakupflg;
