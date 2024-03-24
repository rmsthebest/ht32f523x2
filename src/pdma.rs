#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ch0cr: Ch0cr,
    ch0sadr: Ch0sadr,
    ch0dadr: Ch0dadr,
    _reserved3: [u8; 0x04],
    ch0tsr: Ch0tsr,
    ch0ctsr: Ch0ctsr,
    ch1cr: Ch1cr,
    ch1sadr: Ch1sadr,
    ch1dadr: Ch1dadr,
    _reserved8: [u8; 0x04],
    ch1tsr: Ch1tsr,
    ch1ctsr: Ch1ctsr,
    ch2cr: Ch2cr,
    ch2sadr: Ch2sadr,
    ch2dadr: Ch2dadr,
    _reserved13: [u8; 0x04],
    ch2tsr: Ch2tsr,
    ch2ctsr: Ch2ctsr,
    ch3cr: Ch3cr,
    ch3sadr: Ch3sadr,
    ch3dadr: Ch3dadr,
    _reserved18: [u8; 0x04],
    ch3tsr: Ch3tsr,
    ch3ctsr: Ch3ctsr,
    ch4cr: Ch4cr,
    ch4sadr: Ch4sadr,
    ch4dadr: Ch4dadr,
    _reserved23: [u8; 0x04],
    ch4tsr: Ch4tsr,
    ch4ctsr: Ch4ctsr,
    ch5cr: Ch5cr,
    ch5sadr: Ch5sadr,
    ch5dadr: Ch5dadr,
    _reserved28: [u8; 0x04],
    ch5tsr: Ch5tsr,
    ch5ctsr: Ch5ctsr,
    _reserved30: [u8; 0x90],
    isr: Isr,
    _reserved31: [u8; 0x04],
    iscr: Iscr,
    _reserved32: [u8; 0x04],
    ier: Ier,
}
impl RegisterBlock {
    #[doc = "0x00 - CH0CR"]
    #[inline(always)]
    pub const fn ch0cr(&self) -> &Ch0cr {
        &self.ch0cr
    }
    #[doc = "0x04 - CH0SADR"]
    #[inline(always)]
    pub const fn ch0sadr(&self) -> &Ch0sadr {
        &self.ch0sadr
    }
    #[doc = "0x08 - CH0DADR"]
    #[inline(always)]
    pub const fn ch0dadr(&self) -> &Ch0dadr {
        &self.ch0dadr
    }
    #[doc = "0x10 - CH0TSR"]
    #[inline(always)]
    pub const fn ch0tsr(&self) -> &Ch0tsr {
        &self.ch0tsr
    }
    #[doc = "0x14 - CH0CTSR"]
    #[inline(always)]
    pub const fn ch0ctsr(&self) -> &Ch0ctsr {
        &self.ch0ctsr
    }
    #[doc = "0x18 - CH1CR"]
    #[inline(always)]
    pub const fn ch1cr(&self) -> &Ch1cr {
        &self.ch1cr
    }
    #[doc = "0x1c - CH1SADR"]
    #[inline(always)]
    pub const fn ch1sadr(&self) -> &Ch1sadr {
        &self.ch1sadr
    }
    #[doc = "0x20 - CH1DADR"]
    #[inline(always)]
    pub const fn ch1dadr(&self) -> &Ch1dadr {
        &self.ch1dadr
    }
    #[doc = "0x28 - CH1TSR"]
    #[inline(always)]
    pub const fn ch1tsr(&self) -> &Ch1tsr {
        &self.ch1tsr
    }
    #[doc = "0x2c - CH1CTSR"]
    #[inline(always)]
    pub const fn ch1ctsr(&self) -> &Ch1ctsr {
        &self.ch1ctsr
    }
    #[doc = "0x30 - CH2CR"]
    #[inline(always)]
    pub const fn ch2cr(&self) -> &Ch2cr {
        &self.ch2cr
    }
    #[doc = "0x34 - CH2SADR"]
    #[inline(always)]
    pub const fn ch2sadr(&self) -> &Ch2sadr {
        &self.ch2sadr
    }
    #[doc = "0x38 - CH2DADR"]
    #[inline(always)]
    pub const fn ch2dadr(&self) -> &Ch2dadr {
        &self.ch2dadr
    }
    #[doc = "0x40 - CH2TSR"]
    #[inline(always)]
    pub const fn ch2tsr(&self) -> &Ch2tsr {
        &self.ch2tsr
    }
    #[doc = "0x44 - CH2CTSR"]
    #[inline(always)]
    pub const fn ch2ctsr(&self) -> &Ch2ctsr {
        &self.ch2ctsr
    }
    #[doc = "0x48 - CH3CR"]
    #[inline(always)]
    pub const fn ch3cr(&self) -> &Ch3cr {
        &self.ch3cr
    }
    #[doc = "0x4c - CH3SADR"]
    #[inline(always)]
    pub const fn ch3sadr(&self) -> &Ch3sadr {
        &self.ch3sadr
    }
    #[doc = "0x50 - CH3DADR"]
    #[inline(always)]
    pub const fn ch3dadr(&self) -> &Ch3dadr {
        &self.ch3dadr
    }
    #[doc = "0x58 - CH3TSR"]
    #[inline(always)]
    pub const fn ch3tsr(&self) -> &Ch3tsr {
        &self.ch3tsr
    }
    #[doc = "0x5c - CH3CTSR"]
    #[inline(always)]
    pub const fn ch3ctsr(&self) -> &Ch3ctsr {
        &self.ch3ctsr
    }
    #[doc = "0x60 - CH4CR"]
    #[inline(always)]
    pub const fn ch4cr(&self) -> &Ch4cr {
        &self.ch4cr
    }
    #[doc = "0x64 - CH4SADR"]
    #[inline(always)]
    pub const fn ch4sadr(&self) -> &Ch4sadr {
        &self.ch4sadr
    }
    #[doc = "0x68 - CH4DADR"]
    #[inline(always)]
    pub const fn ch4dadr(&self) -> &Ch4dadr {
        &self.ch4dadr
    }
    #[doc = "0x70 - CH4TSR"]
    #[inline(always)]
    pub const fn ch4tsr(&self) -> &Ch4tsr {
        &self.ch4tsr
    }
    #[doc = "0x74 - CH4CTSR"]
    #[inline(always)]
    pub const fn ch4ctsr(&self) -> &Ch4ctsr {
        &self.ch4ctsr
    }
    #[doc = "0x78 - CH5CR"]
    #[inline(always)]
    pub const fn ch5cr(&self) -> &Ch5cr {
        &self.ch5cr
    }
    #[doc = "0x7c - CH5SADR"]
    #[inline(always)]
    pub const fn ch5sadr(&self) -> &Ch5sadr {
        &self.ch5sadr
    }
    #[doc = "0x80 - CH5DADR"]
    #[inline(always)]
    pub const fn ch5dadr(&self) -> &Ch5dadr {
        &self.ch5dadr
    }
    #[doc = "0x88 - CH5TSR"]
    #[inline(always)]
    pub const fn ch5tsr(&self) -> &Ch5tsr {
        &self.ch5tsr
    }
    #[doc = "0x8c - CH5CTSR"]
    #[inline(always)]
    pub const fn ch5ctsr(&self) -> &Ch5ctsr {
        &self.ch5ctsr
    }
    #[doc = "0x120 - ISR"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x128 - ISCR"]
    #[inline(always)]
    pub const fn iscr(&self) -> &Iscr {
        &self.iscr
    }
    #[doc = "0x130 - IER"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
}
#[doc = "CH0CR (rw) register accessor: CH0CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cr`]
module"]
#[doc(alias = "CH0CR")]
pub type Ch0cr = crate::Reg<ch0cr::Ch0crSpec>;
#[doc = "CH0CR"]
pub mod ch0cr;
#[doc = "CH0SADR (rw) register accessor: CH0SADR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0sadr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0sadr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0sadr`]
module"]
#[doc(alias = "CH0SADR")]
pub type Ch0sadr = crate::Reg<ch0sadr::Ch0sadrSpec>;
#[doc = "CH0SADR"]
pub mod ch0sadr;
#[doc = "CH0DADR (rw) register accessor: CH0DADR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0dadr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0dadr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0dadr`]
module"]
#[doc(alias = "CH0DADR")]
pub type Ch0dadr = crate::Reg<ch0dadr::Ch0dadrSpec>;
#[doc = "CH0DADR"]
pub mod ch0dadr;
#[doc = "CH0TSR (rw) register accessor: CH0TSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0tsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0tsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0tsr`]
module"]
#[doc(alias = "CH0TSR")]
pub type Ch0tsr = crate::Reg<ch0tsr::Ch0tsrSpec>;
#[doc = "CH0TSR"]
pub mod ch0tsr;
#[doc = "CH0CTSR (rw) register accessor: CH0CTSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0ctsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0ctsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0ctsr`]
module"]
#[doc(alias = "CH0CTSR")]
pub type Ch0ctsr = crate::Reg<ch0ctsr::Ch0ctsrSpec>;
#[doc = "CH0CTSR"]
pub mod ch0ctsr;
#[doc = "CH1CR (rw) register accessor: CH1CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cr`]
module"]
#[doc(alias = "CH1CR")]
pub type Ch1cr = crate::Reg<ch1cr::Ch1crSpec>;
#[doc = "CH1CR"]
pub mod ch1cr;
#[doc = "CH1SADR (rw) register accessor: CH1SADR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1sadr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1sadr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1sadr`]
module"]
#[doc(alias = "CH1SADR")]
pub type Ch1sadr = crate::Reg<ch1sadr::Ch1sadrSpec>;
#[doc = "CH1SADR"]
pub mod ch1sadr;
#[doc = "CH1DADR (rw) register accessor: CH1DADR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1dadr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1dadr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1dadr`]
module"]
#[doc(alias = "CH1DADR")]
pub type Ch1dadr = crate::Reg<ch1dadr::Ch1dadrSpec>;
#[doc = "CH1DADR"]
pub mod ch1dadr;
#[doc = "CH1TSR (rw) register accessor: CH1TSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1tsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1tsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1tsr`]
module"]
#[doc(alias = "CH1TSR")]
pub type Ch1tsr = crate::Reg<ch1tsr::Ch1tsrSpec>;
#[doc = "CH1TSR"]
pub mod ch1tsr;
#[doc = "CH1CTSR (rw) register accessor: CH1CTSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1ctsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1ctsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1ctsr`]
module"]
#[doc(alias = "CH1CTSR")]
pub type Ch1ctsr = crate::Reg<ch1ctsr::Ch1ctsrSpec>;
#[doc = "CH1CTSR"]
pub mod ch1ctsr;
#[doc = "CH2CR (rw) register accessor: CH2CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cr`]
module"]
#[doc(alias = "CH2CR")]
pub type Ch2cr = crate::Reg<ch2cr::Ch2crSpec>;
#[doc = "CH2CR"]
pub mod ch2cr;
#[doc = "CH2SADR (rw) register accessor: CH2SADR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2sadr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2sadr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2sadr`]
module"]
#[doc(alias = "CH2SADR")]
pub type Ch2sadr = crate::Reg<ch2sadr::Ch2sadrSpec>;
#[doc = "CH2SADR"]
pub mod ch2sadr;
#[doc = "CH2DADR (rw) register accessor: CH2DADR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2dadr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2dadr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2dadr`]
module"]
#[doc(alias = "CH2DADR")]
pub type Ch2dadr = crate::Reg<ch2dadr::Ch2dadrSpec>;
#[doc = "CH2DADR"]
pub mod ch2dadr;
#[doc = "CH2TSR (rw) register accessor: CH2TSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2tsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2tsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2tsr`]
module"]
#[doc(alias = "CH2TSR")]
pub type Ch2tsr = crate::Reg<ch2tsr::Ch2tsrSpec>;
#[doc = "CH2TSR"]
pub mod ch2tsr;
#[doc = "CH2CTSR (rw) register accessor: CH2CTSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2ctsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2ctsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2ctsr`]
module"]
#[doc(alias = "CH2CTSR")]
pub type Ch2ctsr = crate::Reg<ch2ctsr::Ch2ctsrSpec>;
#[doc = "CH2CTSR"]
pub mod ch2ctsr;
#[doc = "CH3CR (rw) register accessor: CH3CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cr`]
module"]
#[doc(alias = "CH3CR")]
pub type Ch3cr = crate::Reg<ch3cr::Ch3crSpec>;
#[doc = "CH3CR"]
pub mod ch3cr;
#[doc = "CH3SADR (rw) register accessor: CH3SADR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3sadr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3sadr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3sadr`]
module"]
#[doc(alias = "CH3SADR")]
pub type Ch3sadr = crate::Reg<ch3sadr::Ch3sadrSpec>;
#[doc = "CH3SADR"]
pub mod ch3sadr;
#[doc = "CH3DADR (rw) register accessor: CH3DADR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3dadr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3dadr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3dadr`]
module"]
#[doc(alias = "CH3DADR")]
pub type Ch3dadr = crate::Reg<ch3dadr::Ch3dadrSpec>;
#[doc = "CH3DADR"]
pub mod ch3dadr;
#[doc = "CH3TSR (rw) register accessor: CH3TSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3tsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3tsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3tsr`]
module"]
#[doc(alias = "CH3TSR")]
pub type Ch3tsr = crate::Reg<ch3tsr::Ch3tsrSpec>;
#[doc = "CH3TSR"]
pub mod ch3tsr;
#[doc = "CH3CTSR (rw) register accessor: CH3CTSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3ctsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3ctsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3ctsr`]
module"]
#[doc(alias = "CH3CTSR")]
pub type Ch3ctsr = crate::Reg<ch3ctsr::Ch3ctsrSpec>;
#[doc = "CH3CTSR"]
pub mod ch3ctsr;
#[doc = "CH4CR (rw) register accessor: CH4CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cr`]
module"]
#[doc(alias = "CH4CR")]
pub type Ch4cr = crate::Reg<ch4cr::Ch4crSpec>;
#[doc = "CH4CR"]
pub mod ch4cr;
#[doc = "CH4SADR (rw) register accessor: CH4SADR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4sadr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4sadr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4sadr`]
module"]
#[doc(alias = "CH4SADR")]
pub type Ch4sadr = crate::Reg<ch4sadr::Ch4sadrSpec>;
#[doc = "CH4SADR"]
pub mod ch4sadr;
#[doc = "CH4DADR (rw) register accessor: CH4DADR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4dadr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4dadr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4dadr`]
module"]
#[doc(alias = "CH4DADR")]
pub type Ch4dadr = crate::Reg<ch4dadr::Ch4dadrSpec>;
#[doc = "CH4DADR"]
pub mod ch4dadr;
#[doc = "CH4TSR (rw) register accessor: CH4TSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4tsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4tsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4tsr`]
module"]
#[doc(alias = "CH4TSR")]
pub type Ch4tsr = crate::Reg<ch4tsr::Ch4tsrSpec>;
#[doc = "CH4TSR"]
pub mod ch4tsr;
#[doc = "CH4CTSR (rw) register accessor: CH4CTSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4ctsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4ctsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4ctsr`]
module"]
#[doc(alias = "CH4CTSR")]
pub type Ch4ctsr = crate::Reg<ch4ctsr::Ch4ctsrSpec>;
#[doc = "CH4CTSR"]
pub mod ch4ctsr;
#[doc = "CH5CR (rw) register accessor: CH5CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5cr`]
module"]
#[doc(alias = "CH5CR")]
pub type Ch5cr = crate::Reg<ch5cr::Ch5crSpec>;
#[doc = "CH5CR"]
pub mod ch5cr;
#[doc = "CH5SADR (rw) register accessor: CH5SADR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5sadr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5sadr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5sadr`]
module"]
#[doc(alias = "CH5SADR")]
pub type Ch5sadr = crate::Reg<ch5sadr::Ch5sadrSpec>;
#[doc = "CH5SADR"]
pub mod ch5sadr;
#[doc = "CH5DADR (rw) register accessor: CH5DADR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5dadr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5dadr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5dadr`]
module"]
#[doc(alias = "CH5DADR")]
pub type Ch5dadr = crate::Reg<ch5dadr::Ch5dadrSpec>;
#[doc = "CH5DADR"]
pub mod ch5dadr;
#[doc = "CH5TSR (rw) register accessor: CH5TSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5tsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5tsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5tsr`]
module"]
#[doc(alias = "CH5TSR")]
pub type Ch5tsr = crate::Reg<ch5tsr::Ch5tsrSpec>;
#[doc = "CH5TSR"]
pub mod ch5tsr;
#[doc = "CH5CTSR (rw) register accessor: CH5CTSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5ctsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5ctsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5ctsr`]
module"]
#[doc(alias = "CH5CTSR")]
pub type Ch5ctsr = crate::Reg<ch5ctsr::Ch5ctsrSpec>;
#[doc = "CH5CTSR"]
pub mod ch5ctsr;
#[doc = "ISR (rw) register accessor: ISR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "ISR"]
pub mod isr;
#[doc = "ISCR (rw) register accessor: ISCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iscr`]
module"]
#[doc(alias = "ISCR")]
pub type Iscr = crate::Reg<iscr::IscrSpec>;
#[doc = "ISCR"]
pub mod iscr;
#[doc = "IER (rw) register accessor: IER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "IER"]
pub mod ier;
