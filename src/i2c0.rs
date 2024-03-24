#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    ier: Ier,
    addr: Addr,
    sr: Sr,
    shpgr: Shpgr,
    slpgr: Slpgr,
    dr: Dr,
    tar: Tar,
    addmr: Addmr,
    addsr: Addsr,
    tout: Tout,
}
impl RegisterBlock {
    #[doc = "0x00 - CR"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - IER"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x08 - ADDR"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x0c - SR"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x10 - SHPGR"]
    #[inline(always)]
    pub const fn shpgr(&self) -> &Shpgr {
        &self.shpgr
    }
    #[doc = "0x14 - SLPGR"]
    #[inline(always)]
    pub const fn slpgr(&self) -> &Slpgr {
        &self.slpgr
    }
    #[doc = "0x18 - DR"]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    #[doc = "0x1c - TAR"]
    #[inline(always)]
    pub const fn tar(&self) -> &Tar {
        &self.tar
    }
    #[doc = "0x20 - ADDMR"]
    #[inline(always)]
    pub const fn addmr(&self) -> &Addmr {
        &self.addmr
    }
    #[doc = "0x24 - ADDSR"]
    #[inline(always)]
    pub const fn addsr(&self) -> &Addsr {
        &self.addsr
    }
    #[doc = "0x28 - TOUT"]
    #[inline(always)]
    pub const fn tout(&self) -> &Tout {
        &self.tout
    }
}
#[doc = "CR (rw) register accessor: CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "CR"]
pub mod cr;
#[doc = "IER (rw) register accessor: IER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "IER"]
pub mod ier;
#[doc = "ADDR (rw) register accessor: ADDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "ADDR"]
pub mod addr;
#[doc = "SR (rw) register accessor: SR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "SR"]
pub mod sr;
#[doc = "SHPGR (rw) register accessor: SHPGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shpgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shpgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shpgr`]
module"]
#[doc(alias = "SHPGR")]
pub type Shpgr = crate::Reg<shpgr::ShpgrSpec>;
#[doc = "SHPGR"]
pub mod shpgr;
#[doc = "SLPGR (rw) register accessor: SLPGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slpgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slpgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slpgr`]
module"]
#[doc(alias = "SLPGR")]
pub type Slpgr = crate::Reg<slpgr::SlpgrSpec>;
#[doc = "SLPGR"]
pub mod slpgr;
#[doc = "DR (rw) register accessor: DR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "DR"]
pub mod dr;
#[doc = "TAR (rw) register accessor: TAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tar`]
module"]
#[doc(alias = "TAR")]
pub type Tar = crate::Reg<tar::TarSpec>;
#[doc = "TAR"]
pub mod tar;
#[doc = "ADDMR (rw) register accessor: ADDMR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addmr`]
module"]
#[doc(alias = "ADDMR")]
pub type Addmr = crate::Reg<addmr::AddmrSpec>;
#[doc = "ADDMR"]
pub mod addmr;
#[doc = "ADDSR (rw) register accessor: ADDSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addsr`]
module"]
#[doc(alias = "ADDSR")]
pub type Addsr = crate::Reg<addsr::AddsrSpec>;
#[doc = "ADDSR"]
pub mod addsr;
#[doc = "TOUT (rw) register accessor: TOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tout`]
module"]
#[doc(alias = "TOUT")]
pub type Tout = crate::Reg<tout::ToutSpec>;
#[doc = "TOUT"]
pub mod tout;
