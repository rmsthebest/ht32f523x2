#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `FRES` reader - FRES"]
pub type FresR = crate::BitReader;
#[doc = "Field `FRES` writer - FRES"]
pub type FresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDWN` reader - PDWN"]
pub type PdwnR = crate::BitReader;
#[doc = "Field `PDWN` writer - PDWN"]
pub type PdwnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMODE` reader - LPMODE"]
pub type LpmodeR = crate::BitReader;
#[doc = "Field `LPMODE` writer - LPMODE"]
pub type LpmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GENRSM` reader - GENRSM"]
pub type GenrsmR = crate::BitReader;
#[doc = "Field `GENRSM` writer - GENRSM"]
pub type GenrsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDP` reader - RXDP"]
pub type RxdpR = crate::BitReader;
#[doc = "Field `RXDP` writer - RXDP"]
pub type RxdpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDM` reader - RXDM"]
pub type RxdmR = crate::BitReader;
#[doc = "Field `RXDM` writer - RXDM"]
pub type RxdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADRSET` reader - ADRSET"]
pub type AdrsetR = crate::BitReader;
#[doc = "Field `ADRSET` writer - ADRSET"]
pub type AdrsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMRSTC` reader - SRAMRSTC"]
pub type SramrstcR = crate::BitReader;
#[doc = "Field `SRAMRSTC` writer - SRAMRSTC"]
pub type SramrstcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPPUEN` reader - DPPUEN"]
pub type DppuenR = crate::BitReader;
#[doc = "Field `DPPUEN` writer - DPPUEN"]
pub type DppuenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPWKEN` reader - DPWKEN"]
pub type DpwkenR = crate::BitReader;
#[doc = "Field `DPWKEN` writer - DPWKEN"]
pub type DpwkenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - FRES"]
    #[inline(always)]
    pub fn fres(&self) -> FresR {
        FresR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDWN"]
    #[inline(always)]
    pub fn pdwn(&self) -> PdwnR {
        PdwnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LPMODE"]
    #[inline(always)]
    pub fn lpmode(&self) -> LpmodeR {
        LpmodeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - GENRSM"]
    #[inline(always)]
    pub fn genrsm(&self) -> GenrsmR {
        GenrsmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RXDP"]
    #[inline(always)]
    pub fn rxdp(&self) -> RxdpR {
        RxdpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RXDM"]
    #[inline(always)]
    pub fn rxdm(&self) -> RxdmR {
        RxdmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADRSET"]
    #[inline(always)]
    pub fn adrset(&self) -> AdrsetR {
        AdrsetR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAMRSTC"]
    #[inline(always)]
    pub fn sramrstc(&self) -> SramrstcR {
        SramrstcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DPPUEN"]
    #[inline(always)]
    pub fn dppuen(&self) -> DppuenR {
        DppuenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DPWKEN"]
    #[inline(always)]
    pub fn dpwken(&self) -> DpwkenR {
        DpwkenR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FRES"]
    #[inline(always)]
    pub fn fres(&mut self) -> FresW<'_, CsrSpec> {
        FresW::new(self, 1)
    }
    #[doc = "Bit 2 - PDWN"]
    #[inline(always)]
    pub fn pdwn(&mut self) -> PdwnW<'_, CsrSpec> {
        PdwnW::new(self, 2)
    }
    #[doc = "Bit 3 - LPMODE"]
    #[inline(always)]
    pub fn lpmode(&mut self) -> LpmodeW<'_, CsrSpec> {
        LpmodeW::new(self, 3)
    }
    #[doc = "Bit 5 - GENRSM"]
    #[inline(always)]
    pub fn genrsm(&mut self) -> GenrsmW<'_, CsrSpec> {
        GenrsmW::new(self, 5)
    }
    #[doc = "Bit 6 - RXDP"]
    #[inline(always)]
    pub fn rxdp(&mut self) -> RxdpW<'_, CsrSpec> {
        RxdpW::new(self, 6)
    }
    #[doc = "Bit 7 - RXDM"]
    #[inline(always)]
    pub fn rxdm(&mut self) -> RxdmW<'_, CsrSpec> {
        RxdmW::new(self, 7)
    }
    #[doc = "Bit 8 - ADRSET"]
    #[inline(always)]
    pub fn adrset(&mut self) -> AdrsetW<'_, CsrSpec> {
        AdrsetW::new(self, 8)
    }
    #[doc = "Bit 9 - SRAMRSTC"]
    #[inline(always)]
    pub fn sramrstc(&mut self) -> SramrstcW<'_, CsrSpec> {
        SramrstcW::new(self, 9)
    }
    #[doc = "Bit 10 - DPPUEN"]
    #[inline(always)]
    pub fn dppuen(&mut self) -> DppuenW<'_, CsrSpec> {
        DppuenW::new(self, 10)
    }
    #[doc = "Bit 11 - DPWKEN"]
    #[inline(always)]
    pub fn dpwken(&mut self) -> DpwkenW<'_, CsrSpec> {
        DpwkenW::new(self, 11)
    }
}
#[doc = "CSR\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {}
