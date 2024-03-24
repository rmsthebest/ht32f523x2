#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr0: Cr0,
    valr0: Valr0,
    ier0: Ier0,
    tfr0: Tfr0,
    _reserved4: [u8; 0xf0],
    cr1: Cr1,
    valr1: Valr1,
    ier1: Ier1,
    tfr1: Tfr1,
}
impl RegisterBlock {
    #[doc = "0x00 - CR0"]
    #[inline(always)]
    pub const fn cr0(&self) -> &Cr0 {
        &self.cr0
    }
    #[doc = "0x04 - VALR0"]
    #[inline(always)]
    pub const fn valr0(&self) -> &Valr0 {
        &self.valr0
    }
    #[doc = "0x08 - IER0"]
    #[inline(always)]
    pub const fn ier0(&self) -> &Ier0 {
        &self.ier0
    }
    #[doc = "0x0c - TFR0"]
    #[inline(always)]
    pub const fn tfr0(&self) -> &Tfr0 {
        &self.tfr0
    }
    #[doc = "0x100 - CR1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x104 - VALR1"]
    #[inline(always)]
    pub const fn valr1(&self) -> &Valr1 {
        &self.valr1
    }
    #[doc = "0x108 - IER1"]
    #[inline(always)]
    pub const fn ier1(&self) -> &Ier1 {
        &self.ier1
    }
    #[doc = "0x10c - TFR1"]
    #[inline(always)]
    pub const fn tfr1(&self) -> &Tfr1 {
        &self.tfr1
    }
}
#[doc = "CR0 (rw) register accessor: CR0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr0`]
module"]
#[doc(alias = "CR0")]
pub type Cr0 = crate::Reg<cr0::Cr0Spec>;
#[doc = "CR0"]
pub mod cr0;
#[doc = "VALR0 (rw) register accessor: VALR0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`valr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`valr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@valr0`]
module"]
#[doc(alias = "VALR0")]
pub type Valr0 = crate::Reg<valr0::Valr0Spec>;
#[doc = "VALR0"]
pub mod valr0;
#[doc = "IER0 (rw) register accessor: IER0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier0`]
module"]
#[doc(alias = "IER0")]
pub type Ier0 = crate::Reg<ier0::Ier0Spec>;
#[doc = "IER0"]
pub mod ier0;
#[doc = "TFR0 (rw) register accessor: TFR0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tfr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfr0`]
module"]
#[doc(alias = "TFR0")]
pub type Tfr0 = crate::Reg<tfr0::Tfr0Spec>;
#[doc = "TFR0"]
pub mod tfr0;
#[doc = "CR1 (rw) register accessor: CR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "CR1"]
pub mod cr1;
#[doc = "VALR1 (rw) register accessor: VALR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`valr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`valr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@valr1`]
module"]
#[doc(alias = "VALR1")]
pub type Valr1 = crate::Reg<valr1::Valr1Spec>;
#[doc = "VALR1"]
pub mod valr1;
#[doc = "IER1 (rw) register accessor: IER1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier1`]
module"]
#[doc(alias = "IER1")]
pub type Ier1 = crate::Reg<ier1::Ier1Spec>;
#[doc = "IER1"]
pub mod ier1;
#[doc = "TFR1 (rw) register accessor: TFR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tfr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfr1`]
module"]
#[doc(alias = "TFR1")]
pub type Tfr1 = crate::Reg<tfr1::Tfr1Spec>;
#[doc = "TFR1"]
pub mod tfr1;
