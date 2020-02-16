#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TADR"]
    pub tadr: TADR,
    #[doc = "0x04 - WRDR"]
    pub wrdr: WRDR,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - OCMR"]
    pub ocmr: OCMR,
    #[doc = "0x10 - OPCR"]
    pub opcr: OPCR,
    #[doc = "0x14 - OIER"]
    pub oier: OIER,
    #[doc = "0x18 - OISR"]
    pub oisr: OISR,
    _reserved6: [u8; 4usize],
    #[doc = "0x20 - PPSR0"]
    pub ppsr0: PPSR0,
    #[doc = "0x24 - PPSR1"]
    pub ppsr1: PPSR1,
    #[doc = "0x28 - PPSR2"]
    pub ppsr2: PPSR2,
    #[doc = "0x2c - PPSR3"]
    pub ppsr3: PPSR3,
    #[doc = "0x30 - CPSR"]
    pub cpsr: CPSR,
    _reserved11: [u8; 204usize],
    #[doc = "0x100 - VMCR"]
    pub vmcr: VMCR,
    _reserved12: [u8; 124usize],
    #[doc = "0x180 - MDID"]
    pub mdid: MDID,
    #[doc = "0x184 - PNSR"]
    pub pnsr: PNSR,
    #[doc = "0x188 - PSSR"]
    pub pssr: PSSR,
    _reserved15: [u8; 116usize],
    #[doc = "0x200 - CFCR"]
    pub cfcr: CFCR,
    _reserved16: [u8; 268usize],
    #[doc = "0x310 - CID0"]
    pub cid0: CID0,
    #[doc = "0x314 - CID1"]
    pub cid1: CID1,
    #[doc = "0x318 - CID2"]
    pub cid2: CID2,
    #[doc = "0x31c - CID3"]
    pub cid3: CID3,
}
#[doc = "TADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tadr](tadr) module"]
pub type TADR = crate::Reg<u32, _TADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TADR;
#[doc = "`read()` method returns [tadr::R](tadr::R) reader structure"]
impl crate::Readable for TADR {}
#[doc = "`write(|w| ..)` method takes [tadr::W](tadr::W) writer structure"]
impl crate::Writable for TADR {}
#[doc = "TADR"]
pub mod tadr;
#[doc = "WRDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrdr](wrdr) module"]
pub type WRDR = crate::Reg<u32, _WRDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRDR;
#[doc = "`read()` method returns [wrdr::R](wrdr::R) reader structure"]
impl crate::Readable for WRDR {}
#[doc = "`write(|w| ..)` method takes [wrdr::W](wrdr::W) writer structure"]
impl crate::Writable for WRDR {}
#[doc = "WRDR"]
pub mod wrdr;
#[doc = "OCMR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocmr](ocmr) module"]
pub type OCMR = crate::Reg<u32, _OCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCMR;
#[doc = "`read()` method returns [ocmr::R](ocmr::R) reader structure"]
impl crate::Readable for OCMR {}
#[doc = "`write(|w| ..)` method takes [ocmr::W](ocmr::W) writer structure"]
impl crate::Writable for OCMR {}
#[doc = "OCMR"]
pub mod ocmr;
#[doc = "OPCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opcr](opcr) module"]
pub type OPCR = crate::Reg<u32, _OPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPCR;
#[doc = "`read()` method returns [opcr::R](opcr::R) reader structure"]
impl crate::Readable for OPCR {}
#[doc = "`write(|w| ..)` method takes [opcr::W](opcr::W) writer structure"]
impl crate::Writable for OPCR {}
#[doc = "OPCR"]
pub mod opcr;
#[doc = "OIER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oier](oier) module"]
pub type OIER = crate::Reg<u32, _OIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OIER;
#[doc = "`read()` method returns [oier::R](oier::R) reader structure"]
impl crate::Readable for OIER {}
#[doc = "`write(|w| ..)` method takes [oier::W](oier::W) writer structure"]
impl crate::Writable for OIER {}
#[doc = "OIER"]
pub mod oier;
#[doc = "OISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oisr](oisr) module"]
pub type OISR = crate::Reg<u32, _OISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OISR;
#[doc = "`read()` method returns [oisr::R](oisr::R) reader structure"]
impl crate::Readable for OISR {}
#[doc = "`write(|w| ..)` method takes [oisr::W](oisr::W) writer structure"]
impl crate::Writable for OISR {}
#[doc = "OISR"]
pub mod oisr;
#[doc = "PPSR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppsr0](ppsr0) module"]
pub type PPSR0 = crate::Reg<u32, _PPSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPSR0;
#[doc = "`read()` method returns [ppsr0::R](ppsr0::R) reader structure"]
impl crate::Readable for PPSR0 {}
#[doc = "`write(|w| ..)` method takes [ppsr0::W](ppsr0::W) writer structure"]
impl crate::Writable for PPSR0 {}
#[doc = "PPSR0"]
pub mod ppsr0;
#[doc = "PPSR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppsr1](ppsr1) module"]
pub type PPSR1 = crate::Reg<u32, _PPSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPSR1;
#[doc = "`read()` method returns [ppsr1::R](ppsr1::R) reader structure"]
impl crate::Readable for PPSR1 {}
#[doc = "`write(|w| ..)` method takes [ppsr1::W](ppsr1::W) writer structure"]
impl crate::Writable for PPSR1 {}
#[doc = "PPSR1"]
pub mod ppsr1;
#[doc = "PPSR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppsr2](ppsr2) module"]
pub type PPSR2 = crate::Reg<u32, _PPSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPSR2;
#[doc = "`read()` method returns [ppsr2::R](ppsr2::R) reader structure"]
impl crate::Readable for PPSR2 {}
#[doc = "`write(|w| ..)` method takes [ppsr2::W](ppsr2::W) writer structure"]
impl crate::Writable for PPSR2 {}
#[doc = "PPSR2"]
pub mod ppsr2;
#[doc = "PPSR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppsr3](ppsr3) module"]
pub type PPSR3 = crate::Reg<u32, _PPSR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPSR3;
#[doc = "`read()` method returns [ppsr3::R](ppsr3::R) reader structure"]
impl crate::Readable for PPSR3 {}
#[doc = "`write(|w| ..)` method takes [ppsr3::W](ppsr3::W) writer structure"]
impl crate::Writable for PPSR3 {}
#[doc = "PPSR3"]
pub mod ppsr3;
#[doc = "CPSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpsr](cpsr) module"]
pub type CPSR = crate::Reg<u32, _CPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPSR;
#[doc = "`read()` method returns [cpsr::R](cpsr::R) reader structure"]
impl crate::Readable for CPSR {}
#[doc = "`write(|w| ..)` method takes [cpsr::W](cpsr::W) writer structure"]
impl crate::Writable for CPSR {}
#[doc = "CPSR"]
pub mod cpsr;
#[doc = "VMCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vmcr](vmcr) module"]
pub type VMCR = crate::Reg<u32, _VMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VMCR;
#[doc = "`read()` method returns [vmcr::R](vmcr::R) reader structure"]
impl crate::Readable for VMCR {}
#[doc = "`write(|w| ..)` method takes [vmcr::W](vmcr::W) writer structure"]
impl crate::Writable for VMCR {}
#[doc = "VMCR"]
pub mod vmcr;
#[doc = "MDID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdid](mdid) module"]
pub type MDID = crate::Reg<u32, _MDID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDID;
#[doc = "`read()` method returns [mdid::R](mdid::R) reader structure"]
impl crate::Readable for MDID {}
#[doc = "`write(|w| ..)` method takes [mdid::W](mdid::W) writer structure"]
impl crate::Writable for MDID {}
#[doc = "MDID"]
pub mod mdid;
#[doc = "PNSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pnsr](pnsr) module"]
pub type PNSR = crate::Reg<u32, _PNSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PNSR;
#[doc = "`read()` method returns [pnsr::R](pnsr::R) reader structure"]
impl crate::Readable for PNSR {}
#[doc = "`write(|w| ..)` method takes [pnsr::W](pnsr::W) writer structure"]
impl crate::Writable for PNSR {}
#[doc = "PNSR"]
pub mod pnsr;
#[doc = "PSSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pssr](pssr) module"]
pub type PSSR = crate::Reg<u32, _PSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSSR;
#[doc = "`read()` method returns [pssr::R](pssr::R) reader structure"]
impl crate::Readable for PSSR {}
#[doc = "`write(|w| ..)` method takes [pssr::W](pssr::W) writer structure"]
impl crate::Writable for PSSR {}
#[doc = "PSSR"]
pub mod pssr;
#[doc = "CFCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfcr](cfcr) module"]
pub type CFCR = crate::Reg<u32, _CFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFCR;
#[doc = "`read()` method returns [cfcr::R](cfcr::R) reader structure"]
impl crate::Readable for CFCR {}
#[doc = "`write(|w| ..)` method takes [cfcr::W](cfcr::W) writer structure"]
impl crate::Writable for CFCR {}
#[doc = "CFCR"]
pub mod cfcr;
#[doc = "CID0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid0](cid0) module"]
pub type CID0 = crate::Reg<u32, _CID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID0;
#[doc = "`read()` method returns [cid0::R](cid0::R) reader structure"]
impl crate::Readable for CID0 {}
#[doc = "`write(|w| ..)` method takes [cid0::W](cid0::W) writer structure"]
impl crate::Writable for CID0 {}
#[doc = "CID0"]
pub mod cid0;
#[doc = "CID1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid1](cid1) module"]
pub type CID1 = crate::Reg<u32, _CID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID1;
#[doc = "`read()` method returns [cid1::R](cid1::R) reader structure"]
impl crate::Readable for CID1 {}
#[doc = "`write(|w| ..)` method takes [cid1::W](cid1::W) writer structure"]
impl crate::Writable for CID1 {}
#[doc = "CID1"]
pub mod cid1;
#[doc = "CID2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid2](cid2) module"]
pub type CID2 = crate::Reg<u32, _CID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID2;
#[doc = "`read()` method returns [cid2::R](cid2::R) reader structure"]
impl crate::Readable for CID2 {}
#[doc = "`write(|w| ..)` method takes [cid2::W](cid2::W) writer structure"]
impl crate::Writable for CID2 {}
#[doc = "CID2"]
pub mod cid2;
#[doc = "CID3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid3](cid3) module"]
pub type CID3 = crate::Reg<u32, _CID3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID3;
#[doc = "`read()` method returns [cid3::R](cid3::R) reader structure"]
impl crate::Readable for CID3 {}
#[doc = "`write(|w| ..)` method takes [cid3::W](cid3::W) writer structure"]
impl crate::Writable for CID3 {}
#[doc = "CID3"]
pub mod cid3;
