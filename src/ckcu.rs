#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gcfgr: Gcfgr,
    gccr: Gccr,
    gcsr: Gcsr,
    gcir: Gcir,
    _reserved4: [u8; 0x08],
    pllcfgr: Pllcfgr,
    pllcr: Pllcr,
    ahbcfgr: Ahbcfgr,
    ahbccr: Ahbccr,
    apbcfgr: Apbcfgr,
    apbccr0: Apbccr0,
    apbccr1: Apbccr1,
    ckst: Ckst,
    apbpcsr0: Apbpcsr0,
    apbpcsr1: Apbpcsr1,
    hsicr: Hsicr,
    hsiatcr: Hsiatcr,
    _reserved16: [u8; 0x02b8],
    lpcr: Lpcr,
    mcudbgcr: Mcudbgcr,
}
impl RegisterBlock {
    #[doc = "0x00 - GCFGR"]
    #[inline(always)]
    pub const fn gcfgr(&self) -> &Gcfgr {
        &self.gcfgr
    }
    #[doc = "0x04 - GCCR"]
    #[inline(always)]
    pub const fn gccr(&self) -> &Gccr {
        &self.gccr
    }
    #[doc = "0x08 - GCSR"]
    #[inline(always)]
    pub const fn gcsr(&self) -> &Gcsr {
        &self.gcsr
    }
    #[doc = "0x0c - GCIR"]
    #[inline(always)]
    pub const fn gcir(&self) -> &Gcir {
        &self.gcir
    }
    #[doc = "0x18 - PLLCFGR"]
    #[inline(always)]
    pub const fn pllcfgr(&self) -> &Pllcfgr {
        &self.pllcfgr
    }
    #[doc = "0x1c - PLLCR"]
    #[inline(always)]
    pub const fn pllcr(&self) -> &Pllcr {
        &self.pllcr
    }
    #[doc = "0x20 - AHBCFGR"]
    #[inline(always)]
    pub const fn ahbcfgr(&self) -> &Ahbcfgr {
        &self.ahbcfgr
    }
    #[doc = "0x24 - AHBCCR"]
    #[inline(always)]
    pub const fn ahbccr(&self) -> &Ahbccr {
        &self.ahbccr
    }
    #[doc = "0x28 - APBCFGR"]
    #[inline(always)]
    pub const fn apbcfgr(&self) -> &Apbcfgr {
        &self.apbcfgr
    }
    #[doc = "0x2c - APBCCR0"]
    #[inline(always)]
    pub const fn apbccr0(&self) -> &Apbccr0 {
        &self.apbccr0
    }
    #[doc = "0x30 - APBCCR1"]
    #[inline(always)]
    pub const fn apbccr1(&self) -> &Apbccr1 {
        &self.apbccr1
    }
    #[doc = "0x34 - CKST"]
    #[inline(always)]
    pub const fn ckst(&self) -> &Ckst {
        &self.ckst
    }
    #[doc = "0x38 - APBPCSR0"]
    #[inline(always)]
    pub const fn apbpcsr0(&self) -> &Apbpcsr0 {
        &self.apbpcsr0
    }
    #[doc = "0x3c - APBPCSR1"]
    #[inline(always)]
    pub const fn apbpcsr1(&self) -> &Apbpcsr1 {
        &self.apbpcsr1
    }
    #[doc = "0x40 - HSICR"]
    #[inline(always)]
    pub const fn hsicr(&self) -> &Hsicr {
        &self.hsicr
    }
    #[doc = "0x44 - HSIATCR"]
    #[inline(always)]
    pub const fn hsiatcr(&self) -> &Hsiatcr {
        &self.hsiatcr
    }
    #[doc = "0x300 - LPCR"]
    #[inline(always)]
    pub const fn lpcr(&self) -> &Lpcr {
        &self.lpcr
    }
    #[doc = "0x304 - MCUDBGCR"]
    #[inline(always)]
    pub const fn mcudbgcr(&self) -> &Mcudbgcr {
        &self.mcudbgcr
    }
}
#[doc = "GCFGR (rw) register accessor: GCFGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcfgr`]
module"]
#[doc(alias = "GCFGR")]
pub type Gcfgr = crate::Reg<gcfgr::GcfgrSpec>;
#[doc = "GCFGR"]
pub mod gcfgr;
#[doc = "GCCR (rw) register accessor: GCCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gccr`]
module"]
#[doc(alias = "GCCR")]
pub type Gccr = crate::Reg<gccr::GccrSpec>;
#[doc = "GCCR"]
pub mod gccr;
#[doc = "GCSR (rw) register accessor: GCSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcsr`]
module"]
#[doc(alias = "GCSR")]
pub type Gcsr = crate::Reg<gcsr::GcsrSpec>;
#[doc = "GCSR"]
pub mod gcsr;
#[doc = "GCIR (rw) register accessor: GCIR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcir`]
module"]
#[doc(alias = "GCIR")]
pub type Gcir = crate::Reg<gcir::GcirSpec>;
#[doc = "GCIR"]
pub mod gcir;
#[doc = "PLLCFGR (rw) register accessor: PLLCFGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcfgr`]
module"]
#[doc(alias = "PLLCFGR")]
pub type Pllcfgr = crate::Reg<pllcfgr::PllcfgrSpec>;
#[doc = "PLLCFGR"]
pub mod pllcfgr;
#[doc = "PLLCR (rw) register accessor: PLLCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcr`]
module"]
#[doc(alias = "PLLCR")]
pub type Pllcr = crate::Reg<pllcr::PllcrSpec>;
#[doc = "PLLCR"]
pub mod pllcr;
#[doc = "AHBCFGR (rw) register accessor: AHBCFGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbcfgr`]
module"]
#[doc(alias = "AHBCFGR")]
pub type Ahbcfgr = crate::Reg<ahbcfgr::AhbcfgrSpec>;
#[doc = "AHBCFGR"]
pub mod ahbcfgr;
#[doc = "AHBCCR (rw) register accessor: AHBCCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbccr`]
module"]
#[doc(alias = "AHBCCR")]
pub type Ahbccr = crate::Reg<ahbccr::AhbccrSpec>;
#[doc = "AHBCCR"]
pub mod ahbccr;
#[doc = "APBCFGR (rw) register accessor: APBCFGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbcfgr`]
module"]
#[doc(alias = "APBCFGR")]
pub type Apbcfgr = crate::Reg<apbcfgr::ApbcfgrSpec>;
#[doc = "APBCFGR"]
pub mod apbcfgr;
#[doc = "APBCCR0 (rw) register accessor: APBCCR0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbccr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbccr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbccr0`]
module"]
#[doc(alias = "APBCCR0")]
pub type Apbccr0 = crate::Reg<apbccr0::Apbccr0Spec>;
#[doc = "APBCCR0"]
pub mod apbccr0;
#[doc = "APBCCR1 (rw) register accessor: APBCCR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbccr1`]
module"]
#[doc(alias = "APBCCR1")]
pub type Apbccr1 = crate::Reg<apbccr1::Apbccr1Spec>;
#[doc = "APBCCR1"]
pub mod apbccr1;
#[doc = "CKST (rw) register accessor: CKST\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckst`]
module"]
#[doc(alias = "CKST")]
pub type Ckst = crate::Reg<ckst::CkstSpec>;
#[doc = "CKST"]
pub mod ckst;
#[doc = "APBPCSR0 (rw) register accessor: APBPCSR0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbpcsr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbpcsr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbpcsr0`]
module"]
#[doc(alias = "APBPCSR0")]
pub type Apbpcsr0 = crate::Reg<apbpcsr0::Apbpcsr0Spec>;
#[doc = "APBPCSR0"]
pub mod apbpcsr0;
#[doc = "APBPCSR1 (rw) register accessor: APBPCSR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbpcsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbpcsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbpcsr1`]
module"]
#[doc(alias = "APBPCSR1")]
pub type Apbpcsr1 = crate::Reg<apbpcsr1::Apbpcsr1Spec>;
#[doc = "APBPCSR1"]
pub mod apbpcsr1;
#[doc = "HSICR (rw) register accessor: HSICR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsicr`]
module"]
#[doc(alias = "HSICR")]
pub type Hsicr = crate::Reg<hsicr::HsicrSpec>;
#[doc = "HSICR"]
pub mod hsicr;
#[doc = "HSIATCR (rw) register accessor: HSIATCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsiatcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsiatcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsiatcr`]
module"]
#[doc(alias = "HSIATCR")]
pub type Hsiatcr = crate::Reg<hsiatcr::HsiatcrSpec>;
#[doc = "HSIATCR"]
pub mod hsiatcr;
#[doc = "LPCR (rw) register accessor: LPCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpcr`]
module"]
#[doc(alias = "LPCR")]
pub type Lpcr = crate::Reg<lpcr::LpcrSpec>;
#[doc = "LPCR"]
pub mod lpcr;
#[doc = "MCUDBGCR (rw) register accessor: MCUDBGCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcudbgcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcudbgcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcudbgcr`]
module"]
#[doc(alias = "MCUDBGCR")]
pub type Mcudbgcr = crate::Reg<mcudbgcr::McudbgcrSpec>;
#[doc = "MCUDBGCR"]
pub mod mcudbgcr;
