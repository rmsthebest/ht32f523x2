#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csr: Csr,
    ier: Ier,
    isr: Isr,
    fcr: Fcr,
    devar: Devar,
    ep0csr: Ep0csr,
    ep0ier: Ep0ier,
    ep0isr: Ep0isr,
    ep0tcr: Ep0tcr,
    ep0cfgr: Ep0cfgr,
    ep1csr: Ep1csr,
    ep1ier: Ep1ier,
    ep1isr: Ep1isr,
    ep1tcr: Ep1tcr,
    ep1cfgr: Ep1cfgr,
    ep2csr: Ep2csr,
    ep2ier: Ep2ier,
    ep2isr: Ep2isr,
    ep2tcr: Ep2tcr,
    ep2cfgr: Ep2cfgr,
    ep3csr: Ep3csr,
    ep3ier: Ep3ier,
    ep3isr: Ep3isr,
    ep3tcr: Ep3tcr,
    ep3cfgr: Ep3cfgr,
    ep4csr: Ep4csr,
    ep4ier: Ep4ier,
    ep4isr: Ep4isr,
    ep4tcr: Ep4tcr,
    ep4cfgr: Ep4cfgr,
    ep5csr: Ep5csr,
    ep5ier: Ep5ier,
    ep5isr: Ep5isr,
    ep5tcr: Ep5tcr,
    ep5cfgr: Ep5cfgr,
    ep6csr: Ep6csr,
    ep6ier: Ep6ier,
    ep6isr: Ep6isr,
    ep6tcr: Ep6tcr,
    ep6cfgr: Ep6cfgr,
    ep7csr: Ep7csr,
    ep7ier: Ep7ier,
    ep7isr: Ep7isr,
    ep7tcr: Ep7tcr,
    ep7cfgr: Ep7cfgr,
}
impl RegisterBlock {
    #[doc = "0x00 - CSR"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x04 - IER"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x08 - ISR"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x0c - FCR"]
    #[inline(always)]
    pub const fn fcr(&self) -> &Fcr {
        &self.fcr
    }
    #[doc = "0x10 - DEVAR"]
    #[inline(always)]
    pub const fn devar(&self) -> &Devar {
        &self.devar
    }
    #[doc = "0x14 - EP0CSR"]
    #[inline(always)]
    pub const fn ep0csr(&self) -> &Ep0csr {
        &self.ep0csr
    }
    #[doc = "0x18 - EP0IER"]
    #[inline(always)]
    pub const fn ep0ier(&self) -> &Ep0ier {
        &self.ep0ier
    }
    #[doc = "0x1c - EP0ISR"]
    #[inline(always)]
    pub const fn ep0isr(&self) -> &Ep0isr {
        &self.ep0isr
    }
    #[doc = "0x20 - EP0TCR"]
    #[inline(always)]
    pub const fn ep0tcr(&self) -> &Ep0tcr {
        &self.ep0tcr
    }
    #[doc = "0x24 - EP0CFGR"]
    #[inline(always)]
    pub const fn ep0cfgr(&self) -> &Ep0cfgr {
        &self.ep0cfgr
    }
    #[doc = "0x28 - EP1CSR"]
    #[inline(always)]
    pub const fn ep1csr(&self) -> &Ep1csr {
        &self.ep1csr
    }
    #[doc = "0x2c - EP1IER"]
    #[inline(always)]
    pub const fn ep1ier(&self) -> &Ep1ier {
        &self.ep1ier
    }
    #[doc = "0x30 - EP1ISR"]
    #[inline(always)]
    pub const fn ep1isr(&self) -> &Ep1isr {
        &self.ep1isr
    }
    #[doc = "0x34 - EP1TCR"]
    #[inline(always)]
    pub const fn ep1tcr(&self) -> &Ep1tcr {
        &self.ep1tcr
    }
    #[doc = "0x38 - EP1CFGR"]
    #[inline(always)]
    pub const fn ep1cfgr(&self) -> &Ep1cfgr {
        &self.ep1cfgr
    }
    #[doc = "0x3c - EP2CSR"]
    #[inline(always)]
    pub const fn ep2csr(&self) -> &Ep2csr {
        &self.ep2csr
    }
    #[doc = "0x40 - EP2IER"]
    #[inline(always)]
    pub const fn ep2ier(&self) -> &Ep2ier {
        &self.ep2ier
    }
    #[doc = "0x44 - EP2ISR"]
    #[inline(always)]
    pub const fn ep2isr(&self) -> &Ep2isr {
        &self.ep2isr
    }
    #[doc = "0x48 - EP2TCR"]
    #[inline(always)]
    pub const fn ep2tcr(&self) -> &Ep2tcr {
        &self.ep2tcr
    }
    #[doc = "0x4c - EP2CFGR"]
    #[inline(always)]
    pub const fn ep2cfgr(&self) -> &Ep2cfgr {
        &self.ep2cfgr
    }
    #[doc = "0x50 - EP3CSR"]
    #[inline(always)]
    pub const fn ep3csr(&self) -> &Ep3csr {
        &self.ep3csr
    }
    #[doc = "0x54 - EP3IER"]
    #[inline(always)]
    pub const fn ep3ier(&self) -> &Ep3ier {
        &self.ep3ier
    }
    #[doc = "0x58 - EP3ISR"]
    #[inline(always)]
    pub const fn ep3isr(&self) -> &Ep3isr {
        &self.ep3isr
    }
    #[doc = "0x5c - EP3TCR"]
    #[inline(always)]
    pub const fn ep3tcr(&self) -> &Ep3tcr {
        &self.ep3tcr
    }
    #[doc = "0x60 - EP3CFGR"]
    #[inline(always)]
    pub const fn ep3cfgr(&self) -> &Ep3cfgr {
        &self.ep3cfgr
    }
    #[doc = "0x64 - EP4CSR"]
    #[inline(always)]
    pub const fn ep4csr(&self) -> &Ep4csr {
        &self.ep4csr
    }
    #[doc = "0x68 - EP4IER"]
    #[inline(always)]
    pub const fn ep4ier(&self) -> &Ep4ier {
        &self.ep4ier
    }
    #[doc = "0x6c - EP4ISR"]
    #[inline(always)]
    pub const fn ep4isr(&self) -> &Ep4isr {
        &self.ep4isr
    }
    #[doc = "0x70 - EP4TCR"]
    #[inline(always)]
    pub const fn ep4tcr(&self) -> &Ep4tcr {
        &self.ep4tcr
    }
    #[doc = "0x74 - EP4CFGR"]
    #[inline(always)]
    pub const fn ep4cfgr(&self) -> &Ep4cfgr {
        &self.ep4cfgr
    }
    #[doc = "0x78 - EP5CSR"]
    #[inline(always)]
    pub const fn ep5csr(&self) -> &Ep5csr {
        &self.ep5csr
    }
    #[doc = "0x7c - EP5IER"]
    #[inline(always)]
    pub const fn ep5ier(&self) -> &Ep5ier {
        &self.ep5ier
    }
    #[doc = "0x80 - EP5ISR"]
    #[inline(always)]
    pub const fn ep5isr(&self) -> &Ep5isr {
        &self.ep5isr
    }
    #[doc = "0x84 - EP5TCR"]
    #[inline(always)]
    pub const fn ep5tcr(&self) -> &Ep5tcr {
        &self.ep5tcr
    }
    #[doc = "0x88 - EP5CFGR"]
    #[inline(always)]
    pub const fn ep5cfgr(&self) -> &Ep5cfgr {
        &self.ep5cfgr
    }
    #[doc = "0x8c - EP6CSR"]
    #[inline(always)]
    pub const fn ep6csr(&self) -> &Ep6csr {
        &self.ep6csr
    }
    #[doc = "0x90 - EP6IER"]
    #[inline(always)]
    pub const fn ep6ier(&self) -> &Ep6ier {
        &self.ep6ier
    }
    #[doc = "0x94 - EP6ISR"]
    #[inline(always)]
    pub const fn ep6isr(&self) -> &Ep6isr {
        &self.ep6isr
    }
    #[doc = "0x98 - EP6TCR"]
    #[inline(always)]
    pub const fn ep6tcr(&self) -> &Ep6tcr {
        &self.ep6tcr
    }
    #[doc = "0x9c - EP6CFGR"]
    #[inline(always)]
    pub const fn ep6cfgr(&self) -> &Ep6cfgr {
        &self.ep6cfgr
    }
    #[doc = "0xa0 - EP7CSR"]
    #[inline(always)]
    pub const fn ep7csr(&self) -> &Ep7csr {
        &self.ep7csr
    }
    #[doc = "0xa4 - EP7IER"]
    #[inline(always)]
    pub const fn ep7ier(&self) -> &Ep7ier {
        &self.ep7ier
    }
    #[doc = "0xa8 - EP7ISR"]
    #[inline(always)]
    pub const fn ep7isr(&self) -> &Ep7isr {
        &self.ep7isr
    }
    #[doc = "0xac - EP7TCR"]
    #[inline(always)]
    pub const fn ep7tcr(&self) -> &Ep7tcr {
        &self.ep7tcr
    }
    #[doc = "0xb0 - EP7CFGR"]
    #[inline(always)]
    pub const fn ep7cfgr(&self) -> &Ep7cfgr {
        &self.ep7cfgr
    }
}
#[doc = "CSR (rw) register accessor: CSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "CSR"]
pub mod csr;
#[doc = "IER (rw) register accessor: IER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "IER"]
pub mod ier;
#[doc = "ISR (rw) register accessor: ISR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "ISR"]
pub mod isr;
#[doc = "FCR (rw) register accessor: FCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
#[doc(alias = "FCR")]
pub type Fcr = crate::Reg<fcr::FcrSpec>;
#[doc = "FCR"]
pub mod fcr;
#[doc = "DEVAR (rw) register accessor: DEVAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devar`]
module"]
#[doc(alias = "DEVAR")]
pub type Devar = crate::Reg<devar::DevarSpec>;
#[doc = "DEVAR"]
pub mod devar;
#[doc = "EP0CSR (rw) register accessor: EP0CSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0csr`]
module"]
#[doc(alias = "EP0CSR")]
pub type Ep0csr = crate::Reg<ep0csr::Ep0csrSpec>;
#[doc = "EP0CSR"]
pub mod ep0csr;
#[doc = "EP0IER (rw) register accessor: EP0IER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0ier`]
module"]
#[doc(alias = "EP0IER")]
pub type Ep0ier = crate::Reg<ep0ier::Ep0ierSpec>;
#[doc = "EP0IER"]
pub mod ep0ier;
#[doc = "EP0ISR (rw) register accessor: EP0ISR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0isr`]
module"]
#[doc(alias = "EP0ISR")]
pub type Ep0isr = crate::Reg<ep0isr::Ep0isrSpec>;
#[doc = "EP0ISR"]
pub mod ep0isr;
#[doc = "EP0TCR (rw) register accessor: EP0TCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0tcr`]
module"]
#[doc(alias = "EP0TCR")]
pub type Ep0tcr = crate::Reg<ep0tcr::Ep0tcrSpec>;
#[doc = "EP0TCR"]
pub mod ep0tcr;
#[doc = "EP0CFGR (rw) register accessor: EP0CFGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0cfgr`]
module"]
#[doc(alias = "EP0CFGR")]
pub type Ep0cfgr = crate::Reg<ep0cfgr::Ep0cfgrSpec>;
#[doc = "EP0CFGR"]
pub mod ep0cfgr;
#[doc = "EP1CSR (rw) register accessor: EP1CSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep1csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep1csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep1csr`]
module"]
#[doc(alias = "EP1CSR")]
pub type Ep1csr = crate::Reg<ep1csr::Ep1csrSpec>;
#[doc = "EP1CSR"]
pub mod ep1csr;
#[doc = "EP1IER (rw) register accessor: EP1IER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep1ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep1ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep1ier`]
module"]
#[doc(alias = "EP1IER")]
pub type Ep1ier = crate::Reg<ep1ier::Ep1ierSpec>;
#[doc = "EP1IER"]
pub mod ep1ier;
#[doc = "EP1ISR (rw) register accessor: EP1ISR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep1isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep1isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep1isr`]
module"]
#[doc(alias = "EP1ISR")]
pub type Ep1isr = crate::Reg<ep1isr::Ep1isrSpec>;
#[doc = "EP1ISR"]
pub mod ep1isr;
#[doc = "EP1TCR (rw) register accessor: EP1TCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep1tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep1tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep1tcr`]
module"]
#[doc(alias = "EP1TCR")]
pub type Ep1tcr = crate::Reg<ep1tcr::Ep1tcrSpec>;
#[doc = "EP1TCR"]
pub mod ep1tcr;
#[doc = "EP1CFGR (rw) register accessor: EP1CFGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep1cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep1cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep1cfgr`]
module"]
#[doc(alias = "EP1CFGR")]
pub type Ep1cfgr = crate::Reg<ep1cfgr::Ep1cfgrSpec>;
#[doc = "EP1CFGR"]
pub mod ep1cfgr;
#[doc = "EP2CSR (rw) register accessor: EP2CSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep2csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep2csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep2csr`]
module"]
#[doc(alias = "EP2CSR")]
pub type Ep2csr = crate::Reg<ep2csr::Ep2csrSpec>;
#[doc = "EP2CSR"]
pub mod ep2csr;
#[doc = "EP2IER (rw) register accessor: EP2IER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep2ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep2ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep2ier`]
module"]
#[doc(alias = "EP2IER")]
pub type Ep2ier = crate::Reg<ep2ier::Ep2ierSpec>;
#[doc = "EP2IER"]
pub mod ep2ier;
#[doc = "EP2ISR (rw) register accessor: EP2ISR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep2isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep2isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep2isr`]
module"]
#[doc(alias = "EP2ISR")]
pub type Ep2isr = crate::Reg<ep2isr::Ep2isrSpec>;
#[doc = "EP2ISR"]
pub mod ep2isr;
#[doc = "EP2TCR (rw) register accessor: EP2TCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep2tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep2tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep2tcr`]
module"]
#[doc(alias = "EP2TCR")]
pub type Ep2tcr = crate::Reg<ep2tcr::Ep2tcrSpec>;
#[doc = "EP2TCR"]
pub mod ep2tcr;
#[doc = "EP2CFGR (rw) register accessor: EP2CFGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep2cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep2cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep2cfgr`]
module"]
#[doc(alias = "EP2CFGR")]
pub type Ep2cfgr = crate::Reg<ep2cfgr::Ep2cfgrSpec>;
#[doc = "EP2CFGR"]
pub mod ep2cfgr;
#[doc = "EP3CSR (rw) register accessor: EP3CSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep3csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep3csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep3csr`]
module"]
#[doc(alias = "EP3CSR")]
pub type Ep3csr = crate::Reg<ep3csr::Ep3csrSpec>;
#[doc = "EP3CSR"]
pub mod ep3csr;
#[doc = "EP3IER (rw) register accessor: EP3IER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep3ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep3ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep3ier`]
module"]
#[doc(alias = "EP3IER")]
pub type Ep3ier = crate::Reg<ep3ier::Ep3ierSpec>;
#[doc = "EP3IER"]
pub mod ep3ier;
#[doc = "EP3ISR (rw) register accessor: EP3ISR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep3isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep3isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep3isr`]
module"]
#[doc(alias = "EP3ISR")]
pub type Ep3isr = crate::Reg<ep3isr::Ep3isrSpec>;
#[doc = "EP3ISR"]
pub mod ep3isr;
#[doc = "EP3TCR (rw) register accessor: EP3TCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep3tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep3tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep3tcr`]
module"]
#[doc(alias = "EP3TCR")]
pub type Ep3tcr = crate::Reg<ep3tcr::Ep3tcrSpec>;
#[doc = "EP3TCR"]
pub mod ep3tcr;
#[doc = "EP3CFGR (rw) register accessor: EP3CFGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep3cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep3cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep3cfgr`]
module"]
#[doc(alias = "EP3CFGR")]
pub type Ep3cfgr = crate::Reg<ep3cfgr::Ep3cfgrSpec>;
#[doc = "EP3CFGR"]
pub mod ep3cfgr;
#[doc = "EP4CSR (rw) register accessor: EP4CSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep4csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep4csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep4csr`]
module"]
#[doc(alias = "EP4CSR")]
pub type Ep4csr = crate::Reg<ep4csr::Ep4csrSpec>;
#[doc = "EP4CSR"]
pub mod ep4csr;
#[doc = "EP4IER (rw) register accessor: EP4IER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep4ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep4ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep4ier`]
module"]
#[doc(alias = "EP4IER")]
pub type Ep4ier = crate::Reg<ep4ier::Ep4ierSpec>;
#[doc = "EP4IER"]
pub mod ep4ier;
#[doc = "EP4ISR (rw) register accessor: EP4ISR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep4isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep4isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep4isr`]
module"]
#[doc(alias = "EP4ISR")]
pub type Ep4isr = crate::Reg<ep4isr::Ep4isrSpec>;
#[doc = "EP4ISR"]
pub mod ep4isr;
#[doc = "EP4TCR (rw) register accessor: EP4TCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep4tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep4tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep4tcr`]
module"]
#[doc(alias = "EP4TCR")]
pub type Ep4tcr = crate::Reg<ep4tcr::Ep4tcrSpec>;
#[doc = "EP4TCR"]
pub mod ep4tcr;
#[doc = "EP4CFGR (rw) register accessor: EP4CFGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep4cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep4cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep4cfgr`]
module"]
#[doc(alias = "EP4CFGR")]
pub type Ep4cfgr = crate::Reg<ep4cfgr::Ep4cfgrSpec>;
#[doc = "EP4CFGR"]
pub mod ep4cfgr;
#[doc = "EP5CSR (rw) register accessor: EP5CSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep5csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep5csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep5csr`]
module"]
#[doc(alias = "EP5CSR")]
pub type Ep5csr = crate::Reg<ep5csr::Ep5csrSpec>;
#[doc = "EP5CSR"]
pub mod ep5csr;
#[doc = "EP5IER (rw) register accessor: EP5IER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep5ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep5ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep5ier`]
module"]
#[doc(alias = "EP5IER")]
pub type Ep5ier = crate::Reg<ep5ier::Ep5ierSpec>;
#[doc = "EP5IER"]
pub mod ep5ier;
#[doc = "EP5ISR (rw) register accessor: EP5ISR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep5isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep5isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep5isr`]
module"]
#[doc(alias = "EP5ISR")]
pub type Ep5isr = crate::Reg<ep5isr::Ep5isrSpec>;
#[doc = "EP5ISR"]
pub mod ep5isr;
#[doc = "EP5TCR (rw) register accessor: EP5TCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep5tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep5tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep5tcr`]
module"]
#[doc(alias = "EP5TCR")]
pub type Ep5tcr = crate::Reg<ep5tcr::Ep5tcrSpec>;
#[doc = "EP5TCR"]
pub mod ep5tcr;
#[doc = "EP5CFGR (rw) register accessor: EP5CFGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep5cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep5cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep5cfgr`]
module"]
#[doc(alias = "EP5CFGR")]
pub type Ep5cfgr = crate::Reg<ep5cfgr::Ep5cfgrSpec>;
#[doc = "EP5CFGR"]
pub mod ep5cfgr;
#[doc = "EP6CSR (rw) register accessor: EP6CSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep6csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep6csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep6csr`]
module"]
#[doc(alias = "EP6CSR")]
pub type Ep6csr = crate::Reg<ep6csr::Ep6csrSpec>;
#[doc = "EP6CSR"]
pub mod ep6csr;
#[doc = "EP6IER (rw) register accessor: EP6IER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep6ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep6ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep6ier`]
module"]
#[doc(alias = "EP6IER")]
pub type Ep6ier = crate::Reg<ep6ier::Ep6ierSpec>;
#[doc = "EP6IER"]
pub mod ep6ier;
#[doc = "EP6ISR (rw) register accessor: EP6ISR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep6isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep6isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep6isr`]
module"]
#[doc(alias = "EP6ISR")]
pub type Ep6isr = crate::Reg<ep6isr::Ep6isrSpec>;
#[doc = "EP6ISR"]
pub mod ep6isr;
#[doc = "EP6TCR (rw) register accessor: EP6TCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep6tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep6tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep6tcr`]
module"]
#[doc(alias = "EP6TCR")]
pub type Ep6tcr = crate::Reg<ep6tcr::Ep6tcrSpec>;
#[doc = "EP6TCR"]
pub mod ep6tcr;
#[doc = "EP6CFGR (rw) register accessor: EP6CFGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep6cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep6cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep6cfgr`]
module"]
#[doc(alias = "EP6CFGR")]
pub type Ep6cfgr = crate::Reg<ep6cfgr::Ep6cfgrSpec>;
#[doc = "EP6CFGR"]
pub mod ep6cfgr;
#[doc = "EP7CSR (rw) register accessor: EP7CSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep7csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep7csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep7csr`]
module"]
#[doc(alias = "EP7CSR")]
pub type Ep7csr = crate::Reg<ep7csr::Ep7csrSpec>;
#[doc = "EP7CSR"]
pub mod ep7csr;
#[doc = "EP7IER (rw) register accessor: EP7IER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep7ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep7ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep7ier`]
module"]
#[doc(alias = "EP7IER")]
pub type Ep7ier = crate::Reg<ep7ier::Ep7ierSpec>;
#[doc = "EP7IER"]
pub mod ep7ier;
#[doc = "EP7ISR (rw) register accessor: EP7ISR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep7isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep7isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep7isr`]
module"]
#[doc(alias = "EP7ISR")]
pub type Ep7isr = crate::Reg<ep7isr::Ep7isrSpec>;
#[doc = "EP7ISR"]
pub mod ep7isr;
#[doc = "EP7TCR (rw) register accessor: EP7TCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep7tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep7tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep7tcr`]
module"]
#[doc(alias = "EP7TCR")]
pub type Ep7tcr = crate::Reg<ep7tcr::Ep7tcrSpec>;
#[doc = "EP7TCR"]
pub mod ep7tcr;
#[doc = "EP7CFGR (rw) register accessor: EP7CFGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep7cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep7cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep7cfgr`]
module"]
#[doc(alias = "EP7CFGR")]
pub type Ep7cfgr = crate::Reg<ep7cfgr::Ep7cfgrSpec>;
#[doc = "EP7CFGR"]
pub mod ep7cfgr;
