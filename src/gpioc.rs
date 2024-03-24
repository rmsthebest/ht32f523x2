#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dircr: Dircr,
    iner: Iner,
    pur: Pur,
    pdr: Pdr,
    odr: Odr,
    drvr: Drvr,
    lockr: Lockr,
    dinr: Dinr,
    doutr: Doutr,
    srr: Srr,
    rr: Rr,
}
impl RegisterBlock {
    #[doc = "0x00 - DIRCR"]
    #[inline(always)]
    pub const fn dircr(&self) -> &Dircr {
        &self.dircr
    }
    #[doc = "0x04 - INER"]
    #[inline(always)]
    pub const fn iner(&self) -> &Iner {
        &self.iner
    }
    #[doc = "0x08 - PUR"]
    #[inline(always)]
    pub const fn pur(&self) -> &Pur {
        &self.pur
    }
    #[doc = "0x0c - PDR"]
    #[inline(always)]
    pub const fn pdr(&self) -> &Pdr {
        &self.pdr
    }
    #[doc = "0x10 - ODR"]
    #[inline(always)]
    pub const fn odr(&self) -> &Odr {
        &self.odr
    }
    #[doc = "0x14 - DRVR"]
    #[inline(always)]
    pub const fn drvr(&self) -> &Drvr {
        &self.drvr
    }
    #[doc = "0x18 - LOCKR"]
    #[inline(always)]
    pub const fn lockr(&self) -> &Lockr {
        &self.lockr
    }
    #[doc = "0x1c - DINR"]
    #[inline(always)]
    pub const fn dinr(&self) -> &Dinr {
        &self.dinr
    }
    #[doc = "0x20 - DOUTR"]
    #[inline(always)]
    pub const fn doutr(&self) -> &Doutr {
        &self.doutr
    }
    #[doc = "0x24 - SRR"]
    #[inline(always)]
    pub const fn srr(&self) -> &Srr {
        &self.srr
    }
    #[doc = "0x28 - RR"]
    #[inline(always)]
    pub const fn rr(&self) -> &Rr {
        &self.rr
    }
}
#[doc = "DIRCR (rw) register accessor: DIRCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dircr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dircr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dircr`]
module"]
#[doc(alias = "DIRCR")]
pub type Dircr = crate::Reg<dircr::DircrSpec>;
#[doc = "DIRCR"]
pub mod dircr;
#[doc = "INER (rw) register accessor: INER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iner::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iner::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iner`]
module"]
#[doc(alias = "INER")]
pub type Iner = crate::Reg<iner::InerSpec>;
#[doc = "INER"]
pub mod iner;
#[doc = "PUR (rw) register accessor: PUR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pur`]
module"]
#[doc(alias = "PUR")]
pub type Pur = crate::Reg<pur::PurSpec>;
#[doc = "PUR"]
pub mod pur;
#[doc = "PDR (rw) register accessor: PDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdr`]
module"]
#[doc(alias = "PDR")]
pub type Pdr = crate::Reg<pdr::PdrSpec>;
#[doc = "PDR"]
pub mod pdr;
#[doc = "ODR (rw) register accessor: ODR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odr`]
module"]
#[doc(alias = "ODR")]
pub type Odr = crate::Reg<odr::OdrSpec>;
#[doc = "ODR"]
pub mod odr;
#[doc = "DRVR (rw) register accessor: DRVR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`drvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drvr`]
module"]
#[doc(alias = "DRVR")]
pub type Drvr = crate::Reg<drvr::DrvrSpec>;
#[doc = "DRVR"]
pub mod drvr;
#[doc = "LOCKR (rw) register accessor: LOCKR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lockr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lockr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lockr`]
module"]
#[doc(alias = "LOCKR")]
pub type Lockr = crate::Reg<lockr::LockrSpec>;
#[doc = "LOCKR"]
pub mod lockr;
#[doc = "DINR (rw) register accessor: DINR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dinr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr`]
module"]
#[doc(alias = "DINR")]
pub type Dinr = crate::Reg<dinr::DinrSpec>;
#[doc = "DINR"]
pub mod dinr;
#[doc = "DOUTR (rw) register accessor: DOUTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr`]
module"]
#[doc(alias = "DOUTR")]
pub type Doutr = crate::Reg<doutr::DoutrSpec>;
#[doc = "DOUTR"]
pub mod doutr;
#[doc = "SRR (rw) register accessor: SRR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srr`]
module"]
#[doc(alias = "SRR")]
pub type Srr = crate::Reg<srr::SrrSpec>;
#[doc = "SRR"]
pub mod srr;
#[doc = "RR (rw) register accessor: RR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rr`]
module"]
#[doc(alias = "RR")]
pub type Rr = crate::Reg<rr::RrSpec>;
#[doc = "RR"]
pub mod rr;
