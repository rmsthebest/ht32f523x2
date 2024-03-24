#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tadr: Tadr,
    wrdr: Wrdr,
    _reserved2: [u8; 0x04],
    ocmr: Ocmr,
    opcr: Opcr,
    oier: Oier,
    oisr: Oisr,
    _reserved6: [u8; 0x04],
    ppsr0: Ppsr0,
    ppsr1: Ppsr1,
    ppsr2: Ppsr2,
    ppsr3: Ppsr3,
    cpsr: Cpsr,
    _reserved11: [u8; 0xcc],
    vmcr: Vmcr,
    _reserved12: [u8; 0x7c],
    mdid: Mdid,
    pnsr: Pnsr,
    pssr: Pssr,
    _reserved15: [u8; 0x74],
    cfcr: Cfcr,
    _reserved16: [u8; 0x010c],
    cid0: Cid0,
    cid1: Cid1,
    cid2: Cid2,
    cid3: Cid3,
}
impl RegisterBlock {
    #[doc = "0x00 - TADR"]
    #[inline(always)]
    pub const fn tadr(&self) -> &Tadr {
        &self.tadr
    }
    #[doc = "0x04 - WRDR"]
    #[inline(always)]
    pub const fn wrdr(&self) -> &Wrdr {
        &self.wrdr
    }
    #[doc = "0x0c - OCMR"]
    #[inline(always)]
    pub const fn ocmr(&self) -> &Ocmr {
        &self.ocmr
    }
    #[doc = "0x10 - OPCR"]
    #[inline(always)]
    pub const fn opcr(&self) -> &Opcr {
        &self.opcr
    }
    #[doc = "0x14 - OIER"]
    #[inline(always)]
    pub const fn oier(&self) -> &Oier {
        &self.oier
    }
    #[doc = "0x18 - OISR"]
    #[inline(always)]
    pub const fn oisr(&self) -> &Oisr {
        &self.oisr
    }
    #[doc = "0x20 - PPSR0"]
    #[inline(always)]
    pub const fn ppsr0(&self) -> &Ppsr0 {
        &self.ppsr0
    }
    #[doc = "0x24 - PPSR1"]
    #[inline(always)]
    pub const fn ppsr1(&self) -> &Ppsr1 {
        &self.ppsr1
    }
    #[doc = "0x28 - PPSR2"]
    #[inline(always)]
    pub const fn ppsr2(&self) -> &Ppsr2 {
        &self.ppsr2
    }
    #[doc = "0x2c - PPSR3"]
    #[inline(always)]
    pub const fn ppsr3(&self) -> &Ppsr3 {
        &self.ppsr3
    }
    #[doc = "0x30 - CPSR"]
    #[inline(always)]
    pub const fn cpsr(&self) -> &Cpsr {
        &self.cpsr
    }
    #[doc = "0x100 - VMCR"]
    #[inline(always)]
    pub const fn vmcr(&self) -> &Vmcr {
        &self.vmcr
    }
    #[doc = "0x180 - MDID"]
    #[inline(always)]
    pub const fn mdid(&self) -> &Mdid {
        &self.mdid
    }
    #[doc = "0x184 - PNSR"]
    #[inline(always)]
    pub const fn pnsr(&self) -> &Pnsr {
        &self.pnsr
    }
    #[doc = "0x188 - PSSR"]
    #[inline(always)]
    pub const fn pssr(&self) -> &Pssr {
        &self.pssr
    }
    #[doc = "0x200 - CFCR"]
    #[inline(always)]
    pub const fn cfcr(&self) -> &Cfcr {
        &self.cfcr
    }
    #[doc = "0x310 - CID0"]
    #[inline(always)]
    pub const fn cid0(&self) -> &Cid0 {
        &self.cid0
    }
    #[doc = "0x314 - CID1"]
    #[inline(always)]
    pub const fn cid1(&self) -> &Cid1 {
        &self.cid1
    }
    #[doc = "0x318 - CID2"]
    #[inline(always)]
    pub const fn cid2(&self) -> &Cid2 {
        &self.cid2
    }
    #[doc = "0x31c - CID3"]
    #[inline(always)]
    pub const fn cid3(&self) -> &Cid3 {
        &self.cid3
    }
}
#[doc = "TADR (rw) register accessor: TADR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tadr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tadr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tadr`]
module"]
#[doc(alias = "TADR")]
pub type Tadr = crate::Reg<tadr::TadrSpec>;
#[doc = "TADR"]
pub mod tadr;
#[doc = "WRDR (rw) register accessor: WRDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrdr`]
module"]
#[doc(alias = "WRDR")]
pub type Wrdr = crate::Reg<wrdr::WrdrSpec>;
#[doc = "WRDR"]
pub mod wrdr;
#[doc = "OCMR (rw) register accessor: OCMR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocmr`]
module"]
#[doc(alias = "OCMR")]
pub type Ocmr = crate::Reg<ocmr::OcmrSpec>;
#[doc = "OCMR"]
pub mod ocmr;
#[doc = "OPCR (rw) register accessor: OPCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opcr`]
module"]
#[doc(alias = "OPCR")]
pub type Opcr = crate::Reg<opcr::OpcrSpec>;
#[doc = "OPCR"]
pub mod opcr;
#[doc = "OIER (rw) register accessor: OIER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oier`]
module"]
#[doc(alias = "OIER")]
pub type Oier = crate::Reg<oier::OierSpec>;
#[doc = "OIER"]
pub mod oier;
#[doc = "OISR (rw) register accessor: OISR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oisr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oisr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oisr`]
module"]
#[doc(alias = "OISR")]
pub type Oisr = crate::Reg<oisr::OisrSpec>;
#[doc = "OISR"]
pub mod oisr;
#[doc = "PPSR0 (rw) register accessor: PPSR0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppsr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppsr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppsr0`]
module"]
#[doc(alias = "PPSR0")]
pub type Ppsr0 = crate::Reg<ppsr0::Ppsr0Spec>;
#[doc = "PPSR0"]
pub mod ppsr0;
#[doc = "PPSR1 (rw) register accessor: PPSR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppsr1`]
module"]
#[doc(alias = "PPSR1")]
pub type Ppsr1 = crate::Reg<ppsr1::Ppsr1Spec>;
#[doc = "PPSR1"]
pub mod ppsr1;
#[doc = "PPSR2 (rw) register accessor: PPSR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppsr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppsr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppsr2`]
module"]
#[doc(alias = "PPSR2")]
pub type Ppsr2 = crate::Reg<ppsr2::Ppsr2Spec>;
#[doc = "PPSR2"]
pub mod ppsr2;
#[doc = "PPSR3 (rw) register accessor: PPSR3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppsr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppsr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppsr3`]
module"]
#[doc(alias = "PPSR3")]
pub type Ppsr3 = crate::Reg<ppsr3::Ppsr3Spec>;
#[doc = "PPSR3"]
pub mod ppsr3;
#[doc = "CPSR (rw) register accessor: CPSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsr`]
module"]
#[doc(alias = "CPSR")]
pub type Cpsr = crate::Reg<cpsr::CpsrSpec>;
#[doc = "CPSR"]
pub mod cpsr;
#[doc = "VMCR (rw) register accessor: VMCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vmcr`]
module"]
#[doc(alias = "VMCR")]
pub type Vmcr = crate::Reg<vmcr::VmcrSpec>;
#[doc = "VMCR"]
pub mod vmcr;
#[doc = "MDID (rw) register accessor: MDID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdid`]
module"]
#[doc(alias = "MDID")]
pub type Mdid = crate::Reg<mdid::MdidSpec>;
#[doc = "MDID"]
pub mod mdid;
#[doc = "PNSR (rw) register accessor: PNSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pnsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pnsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pnsr`]
module"]
#[doc(alias = "PNSR")]
pub type Pnsr = crate::Reg<pnsr::PnsrSpec>;
#[doc = "PNSR"]
pub mod pnsr;
#[doc = "PSSR (rw) register accessor: PSSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pssr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pssr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pssr`]
module"]
#[doc(alias = "PSSR")]
pub type Pssr = crate::Reg<pssr::PssrSpec>;
#[doc = "PSSR"]
pub mod pssr;
#[doc = "CFCR (rw) register accessor: CFCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfcr`]
module"]
#[doc(alias = "CFCR")]
pub type Cfcr = crate::Reg<cfcr::CfcrSpec>;
#[doc = "CFCR"]
pub mod cfcr;
#[doc = "CID0 (rw) register accessor: CID0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cid0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid0`]
module"]
#[doc(alias = "CID0")]
pub type Cid0 = crate::Reg<cid0::Cid0Spec>;
#[doc = "CID0"]
pub mod cid0;
#[doc = "CID1 (rw) register accessor: CID1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cid1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid1`]
module"]
#[doc(alias = "CID1")]
pub type Cid1 = crate::Reg<cid1::Cid1Spec>;
#[doc = "CID1"]
pub mod cid1;
#[doc = "CID2 (rw) register accessor: CID2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cid2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid2`]
module"]
#[doc(alias = "CID2")]
pub type Cid2 = crate::Reg<cid2::Cid2Spec>;
#[doc = "CID2"]
pub mod cid2;
#[doc = "CID3 (rw) register accessor: CID3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cid3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid3`]
module"]
#[doc(alias = "CID3")]
pub type Cid3 = crate::Reg<cid3::Cid3Spec>;
#[doc = "CID3"]
pub mod cid3;
