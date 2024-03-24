#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    _reserved1: [u8; 0x04],
    sr: Sr,
    _reserved2: [u8; 0x04],
    atr: Atr,
    rtr: Rtr,
    wtr: Wtr,
    pr: Pr,
}
impl RegisterBlock {
    #[doc = "0x00 - CR"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x08 - SR"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x10 - ATR"]
    #[inline(always)]
    pub const fn atr(&self) -> &Atr {
        &self.atr
    }
    #[doc = "0x14 - RTR"]
    #[inline(always)]
    pub const fn rtr(&self) -> &Rtr {
        &self.rtr
    }
    #[doc = "0x18 - WTR"]
    #[inline(always)]
    pub const fn wtr(&self) -> &Wtr {
        &self.wtr
    }
    #[doc = "0x1c - PR"]
    #[inline(always)]
    pub const fn pr(&self) -> &Pr {
        &self.pr
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
#[doc = "ATR (rw) register accessor: ATR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atr`]
module"]
#[doc(alias = "ATR")]
pub type Atr = crate::Reg<atr::AtrSpec>;
#[doc = "ATR"]
pub mod atr;
#[doc = "RTR (rw) register accessor: RTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtr`]
module"]
#[doc(alias = "RTR")]
pub type Rtr = crate::Reg<rtr::RtrSpec>;
#[doc = "RTR"]
pub mod rtr;
#[doc = "WTR (rw) register accessor: WTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wtr`]
module"]
#[doc(alias = "WTR")]
pub type Wtr = crate::Reg<wtr::WtrSpec>;
#[doc = "WTR"]
pub mod wtr;
#[doc = "PR (rw) register accessor: PR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr`]
module"]
#[doc(alias = "PR")]
pub type Pr = crate::Reg<pr::PrSpec>;
#[doc = "PR"]
pub mod pr;
