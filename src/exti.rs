#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfgr0: Cfgr0,
    cfgr1: Cfgr1,
    cfgr2: Cfgr2,
    cfgr3: Cfgr3,
    cfgr4: Cfgr4,
    cfgr5: Cfgr5,
    cfgr6: Cfgr6,
    cfgr7: Cfgr7,
    cfgr8: Cfgr8,
    cfgr9: Cfgr9,
    cfgr10: Cfgr10,
    cfgr11: Cfgr11,
    cfgr12: Cfgr12,
    cfgr13: Cfgr13,
    cfgr14: Cfgr14,
    cfgr15: Cfgr15,
    cr: Cr,
    edgeflgr: Edgeflgr,
    edgesr: Edgesr,
    sscr: Sscr,
    wakupcr: Wakupcr,
    wakuppolr: Wakuppolr,
    wakupflg: Wakupflg,
}
impl RegisterBlock {
    #[doc = "0x00 - CFGR0"]
    #[inline(always)]
    pub const fn cfgr0(&self) -> &Cfgr0 {
        &self.cfgr0
    }
    #[doc = "0x04 - CFGR1"]
    #[inline(always)]
    pub const fn cfgr1(&self) -> &Cfgr1 {
        &self.cfgr1
    }
    #[doc = "0x08 - CFGR2"]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &Cfgr2 {
        &self.cfgr2
    }
    #[doc = "0x0c - CFGR3"]
    #[inline(always)]
    pub const fn cfgr3(&self) -> &Cfgr3 {
        &self.cfgr3
    }
    #[doc = "0x10 - CFGR4"]
    #[inline(always)]
    pub const fn cfgr4(&self) -> &Cfgr4 {
        &self.cfgr4
    }
    #[doc = "0x14 - CFGR5"]
    #[inline(always)]
    pub const fn cfgr5(&self) -> &Cfgr5 {
        &self.cfgr5
    }
    #[doc = "0x18 - CFGR6"]
    #[inline(always)]
    pub const fn cfgr6(&self) -> &Cfgr6 {
        &self.cfgr6
    }
    #[doc = "0x1c - CFGR7"]
    #[inline(always)]
    pub const fn cfgr7(&self) -> &Cfgr7 {
        &self.cfgr7
    }
    #[doc = "0x20 - CFGR8"]
    #[inline(always)]
    pub const fn cfgr8(&self) -> &Cfgr8 {
        &self.cfgr8
    }
    #[doc = "0x24 - CFGR9"]
    #[inline(always)]
    pub const fn cfgr9(&self) -> &Cfgr9 {
        &self.cfgr9
    }
    #[doc = "0x28 - CFGR10"]
    #[inline(always)]
    pub const fn cfgr10(&self) -> &Cfgr10 {
        &self.cfgr10
    }
    #[doc = "0x2c - CFGR11"]
    #[inline(always)]
    pub const fn cfgr11(&self) -> &Cfgr11 {
        &self.cfgr11
    }
    #[doc = "0x30 - CFGR12"]
    #[inline(always)]
    pub const fn cfgr12(&self) -> &Cfgr12 {
        &self.cfgr12
    }
    #[doc = "0x34 - CFGR13"]
    #[inline(always)]
    pub const fn cfgr13(&self) -> &Cfgr13 {
        &self.cfgr13
    }
    #[doc = "0x38 - CFGR14"]
    #[inline(always)]
    pub const fn cfgr14(&self) -> &Cfgr14 {
        &self.cfgr14
    }
    #[doc = "0x3c - CFGR15"]
    #[inline(always)]
    pub const fn cfgr15(&self) -> &Cfgr15 {
        &self.cfgr15
    }
    #[doc = "0x40 - CR"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x44 - EDGEFLGR"]
    #[inline(always)]
    pub const fn edgeflgr(&self) -> &Edgeflgr {
        &self.edgeflgr
    }
    #[doc = "0x48 - EDGESR"]
    #[inline(always)]
    pub const fn edgesr(&self) -> &Edgesr {
        &self.edgesr
    }
    #[doc = "0x4c - SSCR"]
    #[inline(always)]
    pub const fn sscr(&self) -> &Sscr {
        &self.sscr
    }
    #[doc = "0x50 - WAKUPCR"]
    #[inline(always)]
    pub const fn wakupcr(&self) -> &Wakupcr {
        &self.wakupcr
    }
    #[doc = "0x54 - WAKUPPOLR"]
    #[inline(always)]
    pub const fn wakuppolr(&self) -> &Wakuppolr {
        &self.wakuppolr
    }
    #[doc = "0x58 - WAKUPFLG"]
    #[inline(always)]
    pub const fn wakupflg(&self) -> &Wakupflg {
        &self.wakupflg
    }
}
#[doc = "CFGR0 (rw) register accessor: CFGR0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr0`]
module"]
#[doc(alias = "CFGR0")]
pub type Cfgr0 = crate::Reg<cfgr0::Cfgr0Spec>;
#[doc = "CFGR0"]
pub mod cfgr0;
#[doc = "CFGR1 (rw) register accessor: CFGR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr1`]
module"]
#[doc(alias = "CFGR1")]
pub type Cfgr1 = crate::Reg<cfgr1::Cfgr1Spec>;
#[doc = "CFGR1"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: CFGR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`]
module"]
#[doc(alias = "CFGR2")]
pub type Cfgr2 = crate::Reg<cfgr2::Cfgr2Spec>;
#[doc = "CFGR2"]
pub mod cfgr2;
#[doc = "CFGR3 (rw) register accessor: CFGR3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr3`]
module"]
#[doc(alias = "CFGR3")]
pub type Cfgr3 = crate::Reg<cfgr3::Cfgr3Spec>;
#[doc = "CFGR3"]
pub mod cfgr3;
#[doc = "CFGR4 (rw) register accessor: CFGR4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr4`]
module"]
#[doc(alias = "CFGR4")]
pub type Cfgr4 = crate::Reg<cfgr4::Cfgr4Spec>;
#[doc = "CFGR4"]
pub mod cfgr4;
#[doc = "CFGR5 (rw) register accessor: CFGR5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr5`]
module"]
#[doc(alias = "CFGR5")]
pub type Cfgr5 = crate::Reg<cfgr5::Cfgr5Spec>;
#[doc = "CFGR5"]
pub mod cfgr5;
#[doc = "CFGR6 (rw) register accessor: CFGR6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr6`]
module"]
#[doc(alias = "CFGR6")]
pub type Cfgr6 = crate::Reg<cfgr6::Cfgr6Spec>;
#[doc = "CFGR6"]
pub mod cfgr6;
#[doc = "CFGR7 (rw) register accessor: CFGR7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr7`]
module"]
#[doc(alias = "CFGR7")]
pub type Cfgr7 = crate::Reg<cfgr7::Cfgr7Spec>;
#[doc = "CFGR7"]
pub mod cfgr7;
#[doc = "CFGR8 (rw) register accessor: CFGR8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr8`]
module"]
#[doc(alias = "CFGR8")]
pub type Cfgr8 = crate::Reg<cfgr8::Cfgr8Spec>;
#[doc = "CFGR8"]
pub mod cfgr8;
#[doc = "CFGR9 (rw) register accessor: CFGR9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr9`]
module"]
#[doc(alias = "CFGR9")]
pub type Cfgr9 = crate::Reg<cfgr9::Cfgr9Spec>;
#[doc = "CFGR9"]
pub mod cfgr9;
#[doc = "CFGR10 (rw) register accessor: CFGR10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr10`]
module"]
#[doc(alias = "CFGR10")]
pub type Cfgr10 = crate::Reg<cfgr10::Cfgr10Spec>;
#[doc = "CFGR10"]
pub mod cfgr10;
#[doc = "CFGR11 (rw) register accessor: CFGR11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr11`]
module"]
#[doc(alias = "CFGR11")]
pub type Cfgr11 = crate::Reg<cfgr11::Cfgr11Spec>;
#[doc = "CFGR11"]
pub mod cfgr11;
#[doc = "CFGR12 (rw) register accessor: CFGR12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr12`]
module"]
#[doc(alias = "CFGR12")]
pub type Cfgr12 = crate::Reg<cfgr12::Cfgr12Spec>;
#[doc = "CFGR12"]
pub mod cfgr12;
#[doc = "CFGR13 (rw) register accessor: CFGR13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr13`]
module"]
#[doc(alias = "CFGR13")]
pub type Cfgr13 = crate::Reg<cfgr13::Cfgr13Spec>;
#[doc = "CFGR13"]
pub mod cfgr13;
#[doc = "CFGR14 (rw) register accessor: CFGR14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr14`]
module"]
#[doc(alias = "CFGR14")]
pub type Cfgr14 = crate::Reg<cfgr14::Cfgr14Spec>;
#[doc = "CFGR14"]
pub mod cfgr14;
#[doc = "CFGR15 (rw) register accessor: CFGR15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr15`]
module"]
#[doc(alias = "CFGR15")]
pub type Cfgr15 = crate::Reg<cfgr15::Cfgr15Spec>;
#[doc = "CFGR15"]
pub mod cfgr15;
#[doc = "CR (rw) register accessor: CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "CR"]
pub mod cr;
#[doc = "EDGEFLGR (rw) register accessor: EDGEFLGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edgeflgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edgeflgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edgeflgr`]
module"]
#[doc(alias = "EDGEFLGR")]
pub type Edgeflgr = crate::Reg<edgeflgr::EdgeflgrSpec>;
#[doc = "EDGEFLGR"]
pub mod edgeflgr;
#[doc = "EDGESR (rw) register accessor: EDGESR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edgesr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edgesr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edgesr`]
module"]
#[doc(alias = "EDGESR")]
pub type Edgesr = crate::Reg<edgesr::EdgesrSpec>;
#[doc = "EDGESR"]
pub mod edgesr;
#[doc = "SSCR (rw) register accessor: SSCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sscr`]
module"]
#[doc(alias = "SSCR")]
pub type Sscr = crate::Reg<sscr::SscrSpec>;
#[doc = "SSCR"]
pub mod sscr;
#[doc = "WAKUPCR (rw) register accessor: WAKUPCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakupcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakupcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakupcr`]
module"]
#[doc(alias = "WAKUPCR")]
pub type Wakupcr = crate::Reg<wakupcr::WakupcrSpec>;
#[doc = "WAKUPCR"]
pub mod wakupcr;
#[doc = "WAKUPPOLR (rw) register accessor: WAKUPPOLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakuppolr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakuppolr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakuppolr`]
module"]
#[doc(alias = "WAKUPPOLR")]
pub type Wakuppolr = crate::Reg<wakuppolr::WakuppolrSpec>;
#[doc = "WAKUPPOLR"]
pub mod wakuppolr;
#[doc = "WAKUPFLG (rw) register accessor: WAKUPFLG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakupflg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakupflg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakupflg`]
module"]
#[doc(alias = "WAKUPFLG")]
pub type Wakupflg = crate::Reg<wakupflg::WakupflgSpec>;
#[doc = "WAKUPFLG"]
pub mod wakupflg;
