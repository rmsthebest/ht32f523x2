#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CSR"]
    pub csr: CSR,
    #[doc = "0x04 - IER"]
    pub ier: IER,
    #[doc = "0x08 - ISR"]
    pub isr: ISR,
    #[doc = "0x0c - FCR"]
    pub fcr: FCR,
    #[doc = "0x10 - DEVAR"]
    pub devar: DEVAR,
    #[doc = "0x14 - EP0CSR"]
    pub ep0csr: EP0CSR,
    #[doc = "0x18 - EP0IER"]
    pub ep0ier: EP0IER,
    #[doc = "0x1c - EP0ISR"]
    pub ep0isr: EP0ISR,
    #[doc = "0x20 - EP0TCR"]
    pub ep0tcr: EP0TCR,
    #[doc = "0x24 - EP0CFGR"]
    pub ep0cfgr: EP0CFGR,
    #[doc = "0x28 - EP1CSR"]
    pub ep1csr: EP1CSR,
    #[doc = "0x2c - EP1IER"]
    pub ep1ier: EP1IER,
    #[doc = "0x30 - EP1ISR"]
    pub ep1isr: EP1ISR,
    #[doc = "0x34 - EP1TCR"]
    pub ep1tcr: EP1TCR,
    #[doc = "0x38 - EP1CFGR"]
    pub ep1cfgr: EP1CFGR,
    #[doc = "0x3c - EP2CSR"]
    pub ep2csr: EP2CSR,
    #[doc = "0x40 - EP2IER"]
    pub ep2ier: EP2IER,
    #[doc = "0x44 - EP2ISR"]
    pub ep2isr: EP2ISR,
    #[doc = "0x48 - EP2TCR"]
    pub ep2tcr: EP2TCR,
    #[doc = "0x4c - EP2CFGR"]
    pub ep2cfgr: EP2CFGR,
    #[doc = "0x50 - EP3CSR"]
    pub ep3csr: EP3CSR,
    #[doc = "0x54 - EP3IER"]
    pub ep3ier: EP3IER,
    #[doc = "0x58 - EP3ISR"]
    pub ep3isr: EP3ISR,
    #[doc = "0x5c - EP3TCR"]
    pub ep3tcr: EP3TCR,
    #[doc = "0x60 - EP3CFGR"]
    pub ep3cfgr: EP3CFGR,
    #[doc = "0x64 - EP4CSR"]
    pub ep4csr: EP4CSR,
    #[doc = "0x68 - EP4IER"]
    pub ep4ier: EP4IER,
    #[doc = "0x6c - EP4ISR"]
    pub ep4isr: EP4ISR,
    #[doc = "0x70 - EP4TCR"]
    pub ep4tcr: EP4TCR,
    #[doc = "0x74 - EP4CFGR"]
    pub ep4cfgr: EP4CFGR,
    #[doc = "0x78 - EP5CSR"]
    pub ep5csr: EP5CSR,
    #[doc = "0x7c - EP5IER"]
    pub ep5ier: EP5IER,
    #[doc = "0x80 - EP5ISR"]
    pub ep5isr: EP5ISR,
    #[doc = "0x84 - EP5TCR"]
    pub ep5tcr: EP5TCR,
    #[doc = "0x88 - EP5CFGR"]
    pub ep5cfgr: EP5CFGR,
    #[doc = "0x8c - EP6CSR"]
    pub ep6csr: EP6CSR,
    #[doc = "0x90 - EP6IER"]
    pub ep6ier: EP6IER,
    #[doc = "0x94 - EP6ISR"]
    pub ep6isr: EP6ISR,
    #[doc = "0x98 - EP6TCR"]
    pub ep6tcr: EP6TCR,
    #[doc = "0x9c - EP6CFGR"]
    pub ep6cfgr: EP6CFGR,
    #[doc = "0xa0 - EP7CSR"]
    pub ep7csr: EP7CSR,
    #[doc = "0xa4 - EP7IER"]
    pub ep7ier: EP7IER,
    #[doc = "0xa8 - EP7ISR"]
    pub ep7isr: EP7ISR,
    #[doc = "0xac - EP7TCR"]
    pub ep7tcr: EP7TCR,
    #[doc = "0xb0 - EP7CFGR"]
    pub ep7cfgr: EP7CFGR,
}
#[doc = "CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "`write(|w| ..)` method takes [csr::W](csr::W) writer structure"]
impl crate::Writable for CSR {}
#[doc = "CSR"]
pub mod csr;
#[doc = "IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "IER"]
pub mod ier;
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
#[doc = "FCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](fcr) module"]
pub type FCR = crate::Reg<u32, _FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCR;
#[doc = "`read()` method returns [fcr::R](fcr::R) reader structure"]
impl crate::Readable for FCR {}
#[doc = "`write(|w| ..)` method takes [fcr::W](fcr::W) writer structure"]
impl crate::Writable for FCR {}
#[doc = "FCR"]
pub mod fcr;
#[doc = "DEVAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devar](devar) module"]
pub type DEVAR = crate::Reg<u32, _DEVAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVAR;
#[doc = "`read()` method returns [devar::R](devar::R) reader structure"]
impl crate::Readable for DEVAR {}
#[doc = "`write(|w| ..)` method takes [devar::W](devar::W) writer structure"]
impl crate::Writable for DEVAR {}
#[doc = "DEVAR"]
pub mod devar;
#[doc = "EP0CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep0csr](ep0csr) module"]
pub type EP0CSR = crate::Reg<u32, _EP0CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP0CSR;
#[doc = "`read()` method returns [ep0csr::R](ep0csr::R) reader structure"]
impl crate::Readable for EP0CSR {}
#[doc = "`write(|w| ..)` method takes [ep0csr::W](ep0csr::W) writer structure"]
impl crate::Writable for EP0CSR {}
#[doc = "EP0CSR"]
pub mod ep0csr;
#[doc = "EP0IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep0ier](ep0ier) module"]
pub type EP0IER = crate::Reg<u32, _EP0IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP0IER;
#[doc = "`read()` method returns [ep0ier::R](ep0ier::R) reader structure"]
impl crate::Readable for EP0IER {}
#[doc = "`write(|w| ..)` method takes [ep0ier::W](ep0ier::W) writer structure"]
impl crate::Writable for EP0IER {}
#[doc = "EP0IER"]
pub mod ep0ier;
#[doc = "EP0ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep0isr](ep0isr) module"]
pub type EP0ISR = crate::Reg<u32, _EP0ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP0ISR;
#[doc = "`read()` method returns [ep0isr::R](ep0isr::R) reader structure"]
impl crate::Readable for EP0ISR {}
#[doc = "`write(|w| ..)` method takes [ep0isr::W](ep0isr::W) writer structure"]
impl crate::Writable for EP0ISR {}
#[doc = "EP0ISR"]
pub mod ep0isr;
#[doc = "EP0TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep0tcr](ep0tcr) module"]
pub type EP0TCR = crate::Reg<u32, _EP0TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP0TCR;
#[doc = "`read()` method returns [ep0tcr::R](ep0tcr::R) reader structure"]
impl crate::Readable for EP0TCR {}
#[doc = "`write(|w| ..)` method takes [ep0tcr::W](ep0tcr::W) writer structure"]
impl crate::Writable for EP0TCR {}
#[doc = "EP0TCR"]
pub mod ep0tcr;
#[doc = "EP0CFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep0cfgr](ep0cfgr) module"]
pub type EP0CFGR = crate::Reg<u32, _EP0CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP0CFGR;
#[doc = "`read()` method returns [ep0cfgr::R](ep0cfgr::R) reader structure"]
impl crate::Readable for EP0CFGR {}
#[doc = "`write(|w| ..)` method takes [ep0cfgr::W](ep0cfgr::W) writer structure"]
impl crate::Writable for EP0CFGR {}
#[doc = "EP0CFGR"]
pub mod ep0cfgr;
#[doc = "EP1CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep1csr](ep1csr) module"]
pub type EP1CSR = crate::Reg<u32, _EP1CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP1CSR;
#[doc = "`read()` method returns [ep1csr::R](ep1csr::R) reader structure"]
impl crate::Readable for EP1CSR {}
#[doc = "`write(|w| ..)` method takes [ep1csr::W](ep1csr::W) writer structure"]
impl crate::Writable for EP1CSR {}
#[doc = "EP1CSR"]
pub mod ep1csr;
#[doc = "EP1IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep1ier](ep1ier) module"]
pub type EP1IER = crate::Reg<u32, _EP1IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP1IER;
#[doc = "`read()` method returns [ep1ier::R](ep1ier::R) reader structure"]
impl crate::Readable for EP1IER {}
#[doc = "`write(|w| ..)` method takes [ep1ier::W](ep1ier::W) writer structure"]
impl crate::Writable for EP1IER {}
#[doc = "EP1IER"]
pub mod ep1ier;
#[doc = "EP1ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep1isr](ep1isr) module"]
pub type EP1ISR = crate::Reg<u32, _EP1ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP1ISR;
#[doc = "`read()` method returns [ep1isr::R](ep1isr::R) reader structure"]
impl crate::Readable for EP1ISR {}
#[doc = "`write(|w| ..)` method takes [ep1isr::W](ep1isr::W) writer structure"]
impl crate::Writable for EP1ISR {}
#[doc = "EP1ISR"]
pub mod ep1isr;
#[doc = "EP1TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep1tcr](ep1tcr) module"]
pub type EP1TCR = crate::Reg<u32, _EP1TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP1TCR;
#[doc = "`read()` method returns [ep1tcr::R](ep1tcr::R) reader structure"]
impl crate::Readable for EP1TCR {}
#[doc = "`write(|w| ..)` method takes [ep1tcr::W](ep1tcr::W) writer structure"]
impl crate::Writable for EP1TCR {}
#[doc = "EP1TCR"]
pub mod ep1tcr;
#[doc = "EP1CFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep1cfgr](ep1cfgr) module"]
pub type EP1CFGR = crate::Reg<u32, _EP1CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP1CFGR;
#[doc = "`read()` method returns [ep1cfgr::R](ep1cfgr::R) reader structure"]
impl crate::Readable for EP1CFGR {}
#[doc = "`write(|w| ..)` method takes [ep1cfgr::W](ep1cfgr::W) writer structure"]
impl crate::Writable for EP1CFGR {}
#[doc = "EP1CFGR"]
pub mod ep1cfgr;
#[doc = "EP2CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep2csr](ep2csr) module"]
pub type EP2CSR = crate::Reg<u32, _EP2CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP2CSR;
#[doc = "`read()` method returns [ep2csr::R](ep2csr::R) reader structure"]
impl crate::Readable for EP2CSR {}
#[doc = "`write(|w| ..)` method takes [ep2csr::W](ep2csr::W) writer structure"]
impl crate::Writable for EP2CSR {}
#[doc = "EP2CSR"]
pub mod ep2csr;
#[doc = "EP2IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep2ier](ep2ier) module"]
pub type EP2IER = crate::Reg<u32, _EP2IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP2IER;
#[doc = "`read()` method returns [ep2ier::R](ep2ier::R) reader structure"]
impl crate::Readable for EP2IER {}
#[doc = "`write(|w| ..)` method takes [ep2ier::W](ep2ier::W) writer structure"]
impl crate::Writable for EP2IER {}
#[doc = "EP2IER"]
pub mod ep2ier;
#[doc = "EP2ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep2isr](ep2isr) module"]
pub type EP2ISR = crate::Reg<u32, _EP2ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP2ISR;
#[doc = "`read()` method returns [ep2isr::R](ep2isr::R) reader structure"]
impl crate::Readable for EP2ISR {}
#[doc = "`write(|w| ..)` method takes [ep2isr::W](ep2isr::W) writer structure"]
impl crate::Writable for EP2ISR {}
#[doc = "EP2ISR"]
pub mod ep2isr;
#[doc = "EP2TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep2tcr](ep2tcr) module"]
pub type EP2TCR = crate::Reg<u32, _EP2TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP2TCR;
#[doc = "`read()` method returns [ep2tcr::R](ep2tcr::R) reader structure"]
impl crate::Readable for EP2TCR {}
#[doc = "`write(|w| ..)` method takes [ep2tcr::W](ep2tcr::W) writer structure"]
impl crate::Writable for EP2TCR {}
#[doc = "EP2TCR"]
pub mod ep2tcr;
#[doc = "EP2CFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep2cfgr](ep2cfgr) module"]
pub type EP2CFGR = crate::Reg<u32, _EP2CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP2CFGR;
#[doc = "`read()` method returns [ep2cfgr::R](ep2cfgr::R) reader structure"]
impl crate::Readable for EP2CFGR {}
#[doc = "`write(|w| ..)` method takes [ep2cfgr::W](ep2cfgr::W) writer structure"]
impl crate::Writable for EP2CFGR {}
#[doc = "EP2CFGR"]
pub mod ep2cfgr;
#[doc = "EP3CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep3csr](ep3csr) module"]
pub type EP3CSR = crate::Reg<u32, _EP3CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP3CSR;
#[doc = "`read()` method returns [ep3csr::R](ep3csr::R) reader structure"]
impl crate::Readable for EP3CSR {}
#[doc = "`write(|w| ..)` method takes [ep3csr::W](ep3csr::W) writer structure"]
impl crate::Writable for EP3CSR {}
#[doc = "EP3CSR"]
pub mod ep3csr;
#[doc = "EP3IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep3ier](ep3ier) module"]
pub type EP3IER = crate::Reg<u32, _EP3IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP3IER;
#[doc = "`read()` method returns [ep3ier::R](ep3ier::R) reader structure"]
impl crate::Readable for EP3IER {}
#[doc = "`write(|w| ..)` method takes [ep3ier::W](ep3ier::W) writer structure"]
impl crate::Writable for EP3IER {}
#[doc = "EP3IER"]
pub mod ep3ier;
#[doc = "EP3ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep3isr](ep3isr) module"]
pub type EP3ISR = crate::Reg<u32, _EP3ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP3ISR;
#[doc = "`read()` method returns [ep3isr::R](ep3isr::R) reader structure"]
impl crate::Readable for EP3ISR {}
#[doc = "`write(|w| ..)` method takes [ep3isr::W](ep3isr::W) writer structure"]
impl crate::Writable for EP3ISR {}
#[doc = "EP3ISR"]
pub mod ep3isr;
#[doc = "EP3TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep3tcr](ep3tcr) module"]
pub type EP3TCR = crate::Reg<u32, _EP3TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP3TCR;
#[doc = "`read()` method returns [ep3tcr::R](ep3tcr::R) reader structure"]
impl crate::Readable for EP3TCR {}
#[doc = "`write(|w| ..)` method takes [ep3tcr::W](ep3tcr::W) writer structure"]
impl crate::Writable for EP3TCR {}
#[doc = "EP3TCR"]
pub mod ep3tcr;
#[doc = "EP3CFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep3cfgr](ep3cfgr) module"]
pub type EP3CFGR = crate::Reg<u32, _EP3CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP3CFGR;
#[doc = "`read()` method returns [ep3cfgr::R](ep3cfgr::R) reader structure"]
impl crate::Readable for EP3CFGR {}
#[doc = "`write(|w| ..)` method takes [ep3cfgr::W](ep3cfgr::W) writer structure"]
impl crate::Writable for EP3CFGR {}
#[doc = "EP3CFGR"]
pub mod ep3cfgr;
#[doc = "EP4CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep4csr](ep4csr) module"]
pub type EP4CSR = crate::Reg<u32, _EP4CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP4CSR;
#[doc = "`read()` method returns [ep4csr::R](ep4csr::R) reader structure"]
impl crate::Readable for EP4CSR {}
#[doc = "`write(|w| ..)` method takes [ep4csr::W](ep4csr::W) writer structure"]
impl crate::Writable for EP4CSR {}
#[doc = "EP4CSR"]
pub mod ep4csr;
#[doc = "EP4IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep4ier](ep4ier) module"]
pub type EP4IER = crate::Reg<u32, _EP4IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP4IER;
#[doc = "`read()` method returns [ep4ier::R](ep4ier::R) reader structure"]
impl crate::Readable for EP4IER {}
#[doc = "`write(|w| ..)` method takes [ep4ier::W](ep4ier::W) writer structure"]
impl crate::Writable for EP4IER {}
#[doc = "EP4IER"]
pub mod ep4ier;
#[doc = "EP4ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep4isr](ep4isr) module"]
pub type EP4ISR = crate::Reg<u32, _EP4ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP4ISR;
#[doc = "`read()` method returns [ep4isr::R](ep4isr::R) reader structure"]
impl crate::Readable for EP4ISR {}
#[doc = "`write(|w| ..)` method takes [ep4isr::W](ep4isr::W) writer structure"]
impl crate::Writable for EP4ISR {}
#[doc = "EP4ISR"]
pub mod ep4isr;
#[doc = "EP4TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep4tcr](ep4tcr) module"]
pub type EP4TCR = crate::Reg<u32, _EP4TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP4TCR;
#[doc = "`read()` method returns [ep4tcr::R](ep4tcr::R) reader structure"]
impl crate::Readable for EP4TCR {}
#[doc = "`write(|w| ..)` method takes [ep4tcr::W](ep4tcr::W) writer structure"]
impl crate::Writable for EP4TCR {}
#[doc = "EP4TCR"]
pub mod ep4tcr;
#[doc = "EP4CFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep4cfgr](ep4cfgr) module"]
pub type EP4CFGR = crate::Reg<u32, _EP4CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP4CFGR;
#[doc = "`read()` method returns [ep4cfgr::R](ep4cfgr::R) reader structure"]
impl crate::Readable for EP4CFGR {}
#[doc = "`write(|w| ..)` method takes [ep4cfgr::W](ep4cfgr::W) writer structure"]
impl crate::Writable for EP4CFGR {}
#[doc = "EP4CFGR"]
pub mod ep4cfgr;
#[doc = "EP5CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep5csr](ep5csr) module"]
pub type EP5CSR = crate::Reg<u32, _EP5CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP5CSR;
#[doc = "`read()` method returns [ep5csr::R](ep5csr::R) reader structure"]
impl crate::Readable for EP5CSR {}
#[doc = "`write(|w| ..)` method takes [ep5csr::W](ep5csr::W) writer structure"]
impl crate::Writable for EP5CSR {}
#[doc = "EP5CSR"]
pub mod ep5csr;
#[doc = "EP5IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep5ier](ep5ier) module"]
pub type EP5IER = crate::Reg<u32, _EP5IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP5IER;
#[doc = "`read()` method returns [ep5ier::R](ep5ier::R) reader structure"]
impl crate::Readable for EP5IER {}
#[doc = "`write(|w| ..)` method takes [ep5ier::W](ep5ier::W) writer structure"]
impl crate::Writable for EP5IER {}
#[doc = "EP5IER"]
pub mod ep5ier;
#[doc = "EP5ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep5isr](ep5isr) module"]
pub type EP5ISR = crate::Reg<u32, _EP5ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP5ISR;
#[doc = "`read()` method returns [ep5isr::R](ep5isr::R) reader structure"]
impl crate::Readable for EP5ISR {}
#[doc = "`write(|w| ..)` method takes [ep5isr::W](ep5isr::W) writer structure"]
impl crate::Writable for EP5ISR {}
#[doc = "EP5ISR"]
pub mod ep5isr;
#[doc = "EP5TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep5tcr](ep5tcr) module"]
pub type EP5TCR = crate::Reg<u32, _EP5TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP5TCR;
#[doc = "`read()` method returns [ep5tcr::R](ep5tcr::R) reader structure"]
impl crate::Readable for EP5TCR {}
#[doc = "`write(|w| ..)` method takes [ep5tcr::W](ep5tcr::W) writer structure"]
impl crate::Writable for EP5TCR {}
#[doc = "EP5TCR"]
pub mod ep5tcr;
#[doc = "EP5CFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep5cfgr](ep5cfgr) module"]
pub type EP5CFGR = crate::Reg<u32, _EP5CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP5CFGR;
#[doc = "`read()` method returns [ep5cfgr::R](ep5cfgr::R) reader structure"]
impl crate::Readable for EP5CFGR {}
#[doc = "`write(|w| ..)` method takes [ep5cfgr::W](ep5cfgr::W) writer structure"]
impl crate::Writable for EP5CFGR {}
#[doc = "EP5CFGR"]
pub mod ep5cfgr;
#[doc = "EP6CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep6csr](ep6csr) module"]
pub type EP6CSR = crate::Reg<u32, _EP6CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP6CSR;
#[doc = "`read()` method returns [ep6csr::R](ep6csr::R) reader structure"]
impl crate::Readable for EP6CSR {}
#[doc = "`write(|w| ..)` method takes [ep6csr::W](ep6csr::W) writer structure"]
impl crate::Writable for EP6CSR {}
#[doc = "EP6CSR"]
pub mod ep6csr;
#[doc = "EP6IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep6ier](ep6ier) module"]
pub type EP6IER = crate::Reg<u32, _EP6IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP6IER;
#[doc = "`read()` method returns [ep6ier::R](ep6ier::R) reader structure"]
impl crate::Readable for EP6IER {}
#[doc = "`write(|w| ..)` method takes [ep6ier::W](ep6ier::W) writer structure"]
impl crate::Writable for EP6IER {}
#[doc = "EP6IER"]
pub mod ep6ier;
#[doc = "EP6ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep6isr](ep6isr) module"]
pub type EP6ISR = crate::Reg<u32, _EP6ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP6ISR;
#[doc = "`read()` method returns [ep6isr::R](ep6isr::R) reader structure"]
impl crate::Readable for EP6ISR {}
#[doc = "`write(|w| ..)` method takes [ep6isr::W](ep6isr::W) writer structure"]
impl crate::Writable for EP6ISR {}
#[doc = "EP6ISR"]
pub mod ep6isr;
#[doc = "EP6TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep6tcr](ep6tcr) module"]
pub type EP6TCR = crate::Reg<u32, _EP6TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP6TCR;
#[doc = "`read()` method returns [ep6tcr::R](ep6tcr::R) reader structure"]
impl crate::Readable for EP6TCR {}
#[doc = "`write(|w| ..)` method takes [ep6tcr::W](ep6tcr::W) writer structure"]
impl crate::Writable for EP6TCR {}
#[doc = "EP6TCR"]
pub mod ep6tcr;
#[doc = "EP6CFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep6cfgr](ep6cfgr) module"]
pub type EP6CFGR = crate::Reg<u32, _EP6CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP6CFGR;
#[doc = "`read()` method returns [ep6cfgr::R](ep6cfgr::R) reader structure"]
impl crate::Readable for EP6CFGR {}
#[doc = "`write(|w| ..)` method takes [ep6cfgr::W](ep6cfgr::W) writer structure"]
impl crate::Writable for EP6CFGR {}
#[doc = "EP6CFGR"]
pub mod ep6cfgr;
#[doc = "EP7CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep7csr](ep7csr) module"]
pub type EP7CSR = crate::Reg<u32, _EP7CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP7CSR;
#[doc = "`read()` method returns [ep7csr::R](ep7csr::R) reader structure"]
impl crate::Readable for EP7CSR {}
#[doc = "`write(|w| ..)` method takes [ep7csr::W](ep7csr::W) writer structure"]
impl crate::Writable for EP7CSR {}
#[doc = "EP7CSR"]
pub mod ep7csr;
#[doc = "EP7IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep7ier](ep7ier) module"]
pub type EP7IER = crate::Reg<u32, _EP7IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP7IER;
#[doc = "`read()` method returns [ep7ier::R](ep7ier::R) reader structure"]
impl crate::Readable for EP7IER {}
#[doc = "`write(|w| ..)` method takes [ep7ier::W](ep7ier::W) writer structure"]
impl crate::Writable for EP7IER {}
#[doc = "EP7IER"]
pub mod ep7ier;
#[doc = "EP7ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep7isr](ep7isr) module"]
pub type EP7ISR = crate::Reg<u32, _EP7ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP7ISR;
#[doc = "`read()` method returns [ep7isr::R](ep7isr::R) reader structure"]
impl crate::Readable for EP7ISR {}
#[doc = "`write(|w| ..)` method takes [ep7isr::W](ep7isr::W) writer structure"]
impl crate::Writable for EP7ISR {}
#[doc = "EP7ISR"]
pub mod ep7isr;
#[doc = "EP7TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep7tcr](ep7tcr) module"]
pub type EP7TCR = crate::Reg<u32, _EP7TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP7TCR;
#[doc = "`read()` method returns [ep7tcr::R](ep7tcr::R) reader structure"]
impl crate::Readable for EP7TCR {}
#[doc = "`write(|w| ..)` method takes [ep7tcr::W](ep7tcr::W) writer structure"]
impl crate::Writable for EP7TCR {}
#[doc = "EP7TCR"]
pub mod ep7tcr;
#[doc = "EP7CFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep7cfgr](ep7cfgr) module"]
pub type EP7CFGR = crate::Reg<u32, _EP7CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP7CFGR;
#[doc = "`read()` method returns [ep7cfgr::R](ep7cfgr::R) reader structure"]
impl crate::Readable for EP7CFGR {}
#[doc = "`write(|w| ..)` method takes [ep7cfgr::W](ep7cfgr::W) writer structure"]
impl crate::Writable for EP7CFGR {}
#[doc = "EP7CFGR"]
pub mod ep7cfgr;
