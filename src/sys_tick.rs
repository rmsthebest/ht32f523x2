#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csr: Csr,
    rvr: Rvr,
    cvr: Cvr,
    calib: Calib,
}
impl RegisterBlock {
    #[doc = "0x00 - CSR"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x04 - RVR"]
    #[inline(always)]
    pub const fn rvr(&self) -> &Rvr {
        &self.rvr
    }
    #[doc = "0x08 - CVR"]
    #[inline(always)]
    pub const fn cvr(&self) -> &Cvr {
        &self.cvr
    }
    #[doc = "0x0c - CALIB"]
    #[inline(always)]
    pub const fn calib(&self) -> &Calib {
        &self.calib
    }
}
#[doc = "CSR (rw) register accessor: CSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "CSR"]
pub mod csr;
#[doc = "RVR (rw) register accessor: RVR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rvr`]
module"]
#[doc(alias = "RVR")]
pub type Rvr = crate::Reg<rvr::RvrSpec>;
#[doc = "RVR"]
pub mod rvr;
#[doc = "CVR (rw) register accessor: CVR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cvr`]
module"]
#[doc(alias = "CVR")]
pub type Cvr = crate::Reg<cvr::CvrSpec>;
#[doc = "CVR"]
pub mod cvr;
#[doc = "CALIB (rw) register accessor: CALIB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calib::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calib::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calib`]
module"]
#[doc(alias = "CALIB")]
pub type Calib = crate::Reg<calib::CalibSpec>;
#[doc = "CALIB"]
pub mod calib;
