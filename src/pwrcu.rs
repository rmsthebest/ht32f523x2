#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pwrcu_baksr: PwrcuBaksr,
    pwrcu_bakcr: PwrcuBakcr,
    pwrcu_baktest: PwrcuBaktest,
    _reserved3: [u8; 0x04],
    pwrcu_lvdcsr: PwrcuLvdcsr,
    _reserved4: [u8; 0xec],
    pwrcu_bakreg0: PwrcuBakreg0,
    pwrcu_bakreg1: PwrcuBakreg1,
    pwrcu_bakreg2: PwrcuBakreg2,
    pwrcu_bakreg3: PwrcuBakreg3,
    pwrcu_bakreg4: PwrcuBakreg4,
    pwrcu_bakreg5: PwrcuBakreg5,
    pwrcu_bakreg6: PwrcuBakreg6,
    pwrcu_bakreg7: PwrcuBakreg7,
    pwrcu_bakreg8: PwrcuBakreg8,
    pwrcu_bakreg9: PwrcuBakreg9,
}
impl RegisterBlock {
    #[doc = "0x00 - PWRCU_BAKSR"]
    #[inline(always)]
    pub const fn pwrcu_baksr(&self) -> &PwrcuBaksr {
        &self.pwrcu_baksr
    }
    #[doc = "0x04 - PWRCU_BAKCR"]
    #[inline(always)]
    pub const fn pwrcu_bakcr(&self) -> &PwrcuBakcr {
        &self.pwrcu_bakcr
    }
    #[doc = "0x08 - PWRCU_BAKTEST"]
    #[inline(always)]
    pub const fn pwrcu_baktest(&self) -> &PwrcuBaktest {
        &self.pwrcu_baktest
    }
    #[doc = "0x10 - PWRCU_LVDCSR"]
    #[inline(always)]
    pub const fn pwrcu_lvdcsr(&self) -> &PwrcuLvdcsr {
        &self.pwrcu_lvdcsr
    }
    #[doc = "0x100 - PWRCU_BAKREG0"]
    #[inline(always)]
    pub const fn pwrcu_bakreg0(&self) -> &PwrcuBakreg0 {
        &self.pwrcu_bakreg0
    }
    #[doc = "0x104 - PWRCU_BAKREG1"]
    #[inline(always)]
    pub const fn pwrcu_bakreg1(&self) -> &PwrcuBakreg1 {
        &self.pwrcu_bakreg1
    }
    #[doc = "0x108 - PWRCU_BAKREG2"]
    #[inline(always)]
    pub const fn pwrcu_bakreg2(&self) -> &PwrcuBakreg2 {
        &self.pwrcu_bakreg2
    }
    #[doc = "0x10c - PWRCU_BAKREG3"]
    #[inline(always)]
    pub const fn pwrcu_bakreg3(&self) -> &PwrcuBakreg3 {
        &self.pwrcu_bakreg3
    }
    #[doc = "0x110 - PWRCU_BAKREG4"]
    #[inline(always)]
    pub const fn pwrcu_bakreg4(&self) -> &PwrcuBakreg4 {
        &self.pwrcu_bakreg4
    }
    #[doc = "0x114 - PWRCU_BAKREG5"]
    #[inline(always)]
    pub const fn pwrcu_bakreg5(&self) -> &PwrcuBakreg5 {
        &self.pwrcu_bakreg5
    }
    #[doc = "0x118 - PWRCU_BAKREG6"]
    #[inline(always)]
    pub const fn pwrcu_bakreg6(&self) -> &PwrcuBakreg6 {
        &self.pwrcu_bakreg6
    }
    #[doc = "0x11c - PWRCU_BAKREG7"]
    #[inline(always)]
    pub const fn pwrcu_bakreg7(&self) -> &PwrcuBakreg7 {
        &self.pwrcu_bakreg7
    }
    #[doc = "0x120 - PWRCU_BAKREG8"]
    #[inline(always)]
    pub const fn pwrcu_bakreg8(&self) -> &PwrcuBakreg8 {
        &self.pwrcu_bakreg8
    }
    #[doc = "0x124 - PWRCU_BAKREG9"]
    #[inline(always)]
    pub const fn pwrcu_bakreg9(&self) -> &PwrcuBakreg9 {
        &self.pwrcu_bakreg9
    }
}
#[doc = "PWRCU_BAKSR (rw) register accessor: PWRCU_BAKSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcu_baksr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcu_baksr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrcu_baksr`]
module"]
#[doc(alias = "PWRCU_BAKSR")]
pub type PwrcuBaksr = crate::Reg<pwrcu_baksr::PwrcuBaksrSpec>;
#[doc = "PWRCU_BAKSR"]
pub mod pwrcu_baksr;
#[doc = "PWRCU_BAKCR (rw) register accessor: PWRCU_BAKCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcu_bakcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcu_bakcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrcu_bakcr`]
module"]
#[doc(alias = "PWRCU_BAKCR")]
pub type PwrcuBakcr = crate::Reg<pwrcu_bakcr::PwrcuBakcrSpec>;
#[doc = "PWRCU_BAKCR"]
pub mod pwrcu_bakcr;
#[doc = "PWRCU_BAKTEST (rw) register accessor: PWRCU_BAKTEST\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcu_baktest::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcu_baktest::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrcu_baktest`]
module"]
#[doc(alias = "PWRCU_BAKTEST")]
pub type PwrcuBaktest = crate::Reg<pwrcu_baktest::PwrcuBaktestSpec>;
#[doc = "PWRCU_BAKTEST"]
pub mod pwrcu_baktest;
#[doc = "PWRCU_LVDCSR (rw) register accessor: PWRCU_LVDCSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcu_lvdcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcu_lvdcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrcu_lvdcsr`]
module"]
#[doc(alias = "PWRCU_LVDCSR")]
pub type PwrcuLvdcsr = crate::Reg<pwrcu_lvdcsr::PwrcuLvdcsrSpec>;
#[doc = "PWRCU_LVDCSR"]
pub mod pwrcu_lvdcsr;
#[doc = "PWRCU_BAKREG0 (rw) register accessor: PWRCU_BAKREG0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcu_bakreg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcu_bakreg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrcu_bakreg0`]
module"]
#[doc(alias = "PWRCU_BAKREG0")]
pub type PwrcuBakreg0 = crate::Reg<pwrcu_bakreg0::PwrcuBakreg0Spec>;
#[doc = "PWRCU_BAKREG0"]
pub mod pwrcu_bakreg0;
#[doc = "PWRCU_BAKREG1 (rw) register accessor: PWRCU_BAKREG1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcu_bakreg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcu_bakreg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrcu_bakreg1`]
module"]
#[doc(alias = "PWRCU_BAKREG1")]
pub type PwrcuBakreg1 = crate::Reg<pwrcu_bakreg1::PwrcuBakreg1Spec>;
#[doc = "PWRCU_BAKREG1"]
pub mod pwrcu_bakreg1;
#[doc = "PWRCU_BAKREG2 (rw) register accessor: PWRCU_BAKREG2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcu_bakreg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcu_bakreg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrcu_bakreg2`]
module"]
#[doc(alias = "PWRCU_BAKREG2")]
pub type PwrcuBakreg2 = crate::Reg<pwrcu_bakreg2::PwrcuBakreg2Spec>;
#[doc = "PWRCU_BAKREG2"]
pub mod pwrcu_bakreg2;
#[doc = "PWRCU_BAKREG3 (rw) register accessor: PWRCU_BAKREG3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcu_bakreg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcu_bakreg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrcu_bakreg3`]
module"]
#[doc(alias = "PWRCU_BAKREG3")]
pub type PwrcuBakreg3 = crate::Reg<pwrcu_bakreg3::PwrcuBakreg3Spec>;
#[doc = "PWRCU_BAKREG3"]
pub mod pwrcu_bakreg3;
#[doc = "PWRCU_BAKREG4 (rw) register accessor: PWRCU_BAKREG4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcu_bakreg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcu_bakreg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrcu_bakreg4`]
module"]
#[doc(alias = "PWRCU_BAKREG4")]
pub type PwrcuBakreg4 = crate::Reg<pwrcu_bakreg4::PwrcuBakreg4Spec>;
#[doc = "PWRCU_BAKREG4"]
pub mod pwrcu_bakreg4;
#[doc = "PWRCU_BAKREG5 (rw) register accessor: PWRCU_BAKREG5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcu_bakreg5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcu_bakreg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrcu_bakreg5`]
module"]
#[doc(alias = "PWRCU_BAKREG5")]
pub type PwrcuBakreg5 = crate::Reg<pwrcu_bakreg5::PwrcuBakreg5Spec>;
#[doc = "PWRCU_BAKREG5"]
pub mod pwrcu_bakreg5;
#[doc = "PWRCU_BAKREG6 (rw) register accessor: PWRCU_BAKREG6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcu_bakreg6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcu_bakreg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrcu_bakreg6`]
module"]
#[doc(alias = "PWRCU_BAKREG6")]
pub type PwrcuBakreg6 = crate::Reg<pwrcu_bakreg6::PwrcuBakreg6Spec>;
#[doc = "PWRCU_BAKREG6"]
pub mod pwrcu_bakreg6;
#[doc = "PWRCU_BAKREG7 (rw) register accessor: PWRCU_BAKREG7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcu_bakreg7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcu_bakreg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrcu_bakreg7`]
module"]
#[doc(alias = "PWRCU_BAKREG7")]
pub type PwrcuBakreg7 = crate::Reg<pwrcu_bakreg7::PwrcuBakreg7Spec>;
#[doc = "PWRCU_BAKREG7"]
pub mod pwrcu_bakreg7;
#[doc = "PWRCU_BAKREG8 (rw) register accessor: PWRCU_BAKREG8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcu_bakreg8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcu_bakreg8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrcu_bakreg8`]
module"]
#[doc(alias = "PWRCU_BAKREG8")]
pub type PwrcuBakreg8 = crate::Reg<pwrcu_bakreg8::PwrcuBakreg8Spec>;
#[doc = "PWRCU_BAKREG8"]
pub mod pwrcu_bakreg8;
#[doc = "PWRCU_BAKREG9 (rw) register accessor: PWRCU_BAKREG9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcu_bakreg9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcu_bakreg9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrcu_bakreg9`]
module"]
#[doc(alias = "PWRCU_BAKREG9")]
pub type PwrcuBakreg9 = crate::Reg<pwrcu_bakreg9::PwrcuBakreg9Spec>;
#[doc = "PWRCU_BAKREG9"]
pub mod pwrcu_bakreg9;
