#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CNTCFR"]
    pub cntcfr: CNTCFR,
    #[doc = "0x04 - MDCFR"]
    pub mdcfr: MDCFR,
    #[doc = "0x08 - TRCFR"]
    pub trcfr: TRCFR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - CTR"]
    pub ctr: CTR,
    _reserved4: [u8; 12usize],
    #[doc = "0x20 - CH0ICFR"]
    pub ch0icfr: CH0ICFR,
    #[doc = "0x24 - CH1ICFR"]
    pub ch1icfr: CH1ICFR,
    #[doc = "0x28 - CH2ICFR"]
    pub ch2icfr: CH2ICFR,
    #[doc = "0x2c - CH3ICFR"]
    pub ch3icfr: CH3ICFR,
    _reserved8: [u8; 16usize],
    #[doc = "0x40 - CH0OCFR"]
    pub ch0ocfr: CH0OCFR,
    #[doc = "0x44 - CH1OCFR"]
    pub ch1ocfr: CH1OCFR,
    #[doc = "0x48 - CH2OCFR"]
    pub ch2ocfr: CH2OCFR,
    #[doc = "0x4c - CH3OCFR"]
    pub ch3ocfr: CH3OCFR,
    #[doc = "0x50 - CHCTR"]
    pub chctr: CHCTR,
    #[doc = "0x54 - CHPOLR"]
    pub chpolr: CHPOLR,
    _reserved14: [u8; 20usize],
    #[doc = "0x6c - CHBRKCFR"]
    pub chbrkcfr: CHBRKCFR,
    #[doc = "0x70 - CHBRKCTR"]
    pub chbrkctr: CHBRKCTR,
    #[doc = "0x74 - DICTR"]
    pub dictr: DICTR,
    #[doc = "0x78 - EVGR"]
    pub evgr: EVGR,
    #[doc = "0x7c - INTSR"]
    pub intsr: INTSR,
    #[doc = "0x80 - CNTR"]
    pub cntr: CNTR,
    #[doc = "0x84 - PSCR"]
    pub pscr: PSCR,
    #[doc = "0x88 - CRR"]
    pub crr: CRR,
    #[doc = "0x8c - REPR"]
    pub repr: REPR,
    #[doc = "0x90 - CH0CCR"]
    pub ch0ccr: CH0CCR,
    #[doc = "0x94 - CH1CCR"]
    pub ch1ccr: CH1CCR,
    #[doc = "0x98 - CH2CCR"]
    pub ch2ccr: CH2CCR,
    #[doc = "0x9c - CH3CCR"]
    pub ch3ccr: CH3CCR,
    #[doc = "0xa0 - CH0ACR"]
    pub ch0acr: CH0ACR,
    #[doc = "0xa4 - CH1ACR"]
    pub ch1acr: CH1ACR,
    #[doc = "0xa8 - CH2ACR"]
    pub ch2acr: CH2ACR,
    #[doc = "0xac - CH3ACR"]
    pub ch3acr: CH3ACR,
}
#[doc = "CNTCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntcfr](cntcfr) module"]
pub type CNTCFR = crate::Reg<u32, _CNTCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTCFR;
#[doc = "`read()` method returns [cntcfr::R](cntcfr::R) reader structure"]
impl crate::Readable for CNTCFR {}
#[doc = "`write(|w| ..)` method takes [cntcfr::W](cntcfr::W) writer structure"]
impl crate::Writable for CNTCFR {}
#[doc = "CNTCFR"]
pub mod cntcfr;
#[doc = "MDCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdcfr](mdcfr) module"]
pub type MDCFR = crate::Reg<u32, _MDCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDCFR;
#[doc = "`read()` method returns [mdcfr::R](mdcfr::R) reader structure"]
impl crate::Readable for MDCFR {}
#[doc = "`write(|w| ..)` method takes [mdcfr::W](mdcfr::W) writer structure"]
impl crate::Writable for MDCFR {}
#[doc = "MDCFR"]
pub mod mdcfr;
#[doc = "TRCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trcfr](trcfr) module"]
pub type TRCFR = crate::Reg<u32, _TRCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRCFR;
#[doc = "`read()` method returns [trcfr::R](trcfr::R) reader structure"]
impl crate::Readable for TRCFR {}
#[doc = "`write(|w| ..)` method takes [trcfr::W](trcfr::W) writer structure"]
impl crate::Writable for TRCFR {}
#[doc = "TRCFR"]
pub mod trcfr;
#[doc = "CTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr](ctr) module"]
pub type CTR = crate::Reg<u32, _CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTR;
#[doc = "`read()` method returns [ctr::R](ctr::R) reader structure"]
impl crate::Readable for CTR {}
#[doc = "`write(|w| ..)` method takes [ctr::W](ctr::W) writer structure"]
impl crate::Writable for CTR {}
#[doc = "CTR"]
pub mod ctr;
#[doc = "CH0ICFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0icfr](ch0icfr) module"]
pub type CH0ICFR = crate::Reg<u32, _CH0ICFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0ICFR;
#[doc = "`read()` method returns [ch0icfr::R](ch0icfr::R) reader structure"]
impl crate::Readable for CH0ICFR {}
#[doc = "`write(|w| ..)` method takes [ch0icfr::W](ch0icfr::W) writer structure"]
impl crate::Writable for CH0ICFR {}
#[doc = "CH0ICFR"]
pub mod ch0icfr;
#[doc = "CH1ICFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1icfr](ch1icfr) module"]
pub type CH1ICFR = crate::Reg<u32, _CH1ICFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1ICFR;
#[doc = "`read()` method returns [ch1icfr::R](ch1icfr::R) reader structure"]
impl crate::Readable for CH1ICFR {}
#[doc = "`write(|w| ..)` method takes [ch1icfr::W](ch1icfr::W) writer structure"]
impl crate::Writable for CH1ICFR {}
#[doc = "CH1ICFR"]
pub mod ch1icfr;
#[doc = "CH2ICFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2icfr](ch2icfr) module"]
pub type CH2ICFR = crate::Reg<u32, _CH2ICFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2ICFR;
#[doc = "`read()` method returns [ch2icfr::R](ch2icfr::R) reader structure"]
impl crate::Readable for CH2ICFR {}
#[doc = "`write(|w| ..)` method takes [ch2icfr::W](ch2icfr::W) writer structure"]
impl crate::Writable for CH2ICFR {}
#[doc = "CH2ICFR"]
pub mod ch2icfr;
#[doc = "CH3ICFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3icfr](ch3icfr) module"]
pub type CH3ICFR = crate::Reg<u32, _CH3ICFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3ICFR;
#[doc = "`read()` method returns [ch3icfr::R](ch3icfr::R) reader structure"]
impl crate::Readable for CH3ICFR {}
#[doc = "`write(|w| ..)` method takes [ch3icfr::W](ch3icfr::W) writer structure"]
impl crate::Writable for CH3ICFR {}
#[doc = "CH3ICFR"]
pub mod ch3icfr;
#[doc = "CH0OCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0ocfr](ch0ocfr) module"]
pub type CH0OCFR = crate::Reg<u32, _CH0OCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0OCFR;
#[doc = "`read()` method returns [ch0ocfr::R](ch0ocfr::R) reader structure"]
impl crate::Readable for CH0OCFR {}
#[doc = "`write(|w| ..)` method takes [ch0ocfr::W](ch0ocfr::W) writer structure"]
impl crate::Writable for CH0OCFR {}
#[doc = "CH0OCFR"]
pub mod ch0ocfr;
#[doc = "CH1OCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1ocfr](ch1ocfr) module"]
pub type CH1OCFR = crate::Reg<u32, _CH1OCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1OCFR;
#[doc = "`read()` method returns [ch1ocfr::R](ch1ocfr::R) reader structure"]
impl crate::Readable for CH1OCFR {}
#[doc = "`write(|w| ..)` method takes [ch1ocfr::W](ch1ocfr::W) writer structure"]
impl crate::Writable for CH1OCFR {}
#[doc = "CH1OCFR"]
pub mod ch1ocfr;
#[doc = "CH2OCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2ocfr](ch2ocfr) module"]
pub type CH2OCFR = crate::Reg<u32, _CH2OCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2OCFR;
#[doc = "`read()` method returns [ch2ocfr::R](ch2ocfr::R) reader structure"]
impl crate::Readable for CH2OCFR {}
#[doc = "`write(|w| ..)` method takes [ch2ocfr::W](ch2ocfr::W) writer structure"]
impl crate::Writable for CH2OCFR {}
#[doc = "CH2OCFR"]
pub mod ch2ocfr;
#[doc = "CH3OCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3ocfr](ch3ocfr) module"]
pub type CH3OCFR = crate::Reg<u32, _CH3OCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3OCFR;
#[doc = "`read()` method returns [ch3ocfr::R](ch3ocfr::R) reader structure"]
impl crate::Readable for CH3OCFR {}
#[doc = "`write(|w| ..)` method takes [ch3ocfr::W](ch3ocfr::W) writer structure"]
impl crate::Writable for CH3OCFR {}
#[doc = "CH3OCFR"]
pub mod ch3ocfr;
#[doc = "CHCTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctr](chctr) module"]
pub type CHCTR = crate::Reg<u32, _CHCTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCTR;
#[doc = "`read()` method returns [chctr::R](chctr::R) reader structure"]
impl crate::Readable for CHCTR {}
#[doc = "`write(|w| ..)` method takes [chctr::W](chctr::W) writer structure"]
impl crate::Writable for CHCTR {}
#[doc = "CHCTR"]
pub mod chctr;
#[doc = "CHPOLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chpolr](chpolr) module"]
pub type CHPOLR = crate::Reg<u32, _CHPOLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHPOLR;
#[doc = "`read()` method returns [chpolr::R](chpolr::R) reader structure"]
impl crate::Readable for CHPOLR {}
#[doc = "`write(|w| ..)` method takes [chpolr::W](chpolr::W) writer structure"]
impl crate::Writable for CHPOLR {}
#[doc = "CHPOLR"]
pub mod chpolr;
#[doc = "CHBRKCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chbrkcfr](chbrkcfr) module"]
pub type CHBRKCFR = crate::Reg<u32, _CHBRKCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHBRKCFR;
#[doc = "`read()` method returns [chbrkcfr::R](chbrkcfr::R) reader structure"]
impl crate::Readable for CHBRKCFR {}
#[doc = "`write(|w| ..)` method takes [chbrkcfr::W](chbrkcfr::W) writer structure"]
impl crate::Writable for CHBRKCFR {}
#[doc = "CHBRKCFR"]
pub mod chbrkcfr;
#[doc = "CHBRKCTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chbrkctr](chbrkctr) module"]
pub type CHBRKCTR = crate::Reg<u32, _CHBRKCTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHBRKCTR;
#[doc = "`read()` method returns [chbrkctr::R](chbrkctr::R) reader structure"]
impl crate::Readable for CHBRKCTR {}
#[doc = "`write(|w| ..)` method takes [chbrkctr::W](chbrkctr::W) writer structure"]
impl crate::Writable for CHBRKCTR {}
#[doc = "CHBRKCTR"]
pub mod chbrkctr;
#[doc = "DICTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dictr](dictr) module"]
pub type DICTR = crate::Reg<u32, _DICTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DICTR;
#[doc = "`read()` method returns [dictr::R](dictr::R) reader structure"]
impl crate::Readable for DICTR {}
#[doc = "`write(|w| ..)` method takes [dictr::W](dictr::W) writer structure"]
impl crate::Writable for DICTR {}
#[doc = "DICTR"]
pub mod dictr;
#[doc = "EVGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evgr](evgr) module"]
pub type EVGR = crate::Reg<u32, _EVGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVGR;
#[doc = "`read()` method returns [evgr::R](evgr::R) reader structure"]
impl crate::Readable for EVGR {}
#[doc = "`write(|w| ..)` method takes [evgr::W](evgr::W) writer structure"]
impl crate::Writable for EVGR {}
#[doc = "EVGR"]
pub mod evgr;
#[doc = "INTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intsr](intsr) module"]
pub type INTSR = crate::Reg<u32, _INTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSR;
#[doc = "`read()` method returns [intsr::R](intsr::R) reader structure"]
impl crate::Readable for INTSR {}
#[doc = "`write(|w| ..)` method takes [intsr::W](intsr::W) writer structure"]
impl crate::Writable for INTSR {}
#[doc = "INTSR"]
pub mod intsr;
#[doc = "CNTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntr](cntr) module"]
pub type CNTR = crate::Reg<u32, _CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTR;
#[doc = "`read()` method returns [cntr::R](cntr::R) reader structure"]
impl crate::Readable for CNTR {}
#[doc = "`write(|w| ..)` method takes [cntr::W](cntr::W) writer structure"]
impl crate::Writable for CNTR {}
#[doc = "CNTR"]
pub mod cntr;
#[doc = "PSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pscr](pscr) module"]
pub type PSCR = crate::Reg<u32, _PSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSCR;
#[doc = "`read()` method returns [pscr::R](pscr::R) reader structure"]
impl crate::Readable for PSCR {}
#[doc = "`write(|w| ..)` method takes [pscr::W](pscr::W) writer structure"]
impl crate::Writable for PSCR {}
#[doc = "PSCR"]
pub mod pscr;
#[doc = "CRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crr](crr) module"]
pub type CRR = crate::Reg<u32, _CRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRR;
#[doc = "`read()` method returns [crr::R](crr::R) reader structure"]
impl crate::Readable for CRR {}
#[doc = "`write(|w| ..)` method takes [crr::W](crr::W) writer structure"]
impl crate::Writable for CRR {}
#[doc = "CRR"]
pub mod crr;
#[doc = "REPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [repr](repr) module"]
pub type REPR = crate::Reg<u32, _REPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REPR;
#[doc = "`read()` method returns [repr::R](repr::R) reader structure"]
impl crate::Readable for REPR {}
#[doc = "`write(|w| ..)` method takes [repr::W](repr::W) writer structure"]
impl crate::Writable for REPR {}
#[doc = "REPR"]
pub mod repr;
#[doc = "CH0CCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0ccr](ch0ccr) module"]
pub type CH0CCR = crate::Reg<u32, _CH0CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0CCR;
#[doc = "`read()` method returns [ch0ccr::R](ch0ccr::R) reader structure"]
impl crate::Readable for CH0CCR {}
#[doc = "`write(|w| ..)` method takes [ch0ccr::W](ch0ccr::W) writer structure"]
impl crate::Writable for CH0CCR {}
#[doc = "CH0CCR"]
pub mod ch0ccr;
#[doc = "CH1CCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1ccr](ch1ccr) module"]
pub type CH1CCR = crate::Reg<u32, _CH1CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1CCR;
#[doc = "`read()` method returns [ch1ccr::R](ch1ccr::R) reader structure"]
impl crate::Readable for CH1CCR {}
#[doc = "`write(|w| ..)` method takes [ch1ccr::W](ch1ccr::W) writer structure"]
impl crate::Writable for CH1CCR {}
#[doc = "CH1CCR"]
pub mod ch1ccr;
#[doc = "CH2CCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2ccr](ch2ccr) module"]
pub type CH2CCR = crate::Reg<u32, _CH2CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2CCR;
#[doc = "`read()` method returns [ch2ccr::R](ch2ccr::R) reader structure"]
impl crate::Readable for CH2CCR {}
#[doc = "`write(|w| ..)` method takes [ch2ccr::W](ch2ccr::W) writer structure"]
impl crate::Writable for CH2CCR {}
#[doc = "CH2CCR"]
pub mod ch2ccr;
#[doc = "CH3CCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3ccr](ch3ccr) module"]
pub type CH3CCR = crate::Reg<u32, _CH3CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3CCR;
#[doc = "`read()` method returns [ch3ccr::R](ch3ccr::R) reader structure"]
impl crate::Readable for CH3CCR {}
#[doc = "`write(|w| ..)` method takes [ch3ccr::W](ch3ccr::W) writer structure"]
impl crate::Writable for CH3CCR {}
#[doc = "CH3CCR"]
pub mod ch3ccr;
#[doc = "CH0ACR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0acr](ch0acr) module"]
pub type CH0ACR = crate::Reg<u32, _CH0ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0ACR;
#[doc = "`read()` method returns [ch0acr::R](ch0acr::R) reader structure"]
impl crate::Readable for CH0ACR {}
#[doc = "`write(|w| ..)` method takes [ch0acr::W](ch0acr::W) writer structure"]
impl crate::Writable for CH0ACR {}
#[doc = "CH0ACR"]
pub mod ch0acr;
#[doc = "CH1ACR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1acr](ch1acr) module"]
pub type CH1ACR = crate::Reg<u32, _CH1ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1ACR;
#[doc = "`read()` method returns [ch1acr::R](ch1acr::R) reader structure"]
impl crate::Readable for CH1ACR {}
#[doc = "`write(|w| ..)` method takes [ch1acr::W](ch1acr::W) writer structure"]
impl crate::Writable for CH1ACR {}
#[doc = "CH1ACR"]
pub mod ch1acr;
#[doc = "CH2ACR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2acr](ch2acr) module"]
pub type CH2ACR = crate::Reg<u32, _CH2ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2ACR;
#[doc = "`read()` method returns [ch2acr::R](ch2acr::R) reader structure"]
impl crate::Readable for CH2ACR {}
#[doc = "`write(|w| ..)` method takes [ch2acr::W](ch2acr::W) writer structure"]
impl crate::Writable for CH2ACR {}
#[doc = "CH2ACR"]
pub mod ch2acr;
#[doc = "CH3ACR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3acr](ch3acr) module"]
pub type CH3ACR = crate::Reg<u32, _CH3ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3ACR;
#[doc = "`read()` method returns [ch3acr::R](ch3acr::R) reader structure"]
impl crate::Readable for CH3ACR {}
#[doc = "`write(|w| ..)` method takes [ch3acr::W](ch3acr::W) writer structure"]
impl crate::Writable for CH3ACR {}
#[doc = "CH3ACR"]
pub mod ch3acr;
