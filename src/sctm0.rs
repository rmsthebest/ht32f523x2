#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CNTCFR"]
    pub cntcfr: CNTCFR,
    #[doc = "0x04 - GPTM_MDCFR"]
    pub gptm_mdcfr: GPTM_MDCFR,
    #[doc = "0x08 - GPTM_TRCFR"]
    pub gptm_trcfr: GPTM_TRCFR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - GPTM_CTR"]
    pub gptm_ctr: GPTM_CTR,
    _reserved4: [u8; 12usize],
    #[doc = "0x20 - GPTM_CH0ICFR"]
    pub gptm_ch0icfr: GPTM_CH0ICFR,
    _reserved5: [u8; 32usize],
    #[doc = "0x44 - GPTM_CH1OCFR"]
    pub gptm_ch1ocfr: GPTM_CH1OCFR,
    _reserved6: [u8; 8usize],
    #[doc = "0x50 - GPTM_CHCTR"]
    pub gptm_chctr: GPTM_CHCTR,
    #[doc = "0x54 - GPTM_CHPOLR"]
    pub gptm_chpolr: GPTM_CHPOLR,
    _reserved8: [u8; 28usize],
    #[doc = "0x74 - GPTM_DICTR"]
    pub gptm_dictr: GPTM_DICTR,
    #[doc = "0x78 - GPTM_EVGR"]
    pub gptm_evgr: GPTM_EVGR,
    #[doc = "0x7c - GPTM_INTSR"]
    pub gptm_intsr: GPTM_INTSR,
    #[doc = "0x80 - GPTM_CNTR"]
    pub gptm_cntr: GPTM_CNTR,
    #[doc = "0x84 - GPTM_PSCR"]
    pub gptm_pscr: GPTM_PSCR,
    #[doc = "0x88 - GPTM_CRR"]
    pub gptm_crr: GPTM_CRR,
    _reserved14: [u8; 4usize],
    #[doc = "0x90 - GPTM_CH0CCR"]
    pub gptm_ch0ccr: GPTM_CH0CCR,
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
#[doc = "GPTM_MDCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_mdcfr](gptm_mdcfr) module"]
pub type GPTM_MDCFR = crate::Reg<u32, _GPTM_MDCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTM_MDCFR;
#[doc = "`read()` method returns [gptm_mdcfr::R](gptm_mdcfr::R) reader structure"]
impl crate::Readable for GPTM_MDCFR {}
#[doc = "`write(|w| ..)` method takes [gptm_mdcfr::W](gptm_mdcfr::W) writer structure"]
impl crate::Writable for GPTM_MDCFR {}
#[doc = "GPTM_MDCFR"]
pub mod gptm_mdcfr;
#[doc = "GPTM_TRCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_trcfr](gptm_trcfr) module"]
pub type GPTM_TRCFR = crate::Reg<u32, _GPTM_TRCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTM_TRCFR;
#[doc = "`read()` method returns [gptm_trcfr::R](gptm_trcfr::R) reader structure"]
impl crate::Readable for GPTM_TRCFR {}
#[doc = "`write(|w| ..)` method takes [gptm_trcfr::W](gptm_trcfr::W) writer structure"]
impl crate::Writable for GPTM_TRCFR {}
#[doc = "GPTM_TRCFR"]
pub mod gptm_trcfr;
#[doc = "GPTM_CTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_ctr](gptm_ctr) module"]
pub type GPTM_CTR = crate::Reg<u32, _GPTM_CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTM_CTR;
#[doc = "`read()` method returns [gptm_ctr::R](gptm_ctr::R) reader structure"]
impl crate::Readable for GPTM_CTR {}
#[doc = "`write(|w| ..)` method takes [gptm_ctr::W](gptm_ctr::W) writer structure"]
impl crate::Writable for GPTM_CTR {}
#[doc = "GPTM_CTR"]
pub mod gptm_ctr;
#[doc = "GPTM_CH0ICFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_ch0icfr](gptm_ch0icfr) module"]
pub type GPTM_CH0ICFR = crate::Reg<u32, _GPTM_CH0ICFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTM_CH0ICFR;
#[doc = "`read()` method returns [gptm_ch0icfr::R](gptm_ch0icfr::R) reader structure"]
impl crate::Readable for GPTM_CH0ICFR {}
#[doc = "`write(|w| ..)` method takes [gptm_ch0icfr::W](gptm_ch0icfr::W) writer structure"]
impl crate::Writable for GPTM_CH0ICFR {}
#[doc = "GPTM_CH0ICFR"]
pub mod gptm_ch0icfr;
#[doc = "GPTM_CH1OCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_ch1ocfr](gptm_ch1ocfr) module"]
pub type GPTM_CH1OCFR = crate::Reg<u32, _GPTM_CH1OCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTM_CH1OCFR;
#[doc = "`read()` method returns [gptm_ch1ocfr::R](gptm_ch1ocfr::R) reader structure"]
impl crate::Readable for GPTM_CH1OCFR {}
#[doc = "`write(|w| ..)` method takes [gptm_ch1ocfr::W](gptm_ch1ocfr::W) writer structure"]
impl crate::Writable for GPTM_CH1OCFR {}
#[doc = "GPTM_CH1OCFR"]
pub mod gptm_ch1ocfr;
#[doc = "GPTM_CHCTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_chctr](gptm_chctr) module"]
pub type GPTM_CHCTR = crate::Reg<u32, _GPTM_CHCTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTM_CHCTR;
#[doc = "`read()` method returns [gptm_chctr::R](gptm_chctr::R) reader structure"]
impl crate::Readable for GPTM_CHCTR {}
#[doc = "`write(|w| ..)` method takes [gptm_chctr::W](gptm_chctr::W) writer structure"]
impl crate::Writable for GPTM_CHCTR {}
#[doc = "GPTM_CHCTR"]
pub mod gptm_chctr;
#[doc = "GPTM_CHPOLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_chpolr](gptm_chpolr) module"]
pub type GPTM_CHPOLR = crate::Reg<u32, _GPTM_CHPOLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTM_CHPOLR;
#[doc = "`read()` method returns [gptm_chpolr::R](gptm_chpolr::R) reader structure"]
impl crate::Readable for GPTM_CHPOLR {}
#[doc = "`write(|w| ..)` method takes [gptm_chpolr::W](gptm_chpolr::W) writer structure"]
impl crate::Writable for GPTM_CHPOLR {}
#[doc = "GPTM_CHPOLR"]
pub mod gptm_chpolr;
#[doc = "GPTM_DICTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_dictr](gptm_dictr) module"]
pub type GPTM_DICTR = crate::Reg<u32, _GPTM_DICTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTM_DICTR;
#[doc = "`read()` method returns [gptm_dictr::R](gptm_dictr::R) reader structure"]
impl crate::Readable for GPTM_DICTR {}
#[doc = "`write(|w| ..)` method takes [gptm_dictr::W](gptm_dictr::W) writer structure"]
impl crate::Writable for GPTM_DICTR {}
#[doc = "GPTM_DICTR"]
pub mod gptm_dictr;
#[doc = "GPTM_EVGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_evgr](gptm_evgr) module"]
pub type GPTM_EVGR = crate::Reg<u32, _GPTM_EVGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTM_EVGR;
#[doc = "`read()` method returns [gptm_evgr::R](gptm_evgr::R) reader structure"]
impl crate::Readable for GPTM_EVGR {}
#[doc = "`write(|w| ..)` method takes [gptm_evgr::W](gptm_evgr::W) writer structure"]
impl crate::Writable for GPTM_EVGR {}
#[doc = "GPTM_EVGR"]
pub mod gptm_evgr;
#[doc = "GPTM_INTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_intsr](gptm_intsr) module"]
pub type GPTM_INTSR = crate::Reg<u32, _GPTM_INTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTM_INTSR;
#[doc = "`read()` method returns [gptm_intsr::R](gptm_intsr::R) reader structure"]
impl crate::Readable for GPTM_INTSR {}
#[doc = "`write(|w| ..)` method takes [gptm_intsr::W](gptm_intsr::W) writer structure"]
impl crate::Writable for GPTM_INTSR {}
#[doc = "GPTM_INTSR"]
pub mod gptm_intsr;
#[doc = "GPTM_CNTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_cntr](gptm_cntr) module"]
pub type GPTM_CNTR = crate::Reg<u32, _GPTM_CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTM_CNTR;
#[doc = "`read()` method returns [gptm_cntr::R](gptm_cntr::R) reader structure"]
impl crate::Readable for GPTM_CNTR {}
#[doc = "`write(|w| ..)` method takes [gptm_cntr::W](gptm_cntr::W) writer structure"]
impl crate::Writable for GPTM_CNTR {}
#[doc = "GPTM_CNTR"]
pub mod gptm_cntr;
#[doc = "GPTM_PSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_pscr](gptm_pscr) module"]
pub type GPTM_PSCR = crate::Reg<u32, _GPTM_PSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTM_PSCR;
#[doc = "`read()` method returns [gptm_pscr::R](gptm_pscr::R) reader structure"]
impl crate::Readable for GPTM_PSCR {}
#[doc = "`write(|w| ..)` method takes [gptm_pscr::W](gptm_pscr::W) writer structure"]
impl crate::Writable for GPTM_PSCR {}
#[doc = "GPTM_PSCR"]
pub mod gptm_pscr;
#[doc = "GPTM_CRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_crr](gptm_crr) module"]
pub type GPTM_CRR = crate::Reg<u32, _GPTM_CRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTM_CRR;
#[doc = "`read()` method returns [gptm_crr::R](gptm_crr::R) reader structure"]
impl crate::Readable for GPTM_CRR {}
#[doc = "`write(|w| ..)` method takes [gptm_crr::W](gptm_crr::W) writer structure"]
impl crate::Writable for GPTM_CRR {}
#[doc = "GPTM_CRR"]
pub mod gptm_crr;
#[doc = "GPTM_CH0CCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_ch0ccr](gptm_ch0ccr) module"]
pub type GPTM_CH0CCR = crate::Reg<u32, _GPTM_CH0CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTM_CH0CCR;
#[doc = "`read()` method returns [gptm_ch0ccr::R](gptm_ch0ccr::R) reader structure"]
impl crate::Readable for GPTM_CH0CCR {}
#[doc = "`write(|w| ..)` method takes [gptm_ch0ccr::W](gptm_ch0ccr::W) writer structure"]
impl crate::Writable for GPTM_CH0CCR {}
#[doc = "GPTM_CH0CCR"]
pub mod gptm_ch0ccr;
