#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR0"]
    pub cr0: CR0,
    #[doc = "0x04 - VALR0"]
    pub valr0: VALR0,
    #[doc = "0x08 - IER0"]
    pub ier0: IER0,
    #[doc = "0x0c - TFR0"]
    pub tfr0: TFR0,
    _reserved4: [u8; 240usize],
    #[doc = "0x100 - CR1"]
    pub cr1: CR1,
    #[doc = "0x104 - VALR1"]
    pub valr1: VALR1,
    #[doc = "0x108 - IER1"]
    pub ier1: IER1,
    #[doc = "0x10c - TFR1"]
    pub tfr1: TFR1,
}
#[doc = "CR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr0](cr0) module"]
pub type CR0 = crate::Reg<u32, _CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR0;
#[doc = "`read()` method returns [cr0::R](cr0::R) reader structure"]
impl crate::Readable for CR0 {}
#[doc = "`write(|w| ..)` method takes [cr0::W](cr0::W) writer structure"]
impl crate::Writable for CR0 {}
#[doc = "CR0"]
pub mod cr0;
#[doc = "VALR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [valr0](valr0) module"]
pub type VALR0 = crate::Reg<u32, _VALR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VALR0;
#[doc = "`read()` method returns [valr0::R](valr0::R) reader structure"]
impl crate::Readable for VALR0 {}
#[doc = "`write(|w| ..)` method takes [valr0::W](valr0::W) writer structure"]
impl crate::Writable for VALR0 {}
#[doc = "VALR0"]
pub mod valr0;
#[doc = "IER0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier0](ier0) module"]
pub type IER0 = crate::Reg<u32, _IER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER0;
#[doc = "`read()` method returns [ier0::R](ier0::R) reader structure"]
impl crate::Readable for IER0 {}
#[doc = "`write(|w| ..)` method takes [ier0::W](ier0::W) writer structure"]
impl crate::Writable for IER0 {}
#[doc = "IER0"]
pub mod ier0;
#[doc = "TFR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfr0](tfr0) module"]
pub type TFR0 = crate::Reg<u32, _TFR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFR0;
#[doc = "`read()` method returns [tfr0::R](tfr0::R) reader structure"]
impl crate::Readable for TFR0 {}
#[doc = "`write(|w| ..)` method takes [tfr0::W](tfr0::W) writer structure"]
impl crate::Writable for TFR0 {}
#[doc = "TFR0"]
pub mod tfr0;
#[doc = "CR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](cr1) module"]
pub type CR1 = crate::Reg<u32, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
#[doc = "`read()` method returns [cr1::R](cr1::R) reader structure"]
impl crate::Readable for CR1 {}
#[doc = "`write(|w| ..)` method takes [cr1::W](cr1::W) writer structure"]
impl crate::Writable for CR1 {}
#[doc = "CR1"]
pub mod cr1;
#[doc = "VALR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [valr1](valr1) module"]
pub type VALR1 = crate::Reg<u32, _VALR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VALR1;
#[doc = "`read()` method returns [valr1::R](valr1::R) reader structure"]
impl crate::Readable for VALR1 {}
#[doc = "`write(|w| ..)` method takes [valr1::W](valr1::W) writer structure"]
impl crate::Writable for VALR1 {}
#[doc = "VALR1"]
pub mod valr1;
#[doc = "IER1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier1](ier1) module"]
pub type IER1 = crate::Reg<u32, _IER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER1;
#[doc = "`read()` method returns [ier1::R](ier1::R) reader structure"]
impl crate::Readable for IER1 {}
#[doc = "`write(|w| ..)` method takes [ier1::W](ier1::W) writer structure"]
impl crate::Writable for IER1 {}
#[doc = "IER1"]
pub mod ier1;
#[doc = "TFR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfr1](tfr1) module"]
pub type TFR1 = crate::Reg<u32, _TFR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFR1;
#[doc = "`read()` method returns [tfr1::R](tfr1::R) reader structure"]
impl crate::Readable for TFR1 {}
#[doc = "`write(|w| ..)` method takes [tfr1::W](tfr1::W) writer structure"]
impl crate::Writable for TFR1 {}
#[doc = "TFR1"]
pub mod tfr1;
