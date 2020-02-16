#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART_URDR"]
    pub uart_urdr: UART_URDR,
    #[doc = "0x04 - UART_URCR"]
    pub uart_urcr: UART_URCR,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - UART_URIER"]
    pub uart_urier: UART_URIER,
    #[doc = "0x10 - UART_URSIFR"]
    pub uart_ursifr: UART_URSIFR,
    _reserved4: [u8; 16usize],
    #[doc = "0x24 - UART_URDLR"]
    pub uart_urdlr: UART_URDLR,
    #[doc = "0x28 - UART_URTSTR"]
    pub uart_urtstr: UART_URTSTR,
}
#[doc = "UART_URDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_urdr](uart_urdr) module"]
pub type UART_URDR = crate::Reg<u32, _UART_URDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_URDR;
#[doc = "`read()` method returns [uart_urdr::R](uart_urdr::R) reader structure"]
impl crate::Readable for UART_URDR {}
#[doc = "`write(|w| ..)` method takes [uart_urdr::W](uart_urdr::W) writer structure"]
impl crate::Writable for UART_URDR {}
#[doc = "UART_URDR"]
pub mod uart_urdr;
#[doc = "UART_URCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_urcr](uart_urcr) module"]
pub type UART_URCR = crate::Reg<u32, _UART_URCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_URCR;
#[doc = "`read()` method returns [uart_urcr::R](uart_urcr::R) reader structure"]
impl crate::Readable for UART_URCR {}
#[doc = "`write(|w| ..)` method takes [uart_urcr::W](uart_urcr::W) writer structure"]
impl crate::Writable for UART_URCR {}
#[doc = "UART_URCR"]
pub mod uart_urcr;
#[doc = "UART_URIER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_urier](uart_urier) module"]
pub type UART_URIER = crate::Reg<u32, _UART_URIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_URIER;
#[doc = "`read()` method returns [uart_urier::R](uart_urier::R) reader structure"]
impl crate::Readable for UART_URIER {}
#[doc = "`write(|w| ..)` method takes [uart_urier::W](uart_urier::W) writer structure"]
impl crate::Writable for UART_URIER {}
#[doc = "UART_URIER"]
pub mod uart_urier;
#[doc = "UART_URSIFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_ursifr](uart_ursifr) module"]
pub type UART_URSIFR = crate::Reg<u32, _UART_URSIFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_URSIFR;
#[doc = "`read()` method returns [uart_ursifr::R](uart_ursifr::R) reader structure"]
impl crate::Readable for UART_URSIFR {}
#[doc = "`write(|w| ..)` method takes [uart_ursifr::W](uart_ursifr::W) writer structure"]
impl crate::Writable for UART_URSIFR {}
#[doc = "UART_URSIFR"]
pub mod uart_ursifr;
#[doc = "UART_URDLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_urdlr](uart_urdlr) module"]
pub type UART_URDLR = crate::Reg<u32, _UART_URDLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_URDLR;
#[doc = "`read()` method returns [uart_urdlr::R](uart_urdlr::R) reader structure"]
impl crate::Readable for UART_URDLR {}
#[doc = "`write(|w| ..)` method takes [uart_urdlr::W](uart_urdlr::W) writer structure"]
impl crate::Writable for UART_URDLR {}
#[doc = "UART_URDLR"]
pub mod uart_urdlr;
#[doc = "UART_URTSTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_urtstr](uart_urtstr) module"]
pub type UART_URTSTR = crate::Reg<u32, _UART_URTSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_URTSTR;
#[doc = "`read()` method returns [uart_urtstr::R](uart_urtstr::R) reader structure"]
impl crate::Readable for UART_URTSTR {}
#[doc = "`write(|w| ..)` method takes [uart_urtstr::W](uart_urtstr::W) writer structure"]
impl crate::Writable for UART_URTSTR {}
#[doc = "UART_URTSTR"]
pub mod uart_urtstr;
