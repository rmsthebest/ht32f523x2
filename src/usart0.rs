#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USART_USRDR"]
    pub usart_usrdr: USART_USRDR,
    #[doc = "0x04 - USART_USRCR"]
    pub usart_usrcr: USART_USRCR,
    #[doc = "0x08 - USART_USRFCR"]
    pub usart_usrfcr: USART_USRFCR,
    #[doc = "0x0c - USART_USRIER"]
    pub usart_usrier: USART_USRIER,
    #[doc = "0x10 - USART_USRSIFR"]
    pub usart_usrsifr: USART_USRSIFR,
    #[doc = "0x14 - USART_USRTPR"]
    pub usart_usrtpr: USART_USRTPR,
    #[doc = "0x18 - USART_IrDACR"]
    pub usart_ir_dacr: USART_IRDACR,
    #[doc = "0x1c - USART_RS485CR"]
    pub usart_rs485cr: USART_RS485CR,
    #[doc = "0x20 - USART_SYNCR"]
    pub usart_syncr: USART_SYNCR,
    #[doc = "0x24 - USART_USRDLR"]
    pub usart_usrdlr: USART_USRDLR,
    #[doc = "0x28 - USART_USRTSTR"]
    pub usart_usrtstr: USART_USRTSTR,
}
#[doc = "USART_USRDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_usrdr](usart_usrdr) module"]
pub type USART_USRDR = crate::Reg<u32, _USART_USRDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_USRDR;
#[doc = "`read()` method returns [usart_usrdr::R](usart_usrdr::R) reader structure"]
impl crate::Readable for USART_USRDR {}
#[doc = "`write(|w| ..)` method takes [usart_usrdr::W](usart_usrdr::W) writer structure"]
impl crate::Writable for USART_USRDR {}
#[doc = "USART_USRDR"]
pub mod usart_usrdr;
#[doc = "USART_USRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_usrcr](usart_usrcr) module"]
pub type USART_USRCR = crate::Reg<u32, _USART_USRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_USRCR;
#[doc = "`read()` method returns [usart_usrcr::R](usart_usrcr::R) reader structure"]
impl crate::Readable for USART_USRCR {}
#[doc = "`write(|w| ..)` method takes [usart_usrcr::W](usart_usrcr::W) writer structure"]
impl crate::Writable for USART_USRCR {}
#[doc = "USART_USRCR"]
pub mod usart_usrcr;
#[doc = "USART_USRFCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_usrfcr](usart_usrfcr) module"]
pub type USART_USRFCR = crate::Reg<u32, _USART_USRFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_USRFCR;
#[doc = "`read()` method returns [usart_usrfcr::R](usart_usrfcr::R) reader structure"]
impl crate::Readable for USART_USRFCR {}
#[doc = "`write(|w| ..)` method takes [usart_usrfcr::W](usart_usrfcr::W) writer structure"]
impl crate::Writable for USART_USRFCR {}
#[doc = "USART_USRFCR"]
pub mod usart_usrfcr;
#[doc = "USART_USRIER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_usrier](usart_usrier) module"]
pub type USART_USRIER = crate::Reg<u32, _USART_USRIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_USRIER;
#[doc = "`read()` method returns [usart_usrier::R](usart_usrier::R) reader structure"]
impl crate::Readable for USART_USRIER {}
#[doc = "`write(|w| ..)` method takes [usart_usrier::W](usart_usrier::W) writer structure"]
impl crate::Writable for USART_USRIER {}
#[doc = "USART_USRIER"]
pub mod usart_usrier;
#[doc = "USART_USRSIFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_usrsifr](usart_usrsifr) module"]
pub type USART_USRSIFR = crate::Reg<u32, _USART_USRSIFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_USRSIFR;
#[doc = "`read()` method returns [usart_usrsifr::R](usart_usrsifr::R) reader structure"]
impl crate::Readable for USART_USRSIFR {}
#[doc = "`write(|w| ..)` method takes [usart_usrsifr::W](usart_usrsifr::W) writer structure"]
impl crate::Writable for USART_USRSIFR {}
#[doc = "USART_USRSIFR"]
pub mod usart_usrsifr;
#[doc = "USART_USRTPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_usrtpr](usart_usrtpr) module"]
pub type USART_USRTPR = crate::Reg<u32, _USART_USRTPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_USRTPR;
#[doc = "`read()` method returns [usart_usrtpr::R](usart_usrtpr::R) reader structure"]
impl crate::Readable for USART_USRTPR {}
#[doc = "`write(|w| ..)` method takes [usart_usrtpr::W](usart_usrtpr::W) writer structure"]
impl crate::Writable for USART_USRTPR {}
#[doc = "USART_USRTPR"]
pub mod usart_usrtpr;
#[doc = "USART_IrDACR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_ir_dacr](usart_ir_dacr) module"]
pub type USART_IRDACR = crate::Reg<u32, _USART_IRDACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_IRDACR;
#[doc = "`read()` method returns [usart_ir_dacr::R](usart_ir_dacr::R) reader structure"]
impl crate::Readable for USART_IRDACR {}
#[doc = "`write(|w| ..)` method takes [usart_ir_dacr::W](usart_ir_dacr::W) writer structure"]
impl crate::Writable for USART_IRDACR {}
#[doc = "USART_IrDACR"]
pub mod usart_ir_dacr;
#[doc = "USART_RS485CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_rs485cr](usart_rs485cr) module"]
pub type USART_RS485CR = crate::Reg<u32, _USART_RS485CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_RS485CR;
#[doc = "`read()` method returns [usart_rs485cr::R](usart_rs485cr::R) reader structure"]
impl crate::Readable for USART_RS485CR {}
#[doc = "`write(|w| ..)` method takes [usart_rs485cr::W](usart_rs485cr::W) writer structure"]
impl crate::Writable for USART_RS485CR {}
#[doc = "USART_RS485CR"]
pub mod usart_rs485cr;
#[doc = "USART_SYNCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_syncr](usart_syncr) module"]
pub type USART_SYNCR = crate::Reg<u32, _USART_SYNCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_SYNCR;
#[doc = "`read()` method returns [usart_syncr::R](usart_syncr::R) reader structure"]
impl crate::Readable for USART_SYNCR {}
#[doc = "`write(|w| ..)` method takes [usart_syncr::W](usart_syncr::W) writer structure"]
impl crate::Writable for USART_SYNCR {}
#[doc = "USART_SYNCR"]
pub mod usart_syncr;
#[doc = "USART_USRDLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_usrdlr](usart_usrdlr) module"]
pub type USART_USRDLR = crate::Reg<u32, _USART_USRDLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_USRDLR;
#[doc = "`read()` method returns [usart_usrdlr::R](usart_usrdlr::R) reader structure"]
impl crate::Readable for USART_USRDLR {}
#[doc = "`write(|w| ..)` method takes [usart_usrdlr::W](usart_usrdlr::W) writer structure"]
impl crate::Writable for USART_USRDLR {}
#[doc = "USART_USRDLR"]
pub mod usart_usrdlr;
#[doc = "USART_USRTSTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_usrtstr](usart_usrtstr) module"]
pub type USART_USRTSTR = crate::Reg<u32, _USART_USRTSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_USRTSTR;
#[doc = "`read()` method returns [usart_usrtstr::R](usart_usrtstr::R) reader structure"]
impl crate::Readable for USART_USRTSTR {}
#[doc = "`write(|w| ..)` method takes [usart_usrtstr::W](usart_usrtstr::W) writer structure"]
impl crate::Writable for USART_USRTSTR {}
#[doc = "USART_USRTSTR"]
pub mod usart_usrtstr;
