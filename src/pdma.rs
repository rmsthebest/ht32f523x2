#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CH0CR"]
    pub ch0cr: CH0CR,
    #[doc = "0x04 - CH0SADR"]
    pub ch0sadr: CH0SADR,
    #[doc = "0x08 - CH0DADR"]
    pub ch0dadr: CH0DADR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - CH0TSR"]
    pub ch0tsr: CH0TSR,
    #[doc = "0x14 - CH0CTSR"]
    pub ch0ctsr: CH0CTSR,
    #[doc = "0x18 - CH1CR"]
    pub ch1cr: CH1CR,
    #[doc = "0x1c - CH1SADR"]
    pub ch1sadr: CH1SADR,
    #[doc = "0x20 - CH1DADR"]
    pub ch1dadr: CH1DADR,
    _reserved8: [u8; 4usize],
    #[doc = "0x28 - CH1TSR"]
    pub ch1tsr: CH1TSR,
    #[doc = "0x2c - CH1CTSR"]
    pub ch1ctsr: CH1CTSR,
    #[doc = "0x30 - CH2CR"]
    pub ch2cr: CH2CR,
    #[doc = "0x34 - CH2SADR"]
    pub ch2sadr: CH2SADR,
    #[doc = "0x38 - CH2DADR"]
    pub ch2dadr: CH2DADR,
    _reserved13: [u8; 4usize],
    #[doc = "0x40 - CH2TSR"]
    pub ch2tsr: CH2TSR,
    #[doc = "0x44 - CH2CTSR"]
    pub ch2ctsr: CH2CTSR,
    #[doc = "0x48 - CH3CR"]
    pub ch3cr: CH3CR,
    #[doc = "0x4c - CH3SADR"]
    pub ch3sadr: CH3SADR,
    #[doc = "0x50 - CH3DADR"]
    pub ch3dadr: CH3DADR,
    _reserved18: [u8; 4usize],
    #[doc = "0x58 - CH3TSR"]
    pub ch3tsr: CH3TSR,
    #[doc = "0x5c - CH3CTSR"]
    pub ch3ctsr: CH3CTSR,
    #[doc = "0x60 - CH4CR"]
    pub ch4cr: CH4CR,
    #[doc = "0x64 - CH4SADR"]
    pub ch4sadr: CH4SADR,
    #[doc = "0x68 - CH4DADR"]
    pub ch4dadr: CH4DADR,
    _reserved23: [u8; 4usize],
    #[doc = "0x70 - CH4TSR"]
    pub ch4tsr: CH4TSR,
    #[doc = "0x74 - CH4CTSR"]
    pub ch4ctsr: CH4CTSR,
    #[doc = "0x78 - CH5CR"]
    pub ch5cr: CH5CR,
    #[doc = "0x7c - CH5SADR"]
    pub ch5sadr: CH5SADR,
    #[doc = "0x80 - CH5DADR"]
    pub ch5dadr: CH5DADR,
    _reserved28: [u8; 4usize],
    #[doc = "0x88 - CH5TSR"]
    pub ch5tsr: CH5TSR,
    #[doc = "0x8c - CH5CTSR"]
    pub ch5ctsr: CH5CTSR,
    _reserved30: [u8; 144usize],
    #[doc = "0x120 - ISR"]
    pub isr: ISR,
    _reserved31: [u8; 4usize],
    #[doc = "0x128 - ISCR"]
    pub iscr: ISCR,
    _reserved32: [u8; 4usize],
    #[doc = "0x130 - IER"]
    pub ier: IER,
}
#[doc = "CH0CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0cr](ch0cr) module"]
pub type CH0CR = crate::Reg<u32, _CH0CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0CR;
#[doc = "`read()` method returns [ch0cr::R](ch0cr::R) reader structure"]
impl crate::Readable for CH0CR {}
#[doc = "`write(|w| ..)` method takes [ch0cr::W](ch0cr::W) writer structure"]
impl crate::Writable for CH0CR {}
#[doc = "CH0CR"]
pub mod ch0cr;
#[doc = "CH0SADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0sadr](ch0sadr) module"]
pub type CH0SADR = crate::Reg<u32, _CH0SADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0SADR;
#[doc = "`read()` method returns [ch0sadr::R](ch0sadr::R) reader structure"]
impl crate::Readable for CH0SADR {}
#[doc = "`write(|w| ..)` method takes [ch0sadr::W](ch0sadr::W) writer structure"]
impl crate::Writable for CH0SADR {}
#[doc = "CH0SADR"]
pub mod ch0sadr;
#[doc = "CH0DADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0dadr](ch0dadr) module"]
pub type CH0DADR = crate::Reg<u32, _CH0DADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0DADR;
#[doc = "`read()` method returns [ch0dadr::R](ch0dadr::R) reader structure"]
impl crate::Readable for CH0DADR {}
#[doc = "`write(|w| ..)` method takes [ch0dadr::W](ch0dadr::W) writer structure"]
impl crate::Writable for CH0DADR {}
#[doc = "CH0DADR"]
pub mod ch0dadr;
#[doc = "CH0TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0tsr](ch0tsr) module"]
pub type CH0TSR = crate::Reg<u32, _CH0TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0TSR;
#[doc = "`read()` method returns [ch0tsr::R](ch0tsr::R) reader structure"]
impl crate::Readable for CH0TSR {}
#[doc = "`write(|w| ..)` method takes [ch0tsr::W](ch0tsr::W) writer structure"]
impl crate::Writable for CH0TSR {}
#[doc = "CH0TSR"]
pub mod ch0tsr;
#[doc = "CH0CTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0ctsr](ch0ctsr) module"]
pub type CH0CTSR = crate::Reg<u32, _CH0CTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0CTSR;
#[doc = "`read()` method returns [ch0ctsr::R](ch0ctsr::R) reader structure"]
impl crate::Readable for CH0CTSR {}
#[doc = "`write(|w| ..)` method takes [ch0ctsr::W](ch0ctsr::W) writer structure"]
impl crate::Writable for CH0CTSR {}
#[doc = "CH0CTSR"]
pub mod ch0ctsr;
#[doc = "CH1CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1cr](ch1cr) module"]
pub type CH1CR = crate::Reg<u32, _CH1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1CR;
#[doc = "`read()` method returns [ch1cr::R](ch1cr::R) reader structure"]
impl crate::Readable for CH1CR {}
#[doc = "`write(|w| ..)` method takes [ch1cr::W](ch1cr::W) writer structure"]
impl crate::Writable for CH1CR {}
#[doc = "CH1CR"]
pub mod ch1cr;
#[doc = "CH1SADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1sadr](ch1sadr) module"]
pub type CH1SADR = crate::Reg<u32, _CH1SADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1SADR;
#[doc = "`read()` method returns [ch1sadr::R](ch1sadr::R) reader structure"]
impl crate::Readable for CH1SADR {}
#[doc = "`write(|w| ..)` method takes [ch1sadr::W](ch1sadr::W) writer structure"]
impl crate::Writable for CH1SADR {}
#[doc = "CH1SADR"]
pub mod ch1sadr;
#[doc = "CH1DADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1dadr](ch1dadr) module"]
pub type CH1DADR = crate::Reg<u32, _CH1DADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1DADR;
#[doc = "`read()` method returns [ch1dadr::R](ch1dadr::R) reader structure"]
impl crate::Readable for CH1DADR {}
#[doc = "`write(|w| ..)` method takes [ch1dadr::W](ch1dadr::W) writer structure"]
impl crate::Writable for CH1DADR {}
#[doc = "CH1DADR"]
pub mod ch1dadr;
#[doc = "CH1TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1tsr](ch1tsr) module"]
pub type CH1TSR = crate::Reg<u32, _CH1TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1TSR;
#[doc = "`read()` method returns [ch1tsr::R](ch1tsr::R) reader structure"]
impl crate::Readable for CH1TSR {}
#[doc = "`write(|w| ..)` method takes [ch1tsr::W](ch1tsr::W) writer structure"]
impl crate::Writable for CH1TSR {}
#[doc = "CH1TSR"]
pub mod ch1tsr;
#[doc = "CH1CTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1ctsr](ch1ctsr) module"]
pub type CH1CTSR = crate::Reg<u32, _CH1CTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1CTSR;
#[doc = "`read()` method returns [ch1ctsr::R](ch1ctsr::R) reader structure"]
impl crate::Readable for CH1CTSR {}
#[doc = "`write(|w| ..)` method takes [ch1ctsr::W](ch1ctsr::W) writer structure"]
impl crate::Writable for CH1CTSR {}
#[doc = "CH1CTSR"]
pub mod ch1ctsr;
#[doc = "CH2CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2cr](ch2cr) module"]
pub type CH2CR = crate::Reg<u32, _CH2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2CR;
#[doc = "`read()` method returns [ch2cr::R](ch2cr::R) reader structure"]
impl crate::Readable for CH2CR {}
#[doc = "`write(|w| ..)` method takes [ch2cr::W](ch2cr::W) writer structure"]
impl crate::Writable for CH2CR {}
#[doc = "CH2CR"]
pub mod ch2cr;
#[doc = "CH2SADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2sadr](ch2sadr) module"]
pub type CH2SADR = crate::Reg<u32, _CH2SADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2SADR;
#[doc = "`read()` method returns [ch2sadr::R](ch2sadr::R) reader structure"]
impl crate::Readable for CH2SADR {}
#[doc = "`write(|w| ..)` method takes [ch2sadr::W](ch2sadr::W) writer structure"]
impl crate::Writable for CH2SADR {}
#[doc = "CH2SADR"]
pub mod ch2sadr;
#[doc = "CH2DADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2dadr](ch2dadr) module"]
pub type CH2DADR = crate::Reg<u32, _CH2DADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2DADR;
#[doc = "`read()` method returns [ch2dadr::R](ch2dadr::R) reader structure"]
impl crate::Readable for CH2DADR {}
#[doc = "`write(|w| ..)` method takes [ch2dadr::W](ch2dadr::W) writer structure"]
impl crate::Writable for CH2DADR {}
#[doc = "CH2DADR"]
pub mod ch2dadr;
#[doc = "CH2TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2tsr](ch2tsr) module"]
pub type CH2TSR = crate::Reg<u32, _CH2TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2TSR;
#[doc = "`read()` method returns [ch2tsr::R](ch2tsr::R) reader structure"]
impl crate::Readable for CH2TSR {}
#[doc = "`write(|w| ..)` method takes [ch2tsr::W](ch2tsr::W) writer structure"]
impl crate::Writable for CH2TSR {}
#[doc = "CH2TSR"]
pub mod ch2tsr;
#[doc = "CH2CTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2ctsr](ch2ctsr) module"]
pub type CH2CTSR = crate::Reg<u32, _CH2CTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2CTSR;
#[doc = "`read()` method returns [ch2ctsr::R](ch2ctsr::R) reader structure"]
impl crate::Readable for CH2CTSR {}
#[doc = "`write(|w| ..)` method takes [ch2ctsr::W](ch2ctsr::W) writer structure"]
impl crate::Writable for CH2CTSR {}
#[doc = "CH2CTSR"]
pub mod ch2ctsr;
#[doc = "CH3CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3cr](ch3cr) module"]
pub type CH3CR = crate::Reg<u32, _CH3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3CR;
#[doc = "`read()` method returns [ch3cr::R](ch3cr::R) reader structure"]
impl crate::Readable for CH3CR {}
#[doc = "`write(|w| ..)` method takes [ch3cr::W](ch3cr::W) writer structure"]
impl crate::Writable for CH3CR {}
#[doc = "CH3CR"]
pub mod ch3cr;
#[doc = "CH3SADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3sadr](ch3sadr) module"]
pub type CH3SADR = crate::Reg<u32, _CH3SADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3SADR;
#[doc = "`read()` method returns [ch3sadr::R](ch3sadr::R) reader structure"]
impl crate::Readable for CH3SADR {}
#[doc = "`write(|w| ..)` method takes [ch3sadr::W](ch3sadr::W) writer structure"]
impl crate::Writable for CH3SADR {}
#[doc = "CH3SADR"]
pub mod ch3sadr;
#[doc = "CH3DADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3dadr](ch3dadr) module"]
pub type CH3DADR = crate::Reg<u32, _CH3DADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3DADR;
#[doc = "`read()` method returns [ch3dadr::R](ch3dadr::R) reader structure"]
impl crate::Readable for CH3DADR {}
#[doc = "`write(|w| ..)` method takes [ch3dadr::W](ch3dadr::W) writer structure"]
impl crate::Writable for CH3DADR {}
#[doc = "CH3DADR"]
pub mod ch3dadr;
#[doc = "CH3TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3tsr](ch3tsr) module"]
pub type CH3TSR = crate::Reg<u32, _CH3TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3TSR;
#[doc = "`read()` method returns [ch3tsr::R](ch3tsr::R) reader structure"]
impl crate::Readable for CH3TSR {}
#[doc = "`write(|w| ..)` method takes [ch3tsr::W](ch3tsr::W) writer structure"]
impl crate::Writable for CH3TSR {}
#[doc = "CH3TSR"]
pub mod ch3tsr;
#[doc = "CH3CTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3ctsr](ch3ctsr) module"]
pub type CH3CTSR = crate::Reg<u32, _CH3CTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3CTSR;
#[doc = "`read()` method returns [ch3ctsr::R](ch3ctsr::R) reader structure"]
impl crate::Readable for CH3CTSR {}
#[doc = "`write(|w| ..)` method takes [ch3ctsr::W](ch3ctsr::W) writer structure"]
impl crate::Writable for CH3CTSR {}
#[doc = "CH3CTSR"]
pub mod ch3ctsr;
#[doc = "CH4CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4cr](ch4cr) module"]
pub type CH4CR = crate::Reg<u32, _CH4CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4CR;
#[doc = "`read()` method returns [ch4cr::R](ch4cr::R) reader structure"]
impl crate::Readable for CH4CR {}
#[doc = "`write(|w| ..)` method takes [ch4cr::W](ch4cr::W) writer structure"]
impl crate::Writable for CH4CR {}
#[doc = "CH4CR"]
pub mod ch4cr;
#[doc = "CH4SADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4sadr](ch4sadr) module"]
pub type CH4SADR = crate::Reg<u32, _CH4SADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4SADR;
#[doc = "`read()` method returns [ch4sadr::R](ch4sadr::R) reader structure"]
impl crate::Readable for CH4SADR {}
#[doc = "`write(|w| ..)` method takes [ch4sadr::W](ch4sadr::W) writer structure"]
impl crate::Writable for CH4SADR {}
#[doc = "CH4SADR"]
pub mod ch4sadr;
#[doc = "CH4DADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4dadr](ch4dadr) module"]
pub type CH4DADR = crate::Reg<u32, _CH4DADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4DADR;
#[doc = "`read()` method returns [ch4dadr::R](ch4dadr::R) reader structure"]
impl crate::Readable for CH4DADR {}
#[doc = "`write(|w| ..)` method takes [ch4dadr::W](ch4dadr::W) writer structure"]
impl crate::Writable for CH4DADR {}
#[doc = "CH4DADR"]
pub mod ch4dadr;
#[doc = "CH4TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4tsr](ch4tsr) module"]
pub type CH4TSR = crate::Reg<u32, _CH4TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4TSR;
#[doc = "`read()` method returns [ch4tsr::R](ch4tsr::R) reader structure"]
impl crate::Readable for CH4TSR {}
#[doc = "`write(|w| ..)` method takes [ch4tsr::W](ch4tsr::W) writer structure"]
impl crate::Writable for CH4TSR {}
#[doc = "CH4TSR"]
pub mod ch4tsr;
#[doc = "CH4CTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4ctsr](ch4ctsr) module"]
pub type CH4CTSR = crate::Reg<u32, _CH4CTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4CTSR;
#[doc = "`read()` method returns [ch4ctsr::R](ch4ctsr::R) reader structure"]
impl crate::Readable for CH4CTSR {}
#[doc = "`write(|w| ..)` method takes [ch4ctsr::W](ch4ctsr::W) writer structure"]
impl crate::Writable for CH4CTSR {}
#[doc = "CH4CTSR"]
pub mod ch4ctsr;
#[doc = "CH5CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5cr](ch5cr) module"]
pub type CH5CR = crate::Reg<u32, _CH5CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5CR;
#[doc = "`read()` method returns [ch5cr::R](ch5cr::R) reader structure"]
impl crate::Readable for CH5CR {}
#[doc = "`write(|w| ..)` method takes [ch5cr::W](ch5cr::W) writer structure"]
impl crate::Writable for CH5CR {}
#[doc = "CH5CR"]
pub mod ch5cr;
#[doc = "CH5SADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5sadr](ch5sadr) module"]
pub type CH5SADR = crate::Reg<u32, _CH5SADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5SADR;
#[doc = "`read()` method returns [ch5sadr::R](ch5sadr::R) reader structure"]
impl crate::Readable for CH5SADR {}
#[doc = "`write(|w| ..)` method takes [ch5sadr::W](ch5sadr::W) writer structure"]
impl crate::Writable for CH5SADR {}
#[doc = "CH5SADR"]
pub mod ch5sadr;
#[doc = "CH5DADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5dadr](ch5dadr) module"]
pub type CH5DADR = crate::Reg<u32, _CH5DADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5DADR;
#[doc = "`read()` method returns [ch5dadr::R](ch5dadr::R) reader structure"]
impl crate::Readable for CH5DADR {}
#[doc = "`write(|w| ..)` method takes [ch5dadr::W](ch5dadr::W) writer structure"]
impl crate::Writable for CH5DADR {}
#[doc = "CH5DADR"]
pub mod ch5dadr;
#[doc = "CH5TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5tsr](ch5tsr) module"]
pub type CH5TSR = crate::Reg<u32, _CH5TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5TSR;
#[doc = "`read()` method returns [ch5tsr::R](ch5tsr::R) reader structure"]
impl crate::Readable for CH5TSR {}
#[doc = "`write(|w| ..)` method takes [ch5tsr::W](ch5tsr::W) writer structure"]
impl crate::Writable for CH5TSR {}
#[doc = "CH5TSR"]
pub mod ch5tsr;
#[doc = "CH5CTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5ctsr](ch5ctsr) module"]
pub type CH5CTSR = crate::Reg<u32, _CH5CTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5CTSR;
#[doc = "`read()` method returns [ch5ctsr::R](ch5ctsr::R) reader structure"]
impl crate::Readable for CH5CTSR {}
#[doc = "`write(|w| ..)` method takes [ch5ctsr::W](ch5ctsr::W) writer structure"]
impl crate::Writable for CH5CTSR {}
#[doc = "CH5CTSR"]
pub mod ch5ctsr;
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
#[doc = "ISCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iscr](iscr) module"]
pub type ISCR = crate::Reg<u32, _ISCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISCR;
#[doc = "`read()` method returns [iscr::R](iscr::R) reader structure"]
impl crate::Readable for ISCR {}
#[doc = "`write(|w| ..)` method takes [iscr::W](iscr::W) writer structure"]
impl crate::Writable for ISCR {}
#[doc = "ISCR"]
pub mod iscr;
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
