#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    sr: Sr,
    ccr: Ccr,
    etur: Etur,
    gtr: Gtr,
    wtr: Wtr,
    ier: Ier,
    ipr: Ipr,
    txb: Txb,
    rxb: Rxb,
    pscr: Pscr,
}
impl RegisterBlock {
    #[doc = "0x00 - CR"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - SR"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x08 - CCR"]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    #[doc = "0x0c - ETUR"]
    #[inline(always)]
    pub const fn etur(&self) -> &Etur {
        &self.etur
    }
    #[doc = "0x10 - GTR"]
    #[inline(always)]
    pub const fn gtr(&self) -> &Gtr {
        &self.gtr
    }
    #[doc = "0x14 - WTR"]
    #[inline(always)]
    pub const fn wtr(&self) -> &Wtr {
        &self.wtr
    }
    #[doc = "0x18 - IER"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x1c - IPR"]
    #[inline(always)]
    pub const fn ipr(&self) -> &Ipr {
        &self.ipr
    }
    #[doc = "0x20 - TXB"]
    #[inline(always)]
    pub const fn txb(&self) -> &Txb {
        &self.txb
    }
    #[doc = "0x24 - RXB"]
    #[inline(always)]
    pub const fn rxb(&self) -> &Rxb {
        &self.rxb
    }
    #[doc = "0x28 - PSCR"]
    #[inline(always)]
    pub const fn pscr(&self) -> &Pscr {
        &self.pscr
    }
}
#[doc = "CR (rw) register accessor: CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "CR"]
pub mod cr;
#[doc = "SR (rw) register accessor: SR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "SR"]
pub mod sr;
#[doc = "CCR (rw) register accessor: CCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "CCR"]
pub mod ccr;
#[doc = "ETUR (rw) register accessor: ETUR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etur`]
module"]
#[doc(alias = "ETUR")]
pub type Etur = crate::Reg<etur::EturSpec>;
#[doc = "ETUR"]
pub mod etur;
#[doc = "GTR (rw) register accessor: GTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtr`]
module"]
#[doc(alias = "GTR")]
pub type Gtr = crate::Reg<gtr::GtrSpec>;
#[doc = "GTR"]
pub mod gtr;
#[doc = "WTR (rw) register accessor: WTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wtr`]
module"]
#[doc(alias = "WTR")]
pub type Wtr = crate::Reg<wtr::WtrSpec>;
#[doc = "WTR"]
pub mod wtr;
#[doc = "IER (rw) register accessor: IER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "IER"]
pub mod ier;
#[doc = "IPR (rw) register accessor: IPR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipr`]
module"]
#[doc(alias = "IPR")]
pub type Ipr = crate::Reg<ipr::IprSpec>;
#[doc = "IPR"]
pub mod ipr;
#[doc = "TXB (rw) register accessor: TXB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txb`]
module"]
#[doc(alias = "TXB")]
pub type Txb = crate::Reg<txb::TxbSpec>;
#[doc = "TXB"]
pub mod txb;
#[doc = "RXB (rw) register accessor: RXB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxb`]
module"]
#[doc(alias = "RXB")]
pub type Rxb = crate::Reg<rxb::RxbSpec>;
#[doc = "RXB"]
pub mod rxb;
#[doc = "PSCR (rw) register accessor: PSCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscr`]
module"]
#[doc(alias = "PSCR")]
pub type Pscr = crate::Reg<pscr::PscrSpec>;
#[doc = "PSCR"]
pub mod pscr;
