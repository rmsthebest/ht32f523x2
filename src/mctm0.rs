#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cntcfr: Cntcfr,
    mdcfr: Mdcfr,
    trcfr: Trcfr,
    _reserved3: [u8; 0x04],
    ctr: Ctr,
    _reserved4: [u8; 0x0c],
    ch0icfr: Ch0icfr,
    ch1icfr: Ch1icfr,
    ch2icfr: Ch2icfr,
    ch3icfr: Ch3icfr,
    _reserved8: [u8; 0x10],
    ch0ocfr: Ch0ocfr,
    ch1ocfr: Ch1ocfr,
    ch2ocfr: Ch2ocfr,
    ch3ocfr: Ch3ocfr,
    chctr: Chctr,
    chpolr: Chpolr,
    _reserved14: [u8; 0x14],
    chbrkcfr: Chbrkcfr,
    chbrkctr: Chbrkctr,
    dictr: Dictr,
    evgr: Evgr,
    intsr: Intsr,
    cntr: Cntr,
    pscr: Pscr,
    crr: Crr,
    repr: Repr,
    ch0ccr: Ch0ccr,
    ch1ccr: Ch1ccr,
    ch2ccr: Ch2ccr,
    ch3ccr: Ch3ccr,
    ch0acr: Ch0acr,
    ch1acr: Ch1acr,
    ch2acr: Ch2acr,
    ch3acr: Ch3acr,
}
impl RegisterBlock {
    #[doc = "0x00 - CNTCFR"]
    #[inline(always)]
    pub const fn cntcfr(&self) -> &Cntcfr {
        &self.cntcfr
    }
    #[doc = "0x04 - MDCFR"]
    #[inline(always)]
    pub const fn mdcfr(&self) -> &Mdcfr {
        &self.mdcfr
    }
    #[doc = "0x08 - TRCFR"]
    #[inline(always)]
    pub const fn trcfr(&self) -> &Trcfr {
        &self.trcfr
    }
    #[doc = "0x10 - CTR"]
    #[inline(always)]
    pub const fn ctr(&self) -> &Ctr {
        &self.ctr
    }
    #[doc = "0x20 - CH0ICFR"]
    #[inline(always)]
    pub const fn ch0icfr(&self) -> &Ch0icfr {
        &self.ch0icfr
    }
    #[doc = "0x24 - CH1ICFR"]
    #[inline(always)]
    pub const fn ch1icfr(&self) -> &Ch1icfr {
        &self.ch1icfr
    }
    #[doc = "0x28 - CH2ICFR"]
    #[inline(always)]
    pub const fn ch2icfr(&self) -> &Ch2icfr {
        &self.ch2icfr
    }
    #[doc = "0x2c - CH3ICFR"]
    #[inline(always)]
    pub const fn ch3icfr(&self) -> &Ch3icfr {
        &self.ch3icfr
    }
    #[doc = "0x40 - CH0OCFR"]
    #[inline(always)]
    pub const fn ch0ocfr(&self) -> &Ch0ocfr {
        &self.ch0ocfr
    }
    #[doc = "0x44 - CH1OCFR"]
    #[inline(always)]
    pub const fn ch1ocfr(&self) -> &Ch1ocfr {
        &self.ch1ocfr
    }
    #[doc = "0x48 - CH2OCFR"]
    #[inline(always)]
    pub const fn ch2ocfr(&self) -> &Ch2ocfr {
        &self.ch2ocfr
    }
    #[doc = "0x4c - CH3OCFR"]
    #[inline(always)]
    pub const fn ch3ocfr(&self) -> &Ch3ocfr {
        &self.ch3ocfr
    }
    #[doc = "0x50 - CHCTR"]
    #[inline(always)]
    pub const fn chctr(&self) -> &Chctr {
        &self.chctr
    }
    #[doc = "0x54 - CHPOLR"]
    #[inline(always)]
    pub const fn chpolr(&self) -> &Chpolr {
        &self.chpolr
    }
    #[doc = "0x6c - CHBRKCFR"]
    #[inline(always)]
    pub const fn chbrkcfr(&self) -> &Chbrkcfr {
        &self.chbrkcfr
    }
    #[doc = "0x70 - CHBRKCTR"]
    #[inline(always)]
    pub const fn chbrkctr(&self) -> &Chbrkctr {
        &self.chbrkctr
    }
    #[doc = "0x74 - DICTR"]
    #[inline(always)]
    pub const fn dictr(&self) -> &Dictr {
        &self.dictr
    }
    #[doc = "0x78 - EVGR"]
    #[inline(always)]
    pub const fn evgr(&self) -> &Evgr {
        &self.evgr
    }
    #[doc = "0x7c - INTSR"]
    #[inline(always)]
    pub const fn intsr(&self) -> &Intsr {
        &self.intsr
    }
    #[doc = "0x80 - CNTR"]
    #[inline(always)]
    pub const fn cntr(&self) -> &Cntr {
        &self.cntr
    }
    #[doc = "0x84 - PSCR"]
    #[inline(always)]
    pub const fn pscr(&self) -> &Pscr {
        &self.pscr
    }
    #[doc = "0x88 - CRR"]
    #[inline(always)]
    pub const fn crr(&self) -> &Crr {
        &self.crr
    }
    #[doc = "0x8c - REPR"]
    #[inline(always)]
    pub const fn repr(&self) -> &Repr {
        &self.repr
    }
    #[doc = "0x90 - CH0CCR"]
    #[inline(always)]
    pub const fn ch0ccr(&self) -> &Ch0ccr {
        &self.ch0ccr
    }
    #[doc = "0x94 - CH1CCR"]
    #[inline(always)]
    pub const fn ch1ccr(&self) -> &Ch1ccr {
        &self.ch1ccr
    }
    #[doc = "0x98 - CH2CCR"]
    #[inline(always)]
    pub const fn ch2ccr(&self) -> &Ch2ccr {
        &self.ch2ccr
    }
    #[doc = "0x9c - CH3CCR"]
    #[inline(always)]
    pub const fn ch3ccr(&self) -> &Ch3ccr {
        &self.ch3ccr
    }
    #[doc = "0xa0 - CH0ACR"]
    #[inline(always)]
    pub const fn ch0acr(&self) -> &Ch0acr {
        &self.ch0acr
    }
    #[doc = "0xa4 - CH1ACR"]
    #[inline(always)]
    pub const fn ch1acr(&self) -> &Ch1acr {
        &self.ch1acr
    }
    #[doc = "0xa8 - CH2ACR"]
    #[inline(always)]
    pub const fn ch2acr(&self) -> &Ch2acr {
        &self.ch2acr
    }
    #[doc = "0xac - CH3ACR"]
    #[inline(always)]
    pub const fn ch3acr(&self) -> &Ch3acr {
        &self.ch3acr
    }
}
#[doc = "CNTCFR (rw) register accessor: CNTCFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntcfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntcfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntcfr`]
module"]
#[doc(alias = "CNTCFR")]
pub type Cntcfr = crate::Reg<cntcfr::CntcfrSpec>;
#[doc = "CNTCFR"]
pub mod cntcfr;
#[doc = "MDCFR (rw) register accessor: MDCFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdcfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdcfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdcfr`]
module"]
#[doc(alias = "MDCFR")]
pub type Mdcfr = crate::Reg<mdcfr::MdcfrSpec>;
#[doc = "MDCFR"]
pub mod mdcfr;
#[doc = "TRCFR (rw) register accessor: TRCFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trcfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trcfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trcfr`]
module"]
#[doc(alias = "TRCFR")]
pub type Trcfr = crate::Reg<trcfr::TrcfrSpec>;
#[doc = "TRCFR"]
pub mod trcfr;
#[doc = "CTR (rw) register accessor: CTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctr`]
module"]
#[doc(alias = "CTR")]
pub type Ctr = crate::Reg<ctr::CtrSpec>;
#[doc = "CTR"]
pub mod ctr;
#[doc = "CH0ICFR (rw) register accessor: CH0ICFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0icfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0icfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0icfr`]
module"]
#[doc(alias = "CH0ICFR")]
pub type Ch0icfr = crate::Reg<ch0icfr::Ch0icfrSpec>;
#[doc = "CH0ICFR"]
pub mod ch0icfr;
#[doc = "CH1ICFR (rw) register accessor: CH1ICFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1icfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1icfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1icfr`]
module"]
#[doc(alias = "CH1ICFR")]
pub type Ch1icfr = crate::Reg<ch1icfr::Ch1icfrSpec>;
#[doc = "CH1ICFR"]
pub mod ch1icfr;
#[doc = "CH2ICFR (rw) register accessor: CH2ICFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2icfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2icfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2icfr`]
module"]
#[doc(alias = "CH2ICFR")]
pub type Ch2icfr = crate::Reg<ch2icfr::Ch2icfrSpec>;
#[doc = "CH2ICFR"]
pub mod ch2icfr;
#[doc = "CH3ICFR (rw) register accessor: CH3ICFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3icfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3icfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3icfr`]
module"]
#[doc(alias = "CH3ICFR")]
pub type Ch3icfr = crate::Reg<ch3icfr::Ch3icfrSpec>;
#[doc = "CH3ICFR"]
pub mod ch3icfr;
#[doc = "CH0OCFR (rw) register accessor: CH0OCFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0ocfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0ocfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0ocfr`]
module"]
#[doc(alias = "CH0OCFR")]
pub type Ch0ocfr = crate::Reg<ch0ocfr::Ch0ocfrSpec>;
#[doc = "CH0OCFR"]
pub mod ch0ocfr;
#[doc = "CH1OCFR (rw) register accessor: CH1OCFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1ocfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1ocfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1ocfr`]
module"]
#[doc(alias = "CH1OCFR")]
pub type Ch1ocfr = crate::Reg<ch1ocfr::Ch1ocfrSpec>;
#[doc = "CH1OCFR"]
pub mod ch1ocfr;
#[doc = "CH2OCFR (rw) register accessor: CH2OCFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2ocfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2ocfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2ocfr`]
module"]
#[doc(alias = "CH2OCFR")]
pub type Ch2ocfr = crate::Reg<ch2ocfr::Ch2ocfrSpec>;
#[doc = "CH2OCFR"]
pub mod ch2ocfr;
#[doc = "CH3OCFR (rw) register accessor: CH3OCFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3ocfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3ocfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3ocfr`]
module"]
#[doc(alias = "CH3OCFR")]
pub type Ch3ocfr = crate::Reg<ch3ocfr::Ch3ocfrSpec>;
#[doc = "CH3OCFR"]
pub mod ch3ocfr;
#[doc = "CHCTR (rw) register accessor: CHCTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctr`]
module"]
#[doc(alias = "CHCTR")]
pub type Chctr = crate::Reg<chctr::ChctrSpec>;
#[doc = "CHCTR"]
pub mod chctr;
#[doc = "CHPOLR (rw) register accessor: CHPOLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chpolr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chpolr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chpolr`]
module"]
#[doc(alias = "CHPOLR")]
pub type Chpolr = crate::Reg<chpolr::ChpolrSpec>;
#[doc = "CHPOLR"]
pub mod chpolr;
#[doc = "CHBRKCFR (rw) register accessor: CHBRKCFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chbrkcfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chbrkcfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chbrkcfr`]
module"]
#[doc(alias = "CHBRKCFR")]
pub type Chbrkcfr = crate::Reg<chbrkcfr::ChbrkcfrSpec>;
#[doc = "CHBRKCFR"]
pub mod chbrkcfr;
#[doc = "CHBRKCTR (rw) register accessor: CHBRKCTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chbrkctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chbrkctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chbrkctr`]
module"]
#[doc(alias = "CHBRKCTR")]
pub type Chbrkctr = crate::Reg<chbrkctr::ChbrkctrSpec>;
#[doc = "CHBRKCTR"]
pub mod chbrkctr;
#[doc = "DICTR (rw) register accessor: DICTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dictr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dictr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dictr`]
module"]
#[doc(alias = "DICTR")]
pub type Dictr = crate::Reg<dictr::DictrSpec>;
#[doc = "DICTR"]
pub mod dictr;
#[doc = "EVGR (rw) register accessor: EVGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evgr`]
module"]
#[doc(alias = "EVGR")]
pub type Evgr = crate::Reg<evgr::EvgrSpec>;
#[doc = "EVGR"]
pub mod evgr;
#[doc = "INTSR (rw) register accessor: INTSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsr`]
module"]
#[doc(alias = "INTSR")]
pub type Intsr = crate::Reg<intsr::IntsrSpec>;
#[doc = "INTSR"]
pub mod intsr;
#[doc = "CNTR (rw) register accessor: CNTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr`]
module"]
#[doc(alias = "CNTR")]
pub type Cntr = crate::Reg<cntr::CntrSpec>;
#[doc = "CNTR"]
pub mod cntr;
#[doc = "PSCR (rw) register accessor: PSCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscr`]
module"]
#[doc(alias = "PSCR")]
pub type Pscr = crate::Reg<pscr::PscrSpec>;
#[doc = "PSCR"]
pub mod pscr;
#[doc = "CRR (rw) register accessor: CRR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crr`]
module"]
#[doc(alias = "CRR")]
pub type Crr = crate::Reg<crr::CrrSpec>;
#[doc = "CRR"]
pub mod crr;
#[doc = "REPR (rw) register accessor: REPR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`repr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`repr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@repr`]
module"]
#[doc(alias = "REPR")]
pub type Repr = crate::Reg<repr::ReprSpec>;
#[doc = "REPR"]
pub mod repr;
#[doc = "CH0CCR (rw) register accessor: CH0CCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0ccr`]
module"]
#[doc(alias = "CH0CCR")]
pub type Ch0ccr = crate::Reg<ch0ccr::Ch0ccrSpec>;
#[doc = "CH0CCR"]
pub mod ch0ccr;
#[doc = "CH1CCR (rw) register accessor: CH1CCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1ccr`]
module"]
#[doc(alias = "CH1CCR")]
pub type Ch1ccr = crate::Reg<ch1ccr::Ch1ccrSpec>;
#[doc = "CH1CCR"]
pub mod ch1ccr;
#[doc = "CH2CCR (rw) register accessor: CH2CCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2ccr`]
module"]
#[doc(alias = "CH2CCR")]
pub type Ch2ccr = crate::Reg<ch2ccr::Ch2ccrSpec>;
#[doc = "CH2CCR"]
pub mod ch2ccr;
#[doc = "CH3CCR (rw) register accessor: CH3CCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3ccr`]
module"]
#[doc(alias = "CH3CCR")]
pub type Ch3ccr = crate::Reg<ch3ccr::Ch3ccrSpec>;
#[doc = "CH3CCR"]
pub mod ch3ccr;
#[doc = "CH0ACR (rw) register accessor: CH0ACR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0acr`]
module"]
#[doc(alias = "CH0ACR")]
pub type Ch0acr = crate::Reg<ch0acr::Ch0acrSpec>;
#[doc = "CH0ACR"]
pub mod ch0acr;
#[doc = "CH1ACR (rw) register accessor: CH1ACR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1acr`]
module"]
#[doc(alias = "CH1ACR")]
pub type Ch1acr = crate::Reg<ch1acr::Ch1acrSpec>;
#[doc = "CH1ACR"]
pub mod ch1acr;
#[doc = "CH2ACR (rw) register accessor: CH2ACR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2acr`]
module"]
#[doc(alias = "CH2ACR")]
pub type Ch2acr = crate::Reg<ch2acr::Ch2acrSpec>;
#[doc = "CH2ACR"]
pub mod ch2acr;
#[doc = "CH3ACR (rw) register accessor: CH3ACR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3acr`]
module"]
#[doc(alias = "CH3ACR")]
pub type Ch3acr = crate::Reg<ch3acr::Ch3acrSpec>;
#[doc = "CH3ACR"]
pub mod ch3acr;
