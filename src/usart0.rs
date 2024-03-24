#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    usart_usrdr: UsartUsrdr,
    usart_usrcr: UsartUsrcr,
    usart_usrfcr: UsartUsrfcr,
    usart_usrier: UsartUsrier,
    usart_usrsifr: UsartUsrsifr,
    usart_usrtpr: UsartUsrtpr,
    usart_ir_dacr: UsartIrDacr,
    usart_rs485cr: UsartRs485cr,
    usart_syncr: UsartSyncr,
    usart_usrdlr: UsartUsrdlr,
    usart_usrtstr: UsartUsrtstr,
}
impl RegisterBlock {
    #[doc = "0x00 - USART_USRDR"]
    #[inline(always)]
    pub const fn usart_usrdr(&self) -> &UsartUsrdr {
        &self.usart_usrdr
    }
    #[doc = "0x04 - USART_USRCR"]
    #[inline(always)]
    pub const fn usart_usrcr(&self) -> &UsartUsrcr {
        &self.usart_usrcr
    }
    #[doc = "0x08 - USART_USRFCR"]
    #[inline(always)]
    pub const fn usart_usrfcr(&self) -> &UsartUsrfcr {
        &self.usart_usrfcr
    }
    #[doc = "0x0c - USART_USRIER"]
    #[inline(always)]
    pub const fn usart_usrier(&self) -> &UsartUsrier {
        &self.usart_usrier
    }
    #[doc = "0x10 - USART_USRSIFR"]
    #[inline(always)]
    pub const fn usart_usrsifr(&self) -> &UsartUsrsifr {
        &self.usart_usrsifr
    }
    #[doc = "0x14 - USART_USRTPR"]
    #[inline(always)]
    pub const fn usart_usrtpr(&self) -> &UsartUsrtpr {
        &self.usart_usrtpr
    }
    #[doc = "0x18 - USART_IrDACR"]
    #[inline(always)]
    pub const fn usart_ir_dacr(&self) -> &UsartIrDacr {
        &self.usart_ir_dacr
    }
    #[doc = "0x1c - USART_RS485CR"]
    #[inline(always)]
    pub const fn usart_rs485cr(&self) -> &UsartRs485cr {
        &self.usart_rs485cr
    }
    #[doc = "0x20 - USART_SYNCR"]
    #[inline(always)]
    pub const fn usart_syncr(&self) -> &UsartSyncr {
        &self.usart_syncr
    }
    #[doc = "0x24 - USART_USRDLR"]
    #[inline(always)]
    pub const fn usart_usrdlr(&self) -> &UsartUsrdlr {
        &self.usart_usrdlr
    }
    #[doc = "0x28 - USART_USRTSTR"]
    #[inline(always)]
    pub const fn usart_usrtstr(&self) -> &UsartUsrtstr {
        &self.usart_usrtstr
    }
}
#[doc = "USART_USRDR (rw) register accessor: USART_USRDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_usrdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_usrdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_usrdr`]
module"]
#[doc(alias = "USART_USRDR")]
pub type UsartUsrdr = crate::Reg<usart_usrdr::UsartUsrdrSpec>;
#[doc = "USART_USRDR"]
pub mod usart_usrdr;
#[doc = "USART_USRCR (rw) register accessor: USART_USRCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_usrcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_usrcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_usrcr`]
module"]
#[doc(alias = "USART_USRCR")]
pub type UsartUsrcr = crate::Reg<usart_usrcr::UsartUsrcrSpec>;
#[doc = "USART_USRCR"]
pub mod usart_usrcr;
#[doc = "USART_USRFCR (rw) register accessor: USART_USRFCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_usrfcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_usrfcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_usrfcr`]
module"]
#[doc(alias = "USART_USRFCR")]
pub type UsartUsrfcr = crate::Reg<usart_usrfcr::UsartUsrfcrSpec>;
#[doc = "USART_USRFCR"]
pub mod usart_usrfcr;
#[doc = "USART_USRIER (rw) register accessor: USART_USRIER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_usrier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_usrier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_usrier`]
module"]
#[doc(alias = "USART_USRIER")]
pub type UsartUsrier = crate::Reg<usart_usrier::UsartUsrierSpec>;
#[doc = "USART_USRIER"]
pub mod usart_usrier;
#[doc = "USART_USRSIFR (rw) register accessor: USART_USRSIFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_usrsifr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_usrsifr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_usrsifr`]
module"]
#[doc(alias = "USART_USRSIFR")]
pub type UsartUsrsifr = crate::Reg<usart_usrsifr::UsartUsrsifrSpec>;
#[doc = "USART_USRSIFR"]
pub mod usart_usrsifr;
#[doc = "USART_USRTPR (rw) register accessor: USART_USRTPR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_usrtpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_usrtpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_usrtpr`]
module"]
#[doc(alias = "USART_USRTPR")]
pub type UsartUsrtpr = crate::Reg<usart_usrtpr::UsartUsrtprSpec>;
#[doc = "USART_USRTPR"]
pub mod usart_usrtpr;
#[doc = "USART_IrDACR (rw) register accessor: USART_IrDACR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_ir_dacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_ir_dacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_ir_dacr`]
module"]
#[doc(alias = "USART_IrDACR")]
pub type UsartIrDacr = crate::Reg<usart_ir_dacr::UsartIrDacrSpec>;
#[doc = "USART_IrDACR"]
pub mod usart_ir_dacr;
#[doc = "USART_RS485CR (rw) register accessor: USART_RS485CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_rs485cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_rs485cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_rs485cr`]
module"]
#[doc(alias = "USART_RS485CR")]
pub type UsartRs485cr = crate::Reg<usart_rs485cr::UsartRs485crSpec>;
#[doc = "USART_RS485CR"]
pub mod usart_rs485cr;
#[doc = "USART_SYNCR (rw) register accessor: USART_SYNCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_syncr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_syncr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_syncr`]
module"]
#[doc(alias = "USART_SYNCR")]
pub type UsartSyncr = crate::Reg<usart_syncr::UsartSyncrSpec>;
#[doc = "USART_SYNCR"]
pub mod usart_syncr;
#[doc = "USART_USRDLR (rw) register accessor: USART_USRDLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_usrdlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_usrdlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_usrdlr`]
module"]
#[doc(alias = "USART_USRDLR")]
pub type UsartUsrdlr = crate::Reg<usart_usrdlr::UsartUsrdlrSpec>;
#[doc = "USART_USRDLR"]
pub mod usart_usrdlr;
#[doc = "USART_USRTSTR (rw) register accessor: USART_USRTSTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usart_usrtstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usart_usrtstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart_usrtstr`]
module"]
#[doc(alias = "USART_USRTSTR")]
pub type UsartUsrtstr = crate::Reg<usart_usrtstr::UsartUsrtstrSpec>;
#[doc = "USART_USRTSTR"]
pub mod usart_usrtstr;
