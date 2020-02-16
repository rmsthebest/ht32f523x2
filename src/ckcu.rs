#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GCFGR"]
    pub gcfgr: GCFGR,
    #[doc = "0x04 - GCCR"]
    pub gccr: GCCR,
    #[doc = "0x08 - GCSR"]
    pub gcsr: GCSR,
    #[doc = "0x0c - GCIR"]
    pub gcir: GCIR,
    _reserved4: [u8; 8usize],
    #[doc = "0x18 - PLLCFGR"]
    pub pllcfgr: PLLCFGR,
    #[doc = "0x1c - PLLCR"]
    pub pllcr: PLLCR,
    #[doc = "0x20 - AHBCFGR"]
    pub ahbcfgr: AHBCFGR,
    #[doc = "0x24 - AHBCCR"]
    pub ahbccr: AHBCCR,
    #[doc = "0x28 - APBCFGR"]
    pub apbcfgr: APBCFGR,
    #[doc = "0x2c - APBCCR0"]
    pub apbccr0: APBCCR0,
    #[doc = "0x30 - APBCCR1"]
    pub apbccr1: APBCCR1,
    #[doc = "0x34 - CKST"]
    pub ckst: CKST,
    #[doc = "0x38 - APBPCSR0"]
    pub apbpcsr0: APBPCSR0,
    #[doc = "0x3c - APBPCSR1"]
    pub apbpcsr1: APBPCSR1,
    #[doc = "0x40 - HSICR"]
    pub hsicr: HSICR,
    #[doc = "0x44 - HSIATCR"]
    pub hsiatcr: HSIATCR,
    _reserved16: [u8; 696usize],
    #[doc = "0x300 - LPCR"]
    pub lpcr: LPCR,
    #[doc = "0x304 - MCUDBGCR"]
    pub mcudbgcr: MCUDBGCR,
}
#[doc = "GCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcfgr](gcfgr) module"]
pub type GCFGR = crate::Reg<u32, _GCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCFGR;
#[doc = "`read()` method returns [gcfgr::R](gcfgr::R) reader structure"]
impl crate::Readable for GCFGR {}
#[doc = "`write(|w| ..)` method takes [gcfgr::W](gcfgr::W) writer structure"]
impl crate::Writable for GCFGR {}
#[doc = "GCFGR"]
pub mod gcfgr;
#[doc = "GCCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gccr](gccr) module"]
pub type GCCR = crate::Reg<u32, _GCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCCR;
#[doc = "`read()` method returns [gccr::R](gccr::R) reader structure"]
impl crate::Readable for GCCR {}
#[doc = "`write(|w| ..)` method takes [gccr::W](gccr::W) writer structure"]
impl crate::Writable for GCCR {}
#[doc = "GCCR"]
pub mod gccr;
#[doc = "GCSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcsr](gcsr) module"]
pub type GCSR = crate::Reg<u32, _GCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCSR;
#[doc = "`read()` method returns [gcsr::R](gcsr::R) reader structure"]
impl crate::Readable for GCSR {}
#[doc = "`write(|w| ..)` method takes [gcsr::W](gcsr::W) writer structure"]
impl crate::Writable for GCSR {}
#[doc = "GCSR"]
pub mod gcsr;
#[doc = "GCIR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcir](gcir) module"]
pub type GCIR = crate::Reg<u32, _GCIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCIR;
#[doc = "`read()` method returns [gcir::R](gcir::R) reader structure"]
impl crate::Readable for GCIR {}
#[doc = "`write(|w| ..)` method takes [gcir::W](gcir::W) writer structure"]
impl crate::Writable for GCIR {}
#[doc = "GCIR"]
pub mod gcir;
#[doc = "PLLCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcfgr](pllcfgr) module"]
pub type PLLCFGR = crate::Reg<u32, _PLLCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLCFGR;
#[doc = "`read()` method returns [pllcfgr::R](pllcfgr::R) reader structure"]
impl crate::Readable for PLLCFGR {}
#[doc = "`write(|w| ..)` method takes [pllcfgr::W](pllcfgr::W) writer structure"]
impl crate::Writable for PLLCFGR {}
#[doc = "PLLCFGR"]
pub mod pllcfgr;
#[doc = "PLLCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcr](pllcr) module"]
pub type PLLCR = crate::Reg<u32, _PLLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLCR;
#[doc = "`read()` method returns [pllcr::R](pllcr::R) reader structure"]
impl crate::Readable for PLLCR {}
#[doc = "`write(|w| ..)` method takes [pllcr::W](pllcr::W) writer structure"]
impl crate::Writable for PLLCR {}
#[doc = "PLLCR"]
pub mod pllcr;
#[doc = "AHBCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbcfgr](ahbcfgr) module"]
pub type AHBCFGR = crate::Reg<u32, _AHBCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBCFGR;
#[doc = "`read()` method returns [ahbcfgr::R](ahbcfgr::R) reader structure"]
impl crate::Readable for AHBCFGR {}
#[doc = "`write(|w| ..)` method takes [ahbcfgr::W](ahbcfgr::W) writer structure"]
impl crate::Writable for AHBCFGR {}
#[doc = "AHBCFGR"]
pub mod ahbcfgr;
#[doc = "AHBCCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbccr](ahbccr) module"]
pub type AHBCCR = crate::Reg<u32, _AHBCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBCCR;
#[doc = "`read()` method returns [ahbccr::R](ahbccr::R) reader structure"]
impl crate::Readable for AHBCCR {}
#[doc = "`write(|w| ..)` method takes [ahbccr::W](ahbccr::W) writer structure"]
impl crate::Writable for AHBCCR {}
#[doc = "AHBCCR"]
pub mod ahbccr;
#[doc = "APBCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbcfgr](apbcfgr) module"]
pub type APBCFGR = crate::Reg<u32, _APBCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBCFGR;
#[doc = "`read()` method returns [apbcfgr::R](apbcfgr::R) reader structure"]
impl crate::Readable for APBCFGR {}
#[doc = "`write(|w| ..)` method takes [apbcfgr::W](apbcfgr::W) writer structure"]
impl crate::Writable for APBCFGR {}
#[doc = "APBCFGR"]
pub mod apbcfgr;
#[doc = "APBCCR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbccr0](apbccr0) module"]
pub type APBCCR0 = crate::Reg<u32, _APBCCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBCCR0;
#[doc = "`read()` method returns [apbccr0::R](apbccr0::R) reader structure"]
impl crate::Readable for APBCCR0 {}
#[doc = "`write(|w| ..)` method takes [apbccr0::W](apbccr0::W) writer structure"]
impl crate::Writable for APBCCR0 {}
#[doc = "APBCCR0"]
pub mod apbccr0;
#[doc = "APBCCR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbccr1](apbccr1) module"]
pub type APBCCR1 = crate::Reg<u32, _APBCCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBCCR1;
#[doc = "`read()` method returns [apbccr1::R](apbccr1::R) reader structure"]
impl crate::Readable for APBCCR1 {}
#[doc = "`write(|w| ..)` method takes [apbccr1::W](apbccr1::W) writer structure"]
impl crate::Writable for APBCCR1 {}
#[doc = "APBCCR1"]
pub mod apbccr1;
#[doc = "CKST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckst](ckst) module"]
pub type CKST = crate::Reg<u32, _CKST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKST;
#[doc = "`read()` method returns [ckst::R](ckst::R) reader structure"]
impl crate::Readable for CKST {}
#[doc = "`write(|w| ..)` method takes [ckst::W](ckst::W) writer structure"]
impl crate::Writable for CKST {}
#[doc = "CKST"]
pub mod ckst;
#[doc = "APBPCSR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbpcsr0](apbpcsr0) module"]
pub type APBPCSR0 = crate::Reg<u32, _APBPCSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBPCSR0;
#[doc = "`read()` method returns [apbpcsr0::R](apbpcsr0::R) reader structure"]
impl crate::Readable for APBPCSR0 {}
#[doc = "`write(|w| ..)` method takes [apbpcsr0::W](apbpcsr0::W) writer structure"]
impl crate::Writable for APBPCSR0 {}
#[doc = "APBPCSR0"]
pub mod apbpcsr0;
#[doc = "APBPCSR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbpcsr1](apbpcsr1) module"]
pub type APBPCSR1 = crate::Reg<u32, _APBPCSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBPCSR1;
#[doc = "`read()` method returns [apbpcsr1::R](apbpcsr1::R) reader structure"]
impl crate::Readable for APBPCSR1 {}
#[doc = "`write(|w| ..)` method takes [apbpcsr1::W](apbpcsr1::W) writer structure"]
impl crate::Writable for APBPCSR1 {}
#[doc = "APBPCSR1"]
pub mod apbpcsr1;
#[doc = "HSICR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsicr](hsicr) module"]
pub type HSICR = crate::Reg<u32, _HSICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSICR;
#[doc = "`read()` method returns [hsicr::R](hsicr::R) reader structure"]
impl crate::Readable for HSICR {}
#[doc = "`write(|w| ..)` method takes [hsicr::W](hsicr::W) writer structure"]
impl crate::Writable for HSICR {}
#[doc = "HSICR"]
pub mod hsicr;
#[doc = "HSIATCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsiatcr](hsiatcr) module"]
pub type HSIATCR = crate::Reg<u32, _HSIATCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSIATCR;
#[doc = "`read()` method returns [hsiatcr::R](hsiatcr::R) reader structure"]
impl crate::Readable for HSIATCR {}
#[doc = "`write(|w| ..)` method takes [hsiatcr::W](hsiatcr::W) writer structure"]
impl crate::Writable for HSIATCR {}
#[doc = "HSIATCR"]
pub mod hsiatcr;
#[doc = "LPCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpcr](lpcr) module"]
pub type LPCR = crate::Reg<u32, _LPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPCR;
#[doc = "`read()` method returns [lpcr::R](lpcr::R) reader structure"]
impl crate::Readable for LPCR {}
#[doc = "`write(|w| ..)` method takes [lpcr::W](lpcr::W) writer structure"]
impl crate::Writable for LPCR {}
#[doc = "LPCR"]
pub mod lpcr;
#[doc = "MCUDBGCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcudbgcr](mcudbgcr) module"]
pub type MCUDBGCR = crate::Reg<u32, _MCUDBGCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCUDBGCR;
#[doc = "`read()` method returns [mcudbgcr::R](mcudbgcr::R) reader structure"]
impl crate::Readable for MCUDBGCR {}
#[doc = "`write(|w| ..)` method takes [mcudbgcr::W](mcudbgcr::W) writer structure"]
impl crate::Writable for MCUDBGCR {}
#[doc = "MCUDBGCR"]
pub mod mcudbgcr;
