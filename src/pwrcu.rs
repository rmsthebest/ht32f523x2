#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWRCU_BAKSR"]
    pub pwrcu_baksr: PWRCU_BAKSR,
    #[doc = "0x04 - PWRCU_BAKCR"]
    pub pwrcu_bakcr: PWRCU_BAKCR,
    #[doc = "0x08 - PWRCU_BAKTEST"]
    pub pwrcu_baktest: PWRCU_BAKTEST,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - PWRCU_LVDCSR"]
    pub pwrcu_lvdcsr: PWRCU_LVDCSR,
    _reserved4: [u8; 0xec],
    #[doc = "0x100 - PWRCU_BAKREG0"]
    pub pwrcu_bakreg0: PWRCU_BAKREG0,
    #[doc = "0x104 - PWRCU_BAKREG1"]
    pub pwrcu_bakreg1: PWRCU_BAKREG1,
    #[doc = "0x108 - PWRCU_BAKREG2"]
    pub pwrcu_bakreg2: PWRCU_BAKREG2,
    #[doc = "0x10c - PWRCU_BAKREG3"]
    pub pwrcu_bakreg3: PWRCU_BAKREG3,
    #[doc = "0x110 - PWRCU_BAKREG4"]
    pub pwrcu_bakreg4: PWRCU_BAKREG4,
    #[doc = "0x114 - PWRCU_BAKREG5"]
    pub pwrcu_bakreg5: PWRCU_BAKREG5,
    #[doc = "0x118 - PWRCU_BAKREG6"]
    pub pwrcu_bakreg6: PWRCU_BAKREG6,
    #[doc = "0x11c - PWRCU_BAKREG7"]
    pub pwrcu_bakreg7: PWRCU_BAKREG7,
    #[doc = "0x120 - PWRCU_BAKREG8"]
    pub pwrcu_bakreg8: PWRCU_BAKREG8,
    #[doc = "0x124 - PWRCU_BAKREG9"]
    pub pwrcu_bakreg9: PWRCU_BAKREG9,
}
#[doc = "PWRCU_BAKSR (rw) register accessor: an alias for `Reg<PWRCU_BAKSR_SPEC>`"]
pub type PWRCU_BAKSR = crate::Reg<pwrcu_baksr::PWRCU_BAKSR_SPEC>;
#[doc = "PWRCU_BAKSR"]
pub mod pwrcu_baksr;
#[doc = "PWRCU_BAKCR (rw) register accessor: an alias for `Reg<PWRCU_BAKCR_SPEC>`"]
pub type PWRCU_BAKCR = crate::Reg<pwrcu_bakcr::PWRCU_BAKCR_SPEC>;
#[doc = "PWRCU_BAKCR"]
pub mod pwrcu_bakcr;
#[doc = "PWRCU_BAKTEST (rw) register accessor: an alias for `Reg<PWRCU_BAKTEST_SPEC>`"]
pub type PWRCU_BAKTEST = crate::Reg<pwrcu_baktest::PWRCU_BAKTEST_SPEC>;
#[doc = "PWRCU_BAKTEST"]
pub mod pwrcu_baktest;
#[doc = "PWRCU_LVDCSR (rw) register accessor: an alias for `Reg<PWRCU_LVDCSR_SPEC>`"]
pub type PWRCU_LVDCSR = crate::Reg<pwrcu_lvdcsr::PWRCU_LVDCSR_SPEC>;
#[doc = "PWRCU_LVDCSR"]
pub mod pwrcu_lvdcsr;
#[doc = "PWRCU_BAKREG0 (rw) register accessor: an alias for `Reg<PWRCU_BAKREG0_SPEC>`"]
pub type PWRCU_BAKREG0 = crate::Reg<pwrcu_bakreg0::PWRCU_BAKREG0_SPEC>;
#[doc = "PWRCU_BAKREG0"]
pub mod pwrcu_bakreg0;
#[doc = "PWRCU_BAKREG1 (rw) register accessor: an alias for `Reg<PWRCU_BAKREG1_SPEC>`"]
pub type PWRCU_BAKREG1 = crate::Reg<pwrcu_bakreg1::PWRCU_BAKREG1_SPEC>;
#[doc = "PWRCU_BAKREG1"]
pub mod pwrcu_bakreg1;
#[doc = "PWRCU_BAKREG2 (rw) register accessor: an alias for `Reg<PWRCU_BAKREG2_SPEC>`"]
pub type PWRCU_BAKREG2 = crate::Reg<pwrcu_bakreg2::PWRCU_BAKREG2_SPEC>;
#[doc = "PWRCU_BAKREG2"]
pub mod pwrcu_bakreg2;
#[doc = "PWRCU_BAKREG3 (rw) register accessor: an alias for `Reg<PWRCU_BAKREG3_SPEC>`"]
pub type PWRCU_BAKREG3 = crate::Reg<pwrcu_bakreg3::PWRCU_BAKREG3_SPEC>;
#[doc = "PWRCU_BAKREG3"]
pub mod pwrcu_bakreg3;
#[doc = "PWRCU_BAKREG4 (rw) register accessor: an alias for `Reg<PWRCU_BAKREG4_SPEC>`"]
pub type PWRCU_BAKREG4 = crate::Reg<pwrcu_bakreg4::PWRCU_BAKREG4_SPEC>;
#[doc = "PWRCU_BAKREG4"]
pub mod pwrcu_bakreg4;
#[doc = "PWRCU_BAKREG5 (rw) register accessor: an alias for `Reg<PWRCU_BAKREG5_SPEC>`"]
pub type PWRCU_BAKREG5 = crate::Reg<pwrcu_bakreg5::PWRCU_BAKREG5_SPEC>;
#[doc = "PWRCU_BAKREG5"]
pub mod pwrcu_bakreg5;
#[doc = "PWRCU_BAKREG6 (rw) register accessor: an alias for `Reg<PWRCU_BAKREG6_SPEC>`"]
pub type PWRCU_BAKREG6 = crate::Reg<pwrcu_bakreg6::PWRCU_BAKREG6_SPEC>;
#[doc = "PWRCU_BAKREG6"]
pub mod pwrcu_bakreg6;
#[doc = "PWRCU_BAKREG7 (rw) register accessor: an alias for `Reg<PWRCU_BAKREG7_SPEC>`"]
pub type PWRCU_BAKREG7 = crate::Reg<pwrcu_bakreg7::PWRCU_BAKREG7_SPEC>;
#[doc = "PWRCU_BAKREG7"]
pub mod pwrcu_bakreg7;
#[doc = "PWRCU_BAKREG8 (rw) register accessor: an alias for `Reg<PWRCU_BAKREG8_SPEC>`"]
pub type PWRCU_BAKREG8 = crate::Reg<pwrcu_bakreg8::PWRCU_BAKREG8_SPEC>;
#[doc = "PWRCU_BAKREG8"]
pub mod pwrcu_bakreg8;
#[doc = "PWRCU_BAKREG9 (rw) register accessor: an alias for `Reg<PWRCU_BAKREG9_SPEC>`"]
pub type PWRCU_BAKREG9 = crate::Reg<pwrcu_bakreg9::PWRCU_BAKREG9_SPEC>;
#[doc = "PWRCU_BAKREG9"]
pub mod pwrcu_bakreg9;
