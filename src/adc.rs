#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR"]
    pub cr: CR,
    #[doc = "0x04 - LST0"]
    pub lst0: LST0,
    #[doc = "0x08 - LST1"]
    pub lst1: LST1,
    _reserved3: [u8; 20usize],
    #[doc = "0x20 - STR"]
    pub str: STR,
    _reserved4: [u8; 12usize],
    #[doc = "0x30 - DR0"]
    pub dr0: DR0,
    #[doc = "0x34 - DR1"]
    pub dr1: DR1,
    #[doc = "0x38 - DR2"]
    pub dr2: DR2,
    #[doc = "0x3c - DR3"]
    pub dr3: DR3,
    #[doc = "0x40 - DR4"]
    pub dr4: DR4,
    #[doc = "0x44 - DR5"]
    pub dr5: DR5,
    #[doc = "0x48 - DR6"]
    pub dr6: DR6,
    #[doc = "0x4c - DR7"]
    pub dr7: DR7,
    _reserved12: [u8; 32usize],
    #[doc = "0x70 - TCR"]
    pub tcr: TCR,
    #[doc = "0x74 - TSR"]
    pub tsr: TSR,
    #[doc = "0x78 - WCR"]
    pub wcr: WCR,
    #[doc = "0x7c - TR"]
    pub tr: TR,
    #[doc = "0x80 - IMR"]
    pub ier: IER,
    #[doc = "0x84 - IRAW"]
    pub iraw: IRAW,
    #[doc = "0x88 - ISR"]
    pub isr: ISR,
    #[doc = "0x8c - ICLR"]
    pub iclr: ICLR,
    #[doc = "0x90 - DMAR"]
    pub dmar: DMAR,
}
#[doc = "CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "CR"]
pub mod cr;
#[doc = "LST0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lst0](lst0) module"]
pub type LST0 = crate::Reg<u32, _LST0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LST0;
#[doc = "`read()` method returns [lst0::R](lst0::R) reader structure"]
impl crate::Readable for LST0 {}
#[doc = "`write(|w| ..)` method takes [lst0::W](lst0::W) writer structure"]
impl crate::Writable for LST0 {}
#[doc = "LST0"]
pub mod lst0;
#[doc = "LST1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lst1](lst1) module"]
pub type LST1 = crate::Reg<u32, _LST1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LST1;
#[doc = "`read()` method returns [lst1::R](lst1::R) reader structure"]
impl crate::Readable for LST1 {}
#[doc = "`write(|w| ..)` method takes [lst1::W](lst1::W) writer structure"]
impl crate::Writable for LST1 {}
#[doc = "LST1"]
pub mod lst1;
#[doc = "STR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [str](str) module"]
pub type STR = crate::Reg<u32, _STR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STR;
#[doc = "`read()` method returns [str::R](str::R) reader structure"]
impl crate::Readable for STR {}
#[doc = "`write(|w| ..)` method takes [str::W](str::W) writer structure"]
impl crate::Writable for STR {}
#[doc = "STR"]
pub mod str;
#[doc = "DR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr0](dr0) module"]
pub type DR0 = crate::Reg<u32, _DR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR0;
#[doc = "`read()` method returns [dr0::R](dr0::R) reader structure"]
impl crate::Readable for DR0 {}
#[doc = "`write(|w| ..)` method takes [dr0::W](dr0::W) writer structure"]
impl crate::Writable for DR0 {}
#[doc = "DR0"]
pub mod dr0;
#[doc = "DR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr1](dr1) module"]
pub type DR1 = crate::Reg<u32, _DR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR1;
#[doc = "`read()` method returns [dr1::R](dr1::R) reader structure"]
impl crate::Readable for DR1 {}
#[doc = "`write(|w| ..)` method takes [dr1::W](dr1::W) writer structure"]
impl crate::Writable for DR1 {}
#[doc = "DR1"]
pub mod dr1;
#[doc = "DR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr2](dr2) module"]
pub type DR2 = crate::Reg<u32, _DR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR2;
#[doc = "`read()` method returns [dr2::R](dr2::R) reader structure"]
impl crate::Readable for DR2 {}
#[doc = "`write(|w| ..)` method takes [dr2::W](dr2::W) writer structure"]
impl crate::Writable for DR2 {}
#[doc = "DR2"]
pub mod dr2;
#[doc = "DR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr3](dr3) module"]
pub type DR3 = crate::Reg<u32, _DR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR3;
#[doc = "`read()` method returns [dr3::R](dr3::R) reader structure"]
impl crate::Readable for DR3 {}
#[doc = "`write(|w| ..)` method takes [dr3::W](dr3::W) writer structure"]
impl crate::Writable for DR3 {}
#[doc = "DR3"]
pub mod dr3;
#[doc = "DR4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr4](dr4) module"]
pub type DR4 = crate::Reg<u32, _DR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR4;
#[doc = "`read()` method returns [dr4::R](dr4::R) reader structure"]
impl crate::Readable for DR4 {}
#[doc = "`write(|w| ..)` method takes [dr4::W](dr4::W) writer structure"]
impl crate::Writable for DR4 {}
#[doc = "DR4"]
pub mod dr4;
#[doc = "DR5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr5](dr5) module"]
pub type DR5 = crate::Reg<u32, _DR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR5;
#[doc = "`read()` method returns [dr5::R](dr5::R) reader structure"]
impl crate::Readable for DR5 {}
#[doc = "`write(|w| ..)` method takes [dr5::W](dr5::W) writer structure"]
impl crate::Writable for DR5 {}
#[doc = "DR5"]
pub mod dr5;
#[doc = "DR6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr6](dr6) module"]
pub type DR6 = crate::Reg<u32, _DR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR6;
#[doc = "`read()` method returns [dr6::R](dr6::R) reader structure"]
impl crate::Readable for DR6 {}
#[doc = "`write(|w| ..)` method takes [dr6::W](dr6::W) writer structure"]
impl crate::Writable for DR6 {}
#[doc = "DR6"]
pub mod dr6;
#[doc = "DR7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr7](dr7) module"]
pub type DR7 = crate::Reg<u32, _DR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR7;
#[doc = "`read()` method returns [dr7::R](dr7::R) reader structure"]
impl crate::Readable for DR7 {}
#[doc = "`write(|w| ..)` method takes [dr7::W](dr7::W) writer structure"]
impl crate::Writable for DR7 {}
#[doc = "DR7"]
pub mod dr7;
#[doc = "TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr](tcr) module"]
pub type TCR = crate::Reg<u32, _TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCR;
#[doc = "`read()` method returns [tcr::R](tcr::R) reader structure"]
impl crate::Readable for TCR {}
#[doc = "`write(|w| ..)` method takes [tcr::W](tcr::W) writer structure"]
impl crate::Writable for TCR {}
#[doc = "TCR"]
pub mod tcr;
#[doc = "TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsr](tsr) module"]
pub type TSR = crate::Reg<u32, _TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSR;
#[doc = "`read()` method returns [tsr::R](tsr::R) reader structure"]
impl crate::Readable for TSR {}
#[doc = "`write(|w| ..)` method takes [tsr::W](tsr::W) writer structure"]
impl crate::Writable for TSR {}
#[doc = "TSR"]
pub mod tsr;
#[doc = "WCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wcr](wcr) module"]
pub type WCR = crate::Reg<u32, _WCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WCR;
#[doc = "`read()` method returns [wcr::R](wcr::R) reader structure"]
impl crate::Readable for WCR {}
#[doc = "`write(|w| ..)` method takes [wcr::W](wcr::W) writer structure"]
impl crate::Writable for WCR {}
#[doc = "WCR"]
pub mod wcr;
#[doc = "TR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr](tr) module"]
pub type TR = crate::Reg<u32, _TR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR;
#[doc = "`read()` method returns [tr::R](tr::R) reader structure"]
impl crate::Readable for TR {}
#[doc = "`write(|w| ..)` method takes [tr::W](tr::W) writer structure"]
impl crate::Writable for TR {}
#[doc = "TR"]
pub mod tr;
#[doc = "IMR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "IMR"]
pub mod ier;
#[doc = "IRAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iraw](iraw) module"]
pub type IRAW = crate::Reg<u32, _IRAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRAW;
#[doc = "`read()` method returns [iraw::R](iraw::R) reader structure"]
impl crate::Readable for IRAW {}
#[doc = "`write(|w| ..)` method takes [iraw::W](iraw::W) writer structure"]
impl crate::Writable for IRAW {}
#[doc = "IRAW"]
pub mod iraw;
#[doc = "ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "`write(|w| ..)` method takes [isr::W](isr::W) writer structure"]
impl crate::Writable for ISR {}
#[doc = "ISR"]
pub mod isr;
#[doc = "ICLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iclr](iclr) module"]
pub type ICLR = crate::Reg<u32, _ICLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICLR;
#[doc = "`read()` method returns [iclr::R](iclr::R) reader structure"]
impl crate::Readable for ICLR {}
#[doc = "`write(|w| ..)` method takes [iclr::W](iclr::W) writer structure"]
impl crate::Writable for ICLR {}
#[doc = "ICLR"]
pub mod iclr;
#[doc = "DMAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmar](dmar) module"]
pub type DMAR = crate::Reg<u32, _DMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAR;
#[doc = "`read()` method returns [dmar::R](dmar::R) reader structure"]
impl crate::Readable for DMAR {}
#[doc = "`write(|w| ..)` method takes [dmar::W](dmar::W) writer structure"]
impl crate::Writable for DMAR {}
#[doc = "DMAR"]
pub mod dmar;
