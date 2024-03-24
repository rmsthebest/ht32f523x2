#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    grsr: Grsr,
    ahbprstr: Ahbprstr,
    apbprstr0: Apbprstr0,
    apbprstr1: Apbprstr1,
}
impl RegisterBlock {
    #[doc = "0x00 - GRSR"]
    #[inline(always)]
    pub const fn grsr(&self) -> &Grsr {
        &self.grsr
    }
    #[doc = "0x04 - AHBPRSTR"]
    #[inline(always)]
    pub const fn ahbprstr(&self) -> &Ahbprstr {
        &self.ahbprstr
    }
    #[doc = "0x08 - APBPRSTR0"]
    #[inline(always)]
    pub const fn apbprstr0(&self) -> &Apbprstr0 {
        &self.apbprstr0
    }
    #[doc = "0x0c - APBPRSTR1"]
    #[inline(always)]
    pub const fn apbprstr1(&self) -> &Apbprstr1 {
        &self.apbprstr1
    }
}
#[doc = "GRSR (rw) register accessor: GRSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grsr`]
module"]
#[doc(alias = "GRSR")]
pub type Grsr = crate::Reg<grsr::GrsrSpec>;
#[doc = "GRSR"]
pub mod grsr;
#[doc = "AHBPRSTR (rw) register accessor: AHBPRSTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbprstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbprstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbprstr`]
module"]
#[doc(alias = "AHBPRSTR")]
pub type Ahbprstr = crate::Reg<ahbprstr::AhbprstrSpec>;
#[doc = "AHBPRSTR"]
pub mod ahbprstr;
#[doc = "APBPRSTR0 (rw) register accessor: APBPRSTR0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbprstr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbprstr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbprstr0`]
module"]
#[doc(alias = "APBPRSTR0")]
pub type Apbprstr0 = crate::Reg<apbprstr0::Apbprstr0Spec>;
#[doc = "APBPRSTR0"]
pub mod apbprstr0;
#[doc = "APBPRSTR1 (rw) register accessor: APBPRSTR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbprstr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbprstr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbprstr1`]
module"]
#[doc(alias = "APBPRSTR1")]
pub type Apbprstr1 = crate::Reg<apbprstr1::Apbprstr1Spec>;
#[doc = "APBPRSTR1"]
pub mod apbprstr1;
