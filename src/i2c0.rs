#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR"]
    pub cr: CR,
    #[doc = "0x04 - IER"]
    pub ier: IER,
    #[doc = "0x08 - ADDR"]
    pub addr: ADDR,
    #[doc = "0x0c - SR"]
    pub sr: SR,
    #[doc = "0x10 - SHPGR"]
    pub shpgr: SHPGR,
    #[doc = "0x14 - SLPGR"]
    pub slpgr: SLPGR,
    #[doc = "0x18 - DR"]
    pub dr: DR,
    #[doc = "0x1c - TAR"]
    pub tar: TAR,
    #[doc = "0x20 - ADDMR"]
    pub addmr: ADDMR,
    #[doc = "0x24 - ADDSR"]
    pub addsr: ADDSR,
    #[doc = "0x28 - TOUT"]
    pub tout: TOUT,
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
#[doc = "ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](addr) module"]
pub type ADDR = crate::Reg<u32, _ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR;
#[doc = "`read()` method returns [addr::R](addr::R) reader structure"]
impl crate::Readable for ADDR {}
#[doc = "`write(|w| ..)` method takes [addr::W](addr::W) writer structure"]
impl crate::Writable for ADDR {}
#[doc = "ADDR"]
pub mod addr;
#[doc = "SR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "SR"]
pub mod sr;
#[doc = "SHPGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shpgr](shpgr) module"]
pub type SHPGR = crate::Reg<u32, _SHPGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHPGR;
#[doc = "`read()` method returns [shpgr::R](shpgr::R) reader structure"]
impl crate::Readable for SHPGR {}
#[doc = "`write(|w| ..)` method takes [shpgr::W](shpgr::W) writer structure"]
impl crate::Writable for SHPGR {}
#[doc = "SHPGR"]
pub mod shpgr;
#[doc = "SLPGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slpgr](slpgr) module"]
pub type SLPGR = crate::Reg<u32, _SLPGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLPGR;
#[doc = "`read()` method returns [slpgr::R](slpgr::R) reader structure"]
impl crate::Readable for SLPGR {}
#[doc = "`write(|w| ..)` method takes [slpgr::W](slpgr::W) writer structure"]
impl crate::Writable for SLPGR {}
#[doc = "SLPGR"]
pub mod slpgr;
#[doc = "DR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "`write(|w| ..)` method takes [dr::W](dr::W) writer structure"]
impl crate::Writable for DR {}
#[doc = "DR"]
pub mod dr;
#[doc = "TAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tar](tar) module"]
pub type TAR = crate::Reg<u32, _TAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAR;
#[doc = "`read()` method returns [tar::R](tar::R) reader structure"]
impl crate::Readable for TAR {}
#[doc = "`write(|w| ..)` method takes [tar::W](tar::W) writer structure"]
impl crate::Writable for TAR {}
#[doc = "TAR"]
pub mod tar;
#[doc = "ADDMR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addmr](addmr) module"]
pub type ADDMR = crate::Reg<u32, _ADDMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDMR;
#[doc = "`read()` method returns [addmr::R](addmr::R) reader structure"]
impl crate::Readable for ADDMR {}
#[doc = "`write(|w| ..)` method takes [addmr::W](addmr::W) writer structure"]
impl crate::Writable for ADDMR {}
#[doc = "ADDMR"]
pub mod addmr;
#[doc = "ADDSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addsr](addsr) module"]
pub type ADDSR = crate::Reg<u32, _ADDSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDSR;
#[doc = "`read()` method returns [addsr::R](addsr::R) reader structure"]
impl crate::Readable for ADDSR {}
#[doc = "`write(|w| ..)` method takes [addsr::W](addsr::W) writer structure"]
impl crate::Writable for ADDSR {}
#[doc = "ADDSR"]
pub mod addsr;
#[doc = "TOUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tout](tout) module"]
pub type TOUT = crate::Reg<u32, _TOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOUT;
#[doc = "`read()` method returns [tout::R](tout::R) reader structure"]
impl crate::Readable for TOUT {}
#[doc = "`write(|w| ..)` method takes [tout::W](tout::W) writer structure"]
impl crate::Writable for TOUT {}
#[doc = "TOUT"]
pub mod tout;
