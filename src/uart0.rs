#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    uart_urdr: UartUrdr,
    uart_urcr: UartUrcr,
    _reserved2: [u8; 0x04],
    uart_urier: UartUrier,
    uart_ursifr: UartUrsifr,
    _reserved4: [u8; 0x10],
    uart_urdlr: UartUrdlr,
    uart_urtstr: UartUrtstr,
}
impl RegisterBlock {
    #[doc = "0x00 - UART_URDR"]
    #[inline(always)]
    pub const fn uart_urdr(&self) -> &UartUrdr {
        &self.uart_urdr
    }
    #[doc = "0x04 - UART_URCR"]
    #[inline(always)]
    pub const fn uart_urcr(&self) -> &UartUrcr {
        &self.uart_urcr
    }
    #[doc = "0x0c - UART_URIER"]
    #[inline(always)]
    pub const fn uart_urier(&self) -> &UartUrier {
        &self.uart_urier
    }
    #[doc = "0x10 - UART_URSIFR"]
    #[inline(always)]
    pub const fn uart_ursifr(&self) -> &UartUrsifr {
        &self.uart_ursifr
    }
    #[doc = "0x24 - UART_URDLR"]
    #[inline(always)]
    pub const fn uart_urdlr(&self) -> &UartUrdlr {
        &self.uart_urdlr
    }
    #[doc = "0x28 - UART_URTSTR"]
    #[inline(always)]
    pub const fn uart_urtstr(&self) -> &UartUrtstr {
        &self.uart_urtstr
    }
}
#[doc = "UART_URDR (rw) register accessor: UART_URDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_urdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_urdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_urdr`]
module"]
#[doc(alias = "UART_URDR")]
pub type UartUrdr = crate::Reg<uart_urdr::UartUrdrSpec>;
#[doc = "UART_URDR"]
pub mod uart_urdr;
#[doc = "UART_URCR (rw) register accessor: UART_URCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_urcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_urcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_urcr`]
module"]
#[doc(alias = "UART_URCR")]
pub type UartUrcr = crate::Reg<uart_urcr::UartUrcrSpec>;
#[doc = "UART_URCR"]
pub mod uart_urcr;
#[doc = "UART_URIER (rw) register accessor: UART_URIER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_urier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_urier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_urier`]
module"]
#[doc(alias = "UART_URIER")]
pub type UartUrier = crate::Reg<uart_urier::UartUrierSpec>;
#[doc = "UART_URIER"]
pub mod uart_urier;
#[doc = "UART_URSIFR (rw) register accessor: UART_URSIFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_ursifr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_ursifr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_ursifr`]
module"]
#[doc(alias = "UART_URSIFR")]
pub type UartUrsifr = crate::Reg<uart_ursifr::UartUrsifrSpec>;
#[doc = "UART_URSIFR"]
pub mod uart_ursifr;
#[doc = "UART_URDLR (rw) register accessor: UART_URDLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_urdlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_urdlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_urdlr`]
module"]
#[doc(alias = "UART_URDLR")]
pub type UartUrdlr = crate::Reg<uart_urdlr::UartUrdlrSpec>;
#[doc = "UART_URDLR"]
pub mod uart_urdlr;
#[doc = "UART_URTSTR (rw) register accessor: UART_URTSTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_urtstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_urtstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_urtstr`]
module"]
#[doc(alias = "UART_URTSTR")]
pub type UartUrtstr = crate::Reg<uart_urtstr::UartUrtstrSpec>;
#[doc = "UART_URTSTR"]
pub mod uart_urtstr;
