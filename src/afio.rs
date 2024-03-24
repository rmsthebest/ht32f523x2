#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    essr0: Essr0,
    essr1: Essr1,
    _reserved2: [u8; 0x18],
    gpacfglr: Gpacfglr,
    gpacfghr: Gpacfghr,
    gpbcfglr: Gpbcfglr,
    gpbcfghr: Gpbcfghr,
    gpccfglr: Gpccfglr,
    gpccfghr: Gpccfghr,
    gpdcfglr: Gpdcfglr,
    gpdcfghr: Gpdcfghr,
}
impl RegisterBlock {
    #[doc = "0x00 - ESSR0"]
    #[inline(always)]
    pub const fn essr0(&self) -> &Essr0 {
        &self.essr0
    }
    #[doc = "0x04 - ESSR1"]
    #[inline(always)]
    pub const fn essr1(&self) -> &Essr1 {
        &self.essr1
    }
    #[doc = "0x20 - GPACFGLR"]
    #[inline(always)]
    pub const fn gpacfglr(&self) -> &Gpacfglr {
        &self.gpacfglr
    }
    #[doc = "0x24 - GPACFGHR"]
    #[inline(always)]
    pub const fn gpacfghr(&self) -> &Gpacfghr {
        &self.gpacfghr
    }
    #[doc = "0x28 - GPBCFGLR"]
    #[inline(always)]
    pub const fn gpbcfglr(&self) -> &Gpbcfglr {
        &self.gpbcfglr
    }
    #[doc = "0x2c - GPBCFGHR"]
    #[inline(always)]
    pub const fn gpbcfghr(&self) -> &Gpbcfghr {
        &self.gpbcfghr
    }
    #[doc = "0x30 - GPCCFGLR"]
    #[inline(always)]
    pub const fn gpccfglr(&self) -> &Gpccfglr {
        &self.gpccfglr
    }
    #[doc = "0x34 - GPCCFGHR"]
    #[inline(always)]
    pub const fn gpccfghr(&self) -> &Gpccfghr {
        &self.gpccfghr
    }
    #[doc = "0x38 - GPDCFGLR"]
    #[inline(always)]
    pub const fn gpdcfglr(&self) -> &Gpdcfglr {
        &self.gpdcfglr
    }
    #[doc = "0x3c - GPDCFGHR"]
    #[inline(always)]
    pub const fn gpdcfghr(&self) -> &Gpdcfghr {
        &self.gpdcfghr
    }
}
#[doc = "ESSR0 (rw) register accessor: ESSR0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`essr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`essr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@essr0`]
module"]
#[doc(alias = "ESSR0")]
pub type Essr0 = crate::Reg<essr0::Essr0Spec>;
#[doc = "ESSR0"]
pub mod essr0;
#[doc = "ESSR1 (rw) register accessor: ESSR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`essr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`essr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@essr1`]
module"]
#[doc(alias = "ESSR1")]
pub type Essr1 = crate::Reg<essr1::Essr1Spec>;
#[doc = "ESSR1"]
pub mod essr1;
#[doc = "GPACFGLR (rw) register accessor: GPACFGLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpacfglr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpacfglr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpacfglr`]
module"]
#[doc(alias = "GPACFGLR")]
pub type Gpacfglr = crate::Reg<gpacfglr::GpacfglrSpec>;
#[doc = "GPACFGLR"]
pub mod gpacfglr;
#[doc = "GPACFGHR (rw) register accessor: GPACFGHR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpacfghr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpacfghr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpacfghr`]
module"]
#[doc(alias = "GPACFGHR")]
pub type Gpacfghr = crate::Reg<gpacfghr::GpacfghrSpec>;
#[doc = "GPACFGHR"]
pub mod gpacfghr;
#[doc = "GPBCFGLR (rw) register accessor: GPBCFGLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpbcfglr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpbcfglr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpbcfglr`]
module"]
#[doc(alias = "GPBCFGLR")]
pub type Gpbcfglr = crate::Reg<gpbcfglr::GpbcfglrSpec>;
#[doc = "GPBCFGLR"]
pub mod gpbcfglr;
#[doc = "GPBCFGHR (rw) register accessor: GPBCFGHR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpbcfghr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpbcfghr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpbcfghr`]
module"]
#[doc(alias = "GPBCFGHR")]
pub type Gpbcfghr = crate::Reg<gpbcfghr::GpbcfghrSpec>;
#[doc = "GPBCFGHR"]
pub mod gpbcfghr;
#[doc = "GPCCFGLR (rw) register accessor: GPCCFGLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpccfglr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpccfglr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpccfglr`]
module"]
#[doc(alias = "GPCCFGLR")]
pub type Gpccfglr = crate::Reg<gpccfglr::GpccfglrSpec>;
#[doc = "GPCCFGLR"]
pub mod gpccfglr;
#[doc = "GPCCFGHR (rw) register accessor: GPCCFGHR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpccfghr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpccfghr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpccfghr`]
module"]
#[doc(alias = "GPCCFGHR")]
pub type Gpccfghr = crate::Reg<gpccfghr::GpccfghrSpec>;
#[doc = "GPCCFGHR"]
pub mod gpccfghr;
#[doc = "GPDCFGLR (rw) register accessor: GPDCFGLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdcfglr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdcfglr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpdcfglr`]
module"]
#[doc(alias = "GPDCFGLR")]
pub type Gpdcfglr = crate::Reg<gpdcfglr::GpdcfglrSpec>;
#[doc = "GPDCFGLR"]
pub mod gpdcfglr;
#[doc = "GPDCFGHR (rw) register accessor: GPDCFGHR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdcfghr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdcfghr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpdcfghr`]
module"]
#[doc(alias = "GPDCFGHR")]
pub type Gpdcfghr = crate::Reg<gpdcfghr::GpdcfghrSpec>;
#[doc = "GPDCFGHR"]
pub mod gpdcfghr;
