#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dfsr: Dfsr,
}
impl RegisterBlock {
    #[doc = "0x00 - DFSR"]
    #[inline(always)]
    pub const fn dfsr(&self) -> &Dfsr {
        &self.dfsr
    }
}
#[doc = "DFSR (rw) register accessor: DFSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsr`]
module"]
#[doc(alias = "DFSR")]
pub type Dfsr = crate::Reg<dfsr::DfsrSpec>;
#[doc = "DFSR"]
pub mod dfsr;
