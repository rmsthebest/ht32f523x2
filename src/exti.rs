#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CFGR0"]
    pub cfgr0: CFGR0,
    #[doc = "0x04 - CFGR1"]
    pub cfgr1: CFGR1,
    #[doc = "0x08 - CFGR2"]
    pub cfgr2: CFGR2,
    #[doc = "0x0c - CFGR3"]
    pub cfgr3: CFGR3,
    #[doc = "0x10 - CFGR4"]
    pub cfgr4: CFGR4,
    #[doc = "0x14 - CFGR5"]
    pub cfgr5: CFGR5,
    #[doc = "0x18 - CFGR6"]
    pub cfgr6: CFGR6,
    #[doc = "0x1c - CFGR7"]
    pub cfgr7: CFGR7,
    #[doc = "0x20 - CFGR8"]
    pub cfgr8: CFGR8,
    #[doc = "0x24 - CFGR9"]
    pub cfgr9: CFGR9,
    #[doc = "0x28 - CFGR10"]
    pub cfgr10: CFGR10,
    #[doc = "0x2c - CFGR11"]
    pub cfgr11: CFGR11,
    #[doc = "0x30 - CFGR12"]
    pub cfgr12: CFGR12,
    #[doc = "0x34 - CFGR13"]
    pub cfgr13: CFGR13,
    #[doc = "0x38 - CFGR14"]
    pub cfgr14: CFGR14,
    #[doc = "0x3c - CFGR15"]
    pub cfgr15: CFGR15,
    #[doc = "0x40 - CR"]
    pub cr: CR,
    #[doc = "0x44 - EDGEFLGR"]
    pub edgeflgr: EDGEFLGR,
    #[doc = "0x48 - EDGESR"]
    pub edgesr: EDGESR,
    #[doc = "0x4c - SSCR"]
    pub sscr: SSCR,
    #[doc = "0x50 - WAKUPCR"]
    pub wakupcr: WAKUPCR,
    #[doc = "0x54 - WAKUPPOLR"]
    pub wakuppolr: WAKUPPOLR,
    #[doc = "0x58 - WAKUPFLG"]
    pub wakupflg: WAKUPFLG,
}
#[doc = "CFGR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr0](cfgr0) module"]
pub type CFGR0 = crate::Reg<u32, _CFGR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR0;
#[doc = "`read()` method returns [cfgr0::R](cfgr0::R) reader structure"]
impl crate::Readable for CFGR0 {}
#[doc = "`write(|w| ..)` method takes [cfgr0::W](cfgr0::W) writer structure"]
impl crate::Writable for CFGR0 {}
#[doc = "CFGR0"]
pub mod cfgr0;
#[doc = "CFGR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](cfgr1) module"]
pub type CFGR1 = crate::Reg<u32, _CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR1;
#[doc = "`read()` method returns [cfgr1::R](cfgr1::R) reader structure"]
impl crate::Readable for CFGR1 {}
#[doc = "`write(|w| ..)` method takes [cfgr1::W](cfgr1::W) writer structure"]
impl crate::Writable for CFGR1 {}
#[doc = "CFGR1"]
pub mod cfgr1;
#[doc = "CFGR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](cfgr2) module"]
pub type CFGR2 = crate::Reg<u32, _CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR2;
#[doc = "`read()` method returns [cfgr2::R](cfgr2::R) reader structure"]
impl crate::Readable for CFGR2 {}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](cfgr2::W) writer structure"]
impl crate::Writable for CFGR2 {}
#[doc = "CFGR2"]
pub mod cfgr2;
#[doc = "CFGR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr3](cfgr3) module"]
pub type CFGR3 = crate::Reg<u32, _CFGR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR3;
#[doc = "`read()` method returns [cfgr3::R](cfgr3::R) reader structure"]
impl crate::Readable for CFGR3 {}
#[doc = "`write(|w| ..)` method takes [cfgr3::W](cfgr3::W) writer structure"]
impl crate::Writable for CFGR3 {}
#[doc = "CFGR3"]
pub mod cfgr3;
#[doc = "CFGR4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr4](cfgr4) module"]
pub type CFGR4 = crate::Reg<u32, _CFGR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR4;
#[doc = "`read()` method returns [cfgr4::R](cfgr4::R) reader structure"]
impl crate::Readable for CFGR4 {}
#[doc = "`write(|w| ..)` method takes [cfgr4::W](cfgr4::W) writer structure"]
impl crate::Writable for CFGR4 {}
#[doc = "CFGR4"]
pub mod cfgr4;
#[doc = "CFGR5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr5](cfgr5) module"]
pub type CFGR5 = crate::Reg<u32, _CFGR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR5;
#[doc = "`read()` method returns [cfgr5::R](cfgr5::R) reader structure"]
impl crate::Readable for CFGR5 {}
#[doc = "`write(|w| ..)` method takes [cfgr5::W](cfgr5::W) writer structure"]
impl crate::Writable for CFGR5 {}
#[doc = "CFGR5"]
pub mod cfgr5;
#[doc = "CFGR6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr6](cfgr6) module"]
pub type CFGR6 = crate::Reg<u32, _CFGR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR6;
#[doc = "`read()` method returns [cfgr6::R](cfgr6::R) reader structure"]
impl crate::Readable for CFGR6 {}
#[doc = "`write(|w| ..)` method takes [cfgr6::W](cfgr6::W) writer structure"]
impl crate::Writable for CFGR6 {}
#[doc = "CFGR6"]
pub mod cfgr6;
#[doc = "CFGR7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr7](cfgr7) module"]
pub type CFGR7 = crate::Reg<u32, _CFGR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR7;
#[doc = "`read()` method returns [cfgr7::R](cfgr7::R) reader structure"]
impl crate::Readable for CFGR7 {}
#[doc = "`write(|w| ..)` method takes [cfgr7::W](cfgr7::W) writer structure"]
impl crate::Writable for CFGR7 {}
#[doc = "CFGR7"]
pub mod cfgr7;
#[doc = "CFGR8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr8](cfgr8) module"]
pub type CFGR8 = crate::Reg<u32, _CFGR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR8;
#[doc = "`read()` method returns [cfgr8::R](cfgr8::R) reader structure"]
impl crate::Readable for CFGR8 {}
#[doc = "`write(|w| ..)` method takes [cfgr8::W](cfgr8::W) writer structure"]
impl crate::Writable for CFGR8 {}
#[doc = "CFGR8"]
pub mod cfgr8;
#[doc = "CFGR9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr9](cfgr9) module"]
pub type CFGR9 = crate::Reg<u32, _CFGR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR9;
#[doc = "`read()` method returns [cfgr9::R](cfgr9::R) reader structure"]
impl crate::Readable for CFGR9 {}
#[doc = "`write(|w| ..)` method takes [cfgr9::W](cfgr9::W) writer structure"]
impl crate::Writable for CFGR9 {}
#[doc = "CFGR9"]
pub mod cfgr9;
#[doc = "CFGR10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr10](cfgr10) module"]
pub type CFGR10 = crate::Reg<u32, _CFGR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR10;
#[doc = "`read()` method returns [cfgr10::R](cfgr10::R) reader structure"]
impl crate::Readable for CFGR10 {}
#[doc = "`write(|w| ..)` method takes [cfgr10::W](cfgr10::W) writer structure"]
impl crate::Writable for CFGR10 {}
#[doc = "CFGR10"]
pub mod cfgr10;
#[doc = "CFGR11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr11](cfgr11) module"]
pub type CFGR11 = crate::Reg<u32, _CFGR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR11;
#[doc = "`read()` method returns [cfgr11::R](cfgr11::R) reader structure"]
impl crate::Readable for CFGR11 {}
#[doc = "`write(|w| ..)` method takes [cfgr11::W](cfgr11::W) writer structure"]
impl crate::Writable for CFGR11 {}
#[doc = "CFGR11"]
pub mod cfgr11;
#[doc = "CFGR12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr12](cfgr12) module"]
pub type CFGR12 = crate::Reg<u32, _CFGR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR12;
#[doc = "`read()` method returns [cfgr12::R](cfgr12::R) reader structure"]
impl crate::Readable for CFGR12 {}
#[doc = "`write(|w| ..)` method takes [cfgr12::W](cfgr12::W) writer structure"]
impl crate::Writable for CFGR12 {}
#[doc = "CFGR12"]
pub mod cfgr12;
#[doc = "CFGR13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr13](cfgr13) module"]
pub type CFGR13 = crate::Reg<u32, _CFGR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR13;
#[doc = "`read()` method returns [cfgr13::R](cfgr13::R) reader structure"]
impl crate::Readable for CFGR13 {}
#[doc = "`write(|w| ..)` method takes [cfgr13::W](cfgr13::W) writer structure"]
impl crate::Writable for CFGR13 {}
#[doc = "CFGR13"]
pub mod cfgr13;
#[doc = "CFGR14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr14](cfgr14) module"]
pub type CFGR14 = crate::Reg<u32, _CFGR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR14;
#[doc = "`read()` method returns [cfgr14::R](cfgr14::R) reader structure"]
impl crate::Readable for CFGR14 {}
#[doc = "`write(|w| ..)` method takes [cfgr14::W](cfgr14::W) writer structure"]
impl crate::Writable for CFGR14 {}
#[doc = "CFGR14"]
pub mod cfgr14;
#[doc = "CFGR15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr15](cfgr15) module"]
pub type CFGR15 = crate::Reg<u32, _CFGR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR15;
#[doc = "`read()` method returns [cfgr15::R](cfgr15::R) reader structure"]
impl crate::Readable for CFGR15 {}
#[doc = "`write(|w| ..)` method takes [cfgr15::W](cfgr15::W) writer structure"]
impl crate::Writable for CFGR15 {}
#[doc = "CFGR15"]
pub mod cfgr15;
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
#[doc = "EDGEFLGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edgeflgr](edgeflgr) module"]
pub type EDGEFLGR = crate::Reg<u32, _EDGEFLGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EDGEFLGR;
#[doc = "`read()` method returns [edgeflgr::R](edgeflgr::R) reader structure"]
impl crate::Readable for EDGEFLGR {}
#[doc = "`write(|w| ..)` method takes [edgeflgr::W](edgeflgr::W) writer structure"]
impl crate::Writable for EDGEFLGR {}
#[doc = "EDGEFLGR"]
pub mod edgeflgr;
#[doc = "EDGESR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edgesr](edgesr) module"]
pub type EDGESR = crate::Reg<u32, _EDGESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EDGESR;
#[doc = "`read()` method returns [edgesr::R](edgesr::R) reader structure"]
impl crate::Readable for EDGESR {}
#[doc = "`write(|w| ..)` method takes [edgesr::W](edgesr::W) writer structure"]
impl crate::Writable for EDGESR {}
#[doc = "EDGESR"]
pub mod edgesr;
#[doc = "SSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sscr](sscr) module"]
pub type SSCR = crate::Reg<u32, _SSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSCR;
#[doc = "`read()` method returns [sscr::R](sscr::R) reader structure"]
impl crate::Readable for SSCR {}
#[doc = "`write(|w| ..)` method takes [sscr::W](sscr::W) writer structure"]
impl crate::Writable for SSCR {}
#[doc = "SSCR"]
pub mod sscr;
#[doc = "WAKUPCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakupcr](wakupcr) module"]
pub type WAKUPCR = crate::Reg<u32, _WAKUPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKUPCR;
#[doc = "`read()` method returns [wakupcr::R](wakupcr::R) reader structure"]
impl crate::Readable for WAKUPCR {}
#[doc = "`write(|w| ..)` method takes [wakupcr::W](wakupcr::W) writer structure"]
impl crate::Writable for WAKUPCR {}
#[doc = "WAKUPCR"]
pub mod wakupcr;
#[doc = "WAKUPPOLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakuppolr](wakuppolr) module"]
pub type WAKUPPOLR = crate::Reg<u32, _WAKUPPOLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKUPPOLR;
#[doc = "`read()` method returns [wakuppolr::R](wakuppolr::R) reader structure"]
impl crate::Readable for WAKUPPOLR {}
#[doc = "`write(|w| ..)` method takes [wakuppolr::W](wakuppolr::W) writer structure"]
impl crate::Writable for WAKUPPOLR {}
#[doc = "WAKUPPOLR"]
pub mod wakuppolr;
#[doc = "WAKUPFLG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakupflg](wakupflg) module"]
pub type WAKUPFLG = crate::Reg<u32, _WAKUPFLG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKUPFLG;
#[doc = "`read()` method returns [wakupflg::R](wakupflg::R) reader structure"]
impl crate::Readable for WAKUPFLG {}
#[doc = "`write(|w| ..)` method takes [wakupflg::W](wakupflg::W) writer structure"]
impl crate::Writable for WAKUPFLG {}
#[doc = "WAKUPFLG"]
pub mod wakupflg;
