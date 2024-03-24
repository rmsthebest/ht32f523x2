#[doc = "Register `CCR` reader"]
pub type R = crate::R<CcrSpec>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CcrSpec>;
#[doc = "Field `CCLK` reader - CCLK"]
pub type CclkR = crate::BitReader;
#[doc = "Field `CCLK` writer - CCLK"]
pub type CclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDIO` reader - CDIO"]
pub type CdioR = crate::BitReader;
#[doc = "Field `CDIO` writer - CDIO"]
pub type CdioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSEL` reader - CLKSEL"]
pub type ClkselR = crate::BitReader;
#[doc = "Field `CLKSEL` writer - CLKSEL"]
pub type ClkselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - CCLK"]
    #[inline(always)]
    pub fn cclk(&self) -> CclkR {
        CclkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CDIO"]
    #[inline(always)]
    pub fn cdio(&self) -> CdioR {
        CdioR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - CLKSEL"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - CCLK"]
    #[inline(always)]
    #[must_use]
    pub fn cclk(&mut self) -> CclkW<CcrSpec> {
        CclkW::new(self, 2)
    }
    #[doc = "Bit 3 - CDIO"]
    #[inline(always)]
    #[must_use]
    pub fn cdio(&mut self) -> CdioW<CcrSpec> {
        CdioW::new(self, 3)
    }
    #[doc = "Bit 7 - CLKSEL"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> ClkselW<CcrSpec> {
        ClkselW::new(self, 7)
    }
}
#[doc = "CCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcrSpec;
impl crate::RegisterSpec for CcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CcrSpec {
    const RESET_VALUE: u32 = 0;
}
