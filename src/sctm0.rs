#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cntcfr: Cntcfr,
    gptm_mdcfr: GptmMdcfr,
    gptm_trcfr: GptmTrcfr,
    _reserved3: [u8; 0x04],
    gptm_ctr: GptmCtr,
    _reserved4: [u8; 0x0c],
    gptm_ch0icfr: GptmCh0icfr,
    _reserved5: [u8; 0x20],
    gptm_ch1ocfr: GptmCh1ocfr,
    _reserved6: [u8; 0x08],
    gptm_chctr: GptmChctr,
    gptm_chpolr: GptmChpolr,
    _reserved8: [u8; 0x1c],
    gptm_dictr: GptmDictr,
    gptm_evgr: GptmEvgr,
    gptm_intsr: GptmIntsr,
    gptm_cntr: GptmCntr,
    gptm_pscr: GptmPscr,
    gptm_crr: GptmCrr,
    _reserved14: [u8; 0x04],
    gptm_ch0ccr: GptmCh0ccr,
}
impl RegisterBlock {
    #[doc = "0x00 - CNTCFR"]
    #[inline(always)]
    pub const fn cntcfr(&self) -> &Cntcfr {
        &self.cntcfr
    }
    #[doc = "0x04 - GPTM_MDCFR"]
    #[inline(always)]
    pub const fn gptm_mdcfr(&self) -> &GptmMdcfr {
        &self.gptm_mdcfr
    }
    #[doc = "0x08 - GPTM_TRCFR"]
    #[inline(always)]
    pub const fn gptm_trcfr(&self) -> &GptmTrcfr {
        &self.gptm_trcfr
    }
    #[doc = "0x10 - GPTM_CTR"]
    #[inline(always)]
    pub const fn gptm_ctr(&self) -> &GptmCtr {
        &self.gptm_ctr
    }
    #[doc = "0x20 - GPTM_CH0ICFR"]
    #[inline(always)]
    pub const fn gptm_ch0icfr(&self) -> &GptmCh0icfr {
        &self.gptm_ch0icfr
    }
    #[doc = "0x44 - GPTM_CH1OCFR"]
    #[inline(always)]
    pub const fn gptm_ch1ocfr(&self) -> &GptmCh1ocfr {
        &self.gptm_ch1ocfr
    }
    #[doc = "0x50 - GPTM_CHCTR"]
    #[inline(always)]
    pub const fn gptm_chctr(&self) -> &GptmChctr {
        &self.gptm_chctr
    }
    #[doc = "0x54 - GPTM_CHPOLR"]
    #[inline(always)]
    pub const fn gptm_chpolr(&self) -> &GptmChpolr {
        &self.gptm_chpolr
    }
    #[doc = "0x74 - GPTM_DICTR"]
    #[inline(always)]
    pub const fn gptm_dictr(&self) -> &GptmDictr {
        &self.gptm_dictr
    }
    #[doc = "0x78 - GPTM_EVGR"]
    #[inline(always)]
    pub const fn gptm_evgr(&self) -> &GptmEvgr {
        &self.gptm_evgr
    }
    #[doc = "0x7c - GPTM_INTSR"]
    #[inline(always)]
    pub const fn gptm_intsr(&self) -> &GptmIntsr {
        &self.gptm_intsr
    }
    #[doc = "0x80 - GPTM_CNTR"]
    #[inline(always)]
    pub const fn gptm_cntr(&self) -> &GptmCntr {
        &self.gptm_cntr
    }
    #[doc = "0x84 - GPTM_PSCR"]
    #[inline(always)]
    pub const fn gptm_pscr(&self) -> &GptmPscr {
        &self.gptm_pscr
    }
    #[doc = "0x88 - GPTM_CRR"]
    #[inline(always)]
    pub const fn gptm_crr(&self) -> &GptmCrr {
        &self.gptm_crr
    }
    #[doc = "0x90 - GPTM_CH0CCR"]
    #[inline(always)]
    pub const fn gptm_ch0ccr(&self) -> &GptmCh0ccr {
        &self.gptm_ch0ccr
    }
}
#[doc = "CNTCFR (rw) register accessor: CNTCFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntcfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntcfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntcfr`]
module"]
#[doc(alias = "CNTCFR")]
pub type Cntcfr = crate::Reg<cntcfr::CntcfrSpec>;
#[doc = "CNTCFR"]
pub mod cntcfr;
#[doc = "GPTM_MDCFR (rw) register accessor: GPTM_MDCFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_mdcfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_mdcfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptm_mdcfr`]
module"]
#[doc(alias = "GPTM_MDCFR")]
pub type GptmMdcfr = crate::Reg<gptm_mdcfr::GptmMdcfrSpec>;
#[doc = "GPTM_MDCFR"]
pub mod gptm_mdcfr;
#[doc = "GPTM_TRCFR (rw) register accessor: GPTM_TRCFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_trcfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_trcfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptm_trcfr`]
module"]
#[doc(alias = "GPTM_TRCFR")]
pub type GptmTrcfr = crate::Reg<gptm_trcfr::GptmTrcfrSpec>;
#[doc = "GPTM_TRCFR"]
pub mod gptm_trcfr;
#[doc = "GPTM_CTR (rw) register accessor: GPTM_CTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptm_ctr`]
module"]
#[doc(alias = "GPTM_CTR")]
pub type GptmCtr = crate::Reg<gptm_ctr::GptmCtrSpec>;
#[doc = "GPTM_CTR"]
pub mod gptm_ctr;
#[doc = "GPTM_CH0ICFR (rw) register accessor: GPTM_CH0ICFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_ch0icfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_ch0icfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptm_ch0icfr`]
module"]
#[doc(alias = "GPTM_CH0ICFR")]
pub type GptmCh0icfr = crate::Reg<gptm_ch0icfr::GptmCh0icfrSpec>;
#[doc = "GPTM_CH0ICFR"]
pub mod gptm_ch0icfr;
#[doc = "GPTM_CH1OCFR (rw) register accessor: GPTM_CH1OCFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_ch1ocfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_ch1ocfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptm_ch1ocfr`]
module"]
#[doc(alias = "GPTM_CH1OCFR")]
pub type GptmCh1ocfr = crate::Reg<gptm_ch1ocfr::GptmCh1ocfrSpec>;
#[doc = "GPTM_CH1OCFR"]
pub mod gptm_ch1ocfr;
#[doc = "GPTM_CHCTR (rw) register accessor: GPTM_CHCTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_chctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_chctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptm_chctr`]
module"]
#[doc(alias = "GPTM_CHCTR")]
pub type GptmChctr = crate::Reg<gptm_chctr::GptmChctrSpec>;
#[doc = "GPTM_CHCTR"]
pub mod gptm_chctr;
#[doc = "GPTM_CHPOLR (rw) register accessor: GPTM_CHPOLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_chpolr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_chpolr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptm_chpolr`]
module"]
#[doc(alias = "GPTM_CHPOLR")]
pub type GptmChpolr = crate::Reg<gptm_chpolr::GptmChpolrSpec>;
#[doc = "GPTM_CHPOLR"]
pub mod gptm_chpolr;
#[doc = "GPTM_DICTR (rw) register accessor: GPTM_DICTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_dictr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_dictr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptm_dictr`]
module"]
#[doc(alias = "GPTM_DICTR")]
pub type GptmDictr = crate::Reg<gptm_dictr::GptmDictrSpec>;
#[doc = "GPTM_DICTR"]
pub mod gptm_dictr;
#[doc = "GPTM_EVGR (rw) register accessor: GPTM_EVGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_evgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_evgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptm_evgr`]
module"]
#[doc(alias = "GPTM_EVGR")]
pub type GptmEvgr = crate::Reg<gptm_evgr::GptmEvgrSpec>;
#[doc = "GPTM_EVGR"]
pub mod gptm_evgr;
#[doc = "GPTM_INTSR (rw) register accessor: GPTM_INTSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_intsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_intsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptm_intsr`]
module"]
#[doc(alias = "GPTM_INTSR")]
pub type GptmIntsr = crate::Reg<gptm_intsr::GptmIntsrSpec>;
#[doc = "GPTM_INTSR"]
pub mod gptm_intsr;
#[doc = "GPTM_CNTR (rw) register accessor: GPTM_CNTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_cntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_cntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptm_cntr`]
module"]
#[doc(alias = "GPTM_CNTR")]
pub type GptmCntr = crate::Reg<gptm_cntr::GptmCntrSpec>;
#[doc = "GPTM_CNTR"]
pub mod gptm_cntr;
#[doc = "GPTM_PSCR (rw) register accessor: GPTM_PSCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_pscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_pscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptm_pscr`]
module"]
#[doc(alias = "GPTM_PSCR")]
pub type GptmPscr = crate::Reg<gptm_pscr::GptmPscrSpec>;
#[doc = "GPTM_PSCR"]
pub mod gptm_pscr;
#[doc = "GPTM_CRR (rw) register accessor: GPTM_CRR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_crr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_crr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptm_crr`]
module"]
#[doc(alias = "GPTM_CRR")]
pub type GptmCrr = crate::Reg<gptm_crr::GptmCrrSpec>;
#[doc = "GPTM_CRR"]
pub mod gptm_crr;
#[doc = "GPTM_CH0CCR (rw) register accessor: GPTM_CH0CCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_ch0ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_ch0ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptm_ch0ccr`]
module"]
#[doc(alias = "GPTM_CH0CCR")]
pub type GptmCh0ccr = crate::Reg<gptm_ch0ccr::GptmCh0ccrSpec>;
#[doc = "GPTM_CH0CCR"]
pub mod gptm_ch0ccr;
