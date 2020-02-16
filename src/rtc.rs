#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CNT"]
    pub cnt: CNT,
    #[doc = "0x04 - CMP"]
    pub cmp: CMP,
    #[doc = "0x08 - CR"]
    pub cr: CR,
    #[doc = "0x0c - SR"]
    pub sr: SR,
    #[doc = "0x10 - IWEN"]
    pub iwen: IWEN,
}
#[doc = "CNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt](cnt) module"]
pub type CNT = crate::Reg<u32, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
#[doc = "`read()` method returns [cnt::R](cnt::R) reader structure"]
impl crate::Readable for CNT {}
#[doc = "`write(|w| ..)` method takes [cnt::W](cnt::W) writer structure"]
impl crate::Writable for CNT {}
#[doc = "CNT"]
pub mod cnt;
#[doc = "CMP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp](cmp) module"]
pub type CMP = crate::Reg<u32, _CMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP;
#[doc = "`read()` method returns [cmp::R](cmp::R) reader structure"]
impl crate::Readable for CMP {}
#[doc = "`write(|w| ..)` method takes [cmp::W](cmp::W) writer structure"]
impl crate::Writable for CMP {}
#[doc = "CMP"]
pub mod cmp;
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
#[doc = "IWEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iwen](iwen) module"]
pub type IWEN = crate::Reg<u32, _IWEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IWEN;
#[doc = "`read()` method returns [iwen::R](iwen::R) reader structure"]
impl crate::Readable for IWEN {}
#[doc = "`write(|w| ..)` method takes [iwen::W](iwen::W) writer structure"]
impl crate::Writable for IWEN {}
#[doc = "IWEN"]
pub mod iwen;
