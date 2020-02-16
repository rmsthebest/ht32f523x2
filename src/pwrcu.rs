#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWRCU_BAKSR"]
    pub pwrcu_baksr: PWRCU_BAKSR,
    #[doc = "0x04 - PWRCU_BAKCR"]
    pub pwrcu_bakcr: PWRCU_BAKCR,
    #[doc = "0x08 - PWRCU_BAKTEST"]
    pub pwrcu_baktest: PWRCU_BAKTEST,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - PWRCU_LVDCSR"]
    pub pwrcu_lvdcsr: PWRCU_LVDCSR,
    _reserved4: [u8; 236usize],
    #[doc = "0x100 - PWRCU_BAKREG0"]
    pub pwrcu_bakreg0: PWRCU_BAKREG0,
    #[doc = "0x104 - PWRCU_BAKREG1"]
    pub pwrcu_bakreg1: PWRCU_BAKREG1,
    #[doc = "0x108 - PWRCU_BAKREG2"]
    pub pwrcu_bakreg2: PWRCU_BAKREG2,
    #[doc = "0x10c - PWRCU_BAKREG3"]
    pub pwrcu_bakreg3: PWRCU_BAKREG3,
    #[doc = "0x110 - PWRCU_BAKREG4"]
    pub pwrcu_bakreg4: PWRCU_BAKREG4,
    #[doc = "0x114 - PWRCU_BAKREG5"]
    pub pwrcu_bakreg5: PWRCU_BAKREG5,
    #[doc = "0x118 - PWRCU_BAKREG6"]
    pub pwrcu_bakreg6: PWRCU_BAKREG6,
    #[doc = "0x11c - PWRCU_BAKREG7"]
    pub pwrcu_bakreg7: PWRCU_BAKREG7,
    #[doc = "0x120 - PWRCU_BAKREG8"]
    pub pwrcu_bakreg8: PWRCU_BAKREG8,
    #[doc = "0x124 - PWRCU_BAKREG9"]
    pub pwrcu_bakreg9: PWRCU_BAKREG9,
}
#[doc = "PWRCU_BAKSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcu_baksr](pwrcu_baksr) module"]
pub type PWRCU_BAKSR = crate::Reg<u32, _PWRCU_BAKSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCU_BAKSR;
#[doc = "`read()` method returns [pwrcu_baksr::R](pwrcu_baksr::R) reader structure"]
impl crate::Readable for PWRCU_BAKSR {}
#[doc = "`write(|w| ..)` method takes [pwrcu_baksr::W](pwrcu_baksr::W) writer structure"]
impl crate::Writable for PWRCU_BAKSR {}
#[doc = "PWRCU_BAKSR"]
pub mod pwrcu_baksr;
#[doc = "PWRCU_BAKCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcu_bakcr](pwrcu_bakcr) module"]
pub type PWRCU_BAKCR = crate::Reg<u32, _PWRCU_BAKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCU_BAKCR;
#[doc = "`read()` method returns [pwrcu_bakcr::R](pwrcu_bakcr::R) reader structure"]
impl crate::Readable for PWRCU_BAKCR {}
#[doc = "`write(|w| ..)` method takes [pwrcu_bakcr::W](pwrcu_bakcr::W) writer structure"]
impl crate::Writable for PWRCU_BAKCR {}
#[doc = "PWRCU_BAKCR"]
pub mod pwrcu_bakcr;
#[doc = "PWRCU_BAKTEST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcu_baktest](pwrcu_baktest) module"]
pub type PWRCU_BAKTEST = crate::Reg<u32, _PWRCU_BAKTEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCU_BAKTEST;
#[doc = "`read()` method returns [pwrcu_baktest::R](pwrcu_baktest::R) reader structure"]
impl crate::Readable for PWRCU_BAKTEST {}
#[doc = "`write(|w| ..)` method takes [pwrcu_baktest::W](pwrcu_baktest::W) writer structure"]
impl crate::Writable for PWRCU_BAKTEST {}
#[doc = "PWRCU_BAKTEST"]
pub mod pwrcu_baktest;
#[doc = "PWRCU_LVDCSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcu_lvdcsr](pwrcu_lvdcsr) module"]
pub type PWRCU_LVDCSR = crate::Reg<u32, _PWRCU_LVDCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCU_LVDCSR;
#[doc = "`read()` method returns [pwrcu_lvdcsr::R](pwrcu_lvdcsr::R) reader structure"]
impl crate::Readable for PWRCU_LVDCSR {}
#[doc = "`write(|w| ..)` method takes [pwrcu_lvdcsr::W](pwrcu_lvdcsr::W) writer structure"]
impl crate::Writable for PWRCU_LVDCSR {}
#[doc = "PWRCU_LVDCSR"]
pub mod pwrcu_lvdcsr;
#[doc = "PWRCU_BAKREG0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcu_bakreg0](pwrcu_bakreg0) module"]
pub type PWRCU_BAKREG0 = crate::Reg<u32, _PWRCU_BAKREG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCU_BAKREG0;
#[doc = "`read()` method returns [pwrcu_bakreg0::R](pwrcu_bakreg0::R) reader structure"]
impl crate::Readable for PWRCU_BAKREG0 {}
#[doc = "`write(|w| ..)` method takes [pwrcu_bakreg0::W](pwrcu_bakreg0::W) writer structure"]
impl crate::Writable for PWRCU_BAKREG0 {}
#[doc = "PWRCU_BAKREG0"]
pub mod pwrcu_bakreg0;
#[doc = "PWRCU_BAKREG1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcu_bakreg1](pwrcu_bakreg1) module"]
pub type PWRCU_BAKREG1 = crate::Reg<u32, _PWRCU_BAKREG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCU_BAKREG1;
#[doc = "`read()` method returns [pwrcu_bakreg1::R](pwrcu_bakreg1::R) reader structure"]
impl crate::Readable for PWRCU_BAKREG1 {}
#[doc = "`write(|w| ..)` method takes [pwrcu_bakreg1::W](pwrcu_bakreg1::W) writer structure"]
impl crate::Writable for PWRCU_BAKREG1 {}
#[doc = "PWRCU_BAKREG1"]
pub mod pwrcu_bakreg1;
#[doc = "PWRCU_BAKREG2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcu_bakreg2](pwrcu_bakreg2) module"]
pub type PWRCU_BAKREG2 = crate::Reg<u32, _PWRCU_BAKREG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCU_BAKREG2;
#[doc = "`read()` method returns [pwrcu_bakreg2::R](pwrcu_bakreg2::R) reader structure"]
impl crate::Readable for PWRCU_BAKREG2 {}
#[doc = "`write(|w| ..)` method takes [pwrcu_bakreg2::W](pwrcu_bakreg2::W) writer structure"]
impl crate::Writable for PWRCU_BAKREG2 {}
#[doc = "PWRCU_BAKREG2"]
pub mod pwrcu_bakreg2;
#[doc = "PWRCU_BAKREG3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcu_bakreg3](pwrcu_bakreg3) module"]
pub type PWRCU_BAKREG3 = crate::Reg<u32, _PWRCU_BAKREG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCU_BAKREG3;
#[doc = "`read()` method returns [pwrcu_bakreg3::R](pwrcu_bakreg3::R) reader structure"]
impl crate::Readable for PWRCU_BAKREG3 {}
#[doc = "`write(|w| ..)` method takes [pwrcu_bakreg3::W](pwrcu_bakreg3::W) writer structure"]
impl crate::Writable for PWRCU_BAKREG3 {}
#[doc = "PWRCU_BAKREG3"]
pub mod pwrcu_bakreg3;
#[doc = "PWRCU_BAKREG4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcu_bakreg4](pwrcu_bakreg4) module"]
pub type PWRCU_BAKREG4 = crate::Reg<u32, _PWRCU_BAKREG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCU_BAKREG4;
#[doc = "`read()` method returns [pwrcu_bakreg4::R](pwrcu_bakreg4::R) reader structure"]
impl crate::Readable for PWRCU_BAKREG4 {}
#[doc = "`write(|w| ..)` method takes [pwrcu_bakreg4::W](pwrcu_bakreg4::W) writer structure"]
impl crate::Writable for PWRCU_BAKREG4 {}
#[doc = "PWRCU_BAKREG4"]
pub mod pwrcu_bakreg4;
#[doc = "PWRCU_BAKREG5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcu_bakreg5](pwrcu_bakreg5) module"]
pub type PWRCU_BAKREG5 = crate::Reg<u32, _PWRCU_BAKREG5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCU_BAKREG5;
#[doc = "`read()` method returns [pwrcu_bakreg5::R](pwrcu_bakreg5::R) reader structure"]
impl crate::Readable for PWRCU_BAKREG5 {}
#[doc = "`write(|w| ..)` method takes [pwrcu_bakreg5::W](pwrcu_bakreg5::W) writer structure"]
impl crate::Writable for PWRCU_BAKREG5 {}
#[doc = "PWRCU_BAKREG5"]
pub mod pwrcu_bakreg5;
#[doc = "PWRCU_BAKREG6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcu_bakreg6](pwrcu_bakreg6) module"]
pub type PWRCU_BAKREG6 = crate::Reg<u32, _PWRCU_BAKREG6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCU_BAKREG6;
#[doc = "`read()` method returns [pwrcu_bakreg6::R](pwrcu_bakreg6::R) reader structure"]
impl crate::Readable for PWRCU_BAKREG6 {}
#[doc = "`write(|w| ..)` method takes [pwrcu_bakreg6::W](pwrcu_bakreg6::W) writer structure"]
impl crate::Writable for PWRCU_BAKREG6 {}
#[doc = "PWRCU_BAKREG6"]
pub mod pwrcu_bakreg6;
#[doc = "PWRCU_BAKREG7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcu_bakreg7](pwrcu_bakreg7) module"]
pub type PWRCU_BAKREG7 = crate::Reg<u32, _PWRCU_BAKREG7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCU_BAKREG7;
#[doc = "`read()` method returns [pwrcu_bakreg7::R](pwrcu_bakreg7::R) reader structure"]
impl crate::Readable for PWRCU_BAKREG7 {}
#[doc = "`write(|w| ..)` method takes [pwrcu_bakreg7::W](pwrcu_bakreg7::W) writer structure"]
impl crate::Writable for PWRCU_BAKREG7 {}
#[doc = "PWRCU_BAKREG7"]
pub mod pwrcu_bakreg7;
#[doc = "PWRCU_BAKREG8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcu_bakreg8](pwrcu_bakreg8) module"]
pub type PWRCU_BAKREG8 = crate::Reg<u32, _PWRCU_BAKREG8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCU_BAKREG8;
#[doc = "`read()` method returns [pwrcu_bakreg8::R](pwrcu_bakreg8::R) reader structure"]
impl crate::Readable for PWRCU_BAKREG8 {}
#[doc = "`write(|w| ..)` method takes [pwrcu_bakreg8::W](pwrcu_bakreg8::W) writer structure"]
impl crate::Writable for PWRCU_BAKREG8 {}
#[doc = "PWRCU_BAKREG8"]
pub mod pwrcu_bakreg8;
#[doc = "PWRCU_BAKREG9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcu_bakreg9](pwrcu_bakreg9) module"]
pub type PWRCU_BAKREG9 = crate::Reg<u32, _PWRCU_BAKREG9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCU_BAKREG9;
#[doc = "`read()` method returns [pwrcu_bakreg9::R](pwrcu_bakreg9::R) reader structure"]
impl crate::Readable for PWRCU_BAKREG9 {}
#[doc = "`write(|w| ..)` method takes [pwrcu_bakreg9::W](pwrcu_bakreg9::W) writer structure"]
impl crate::Writable for PWRCU_BAKREG9 {}
#[doc = "PWRCU_BAKREG9"]
pub mod pwrcu_bakreg9;
