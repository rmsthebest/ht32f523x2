#[doc = "Register `GPTM_MDCFR` reader"]
pub type R = crate::R<GptmMdcfrSpec>;
#[doc = "Register `GPTM_MDCFR` writer"]
pub type W = crate::W<GptmMdcfrSpec>;
#[doc = "Field `TSE` reader - TSE"]
pub type TseR = crate::BitReader;
#[doc = "Field `TSE` writer - TSE"]
pub type TseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMSEL` reader - SMSEL"]
pub type SmselR = crate::FieldReader;
#[doc = "Field `SMSEL` writer - SMSEL"]
pub type SmselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MMSEL` reader - MMSEL"]
pub type MmselR = crate::FieldReader;
#[doc = "Field `MMSEL` writer - MMSEL"]
pub type MmselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPMSET` reader - SPMSET"]
pub type SpmsetR = crate::BitReader;
#[doc = "Field `SPMSET` writer - SPMSET"]
pub type SpmsetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TSE"]
    #[inline(always)]
    pub fn tse(&self) -> TseR {
        TseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - SMSEL"]
    #[inline(always)]
    pub fn smsel(&self) -> SmselR {
        SmselR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - MMSEL"]
    #[inline(always)]
    pub fn mmsel(&self) -> MmselR {
        MmselR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - SPMSET"]
    #[inline(always)]
    pub fn spmset(&self) -> SpmsetR {
        SpmsetR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TSE"]
    #[inline(always)]
    pub fn tse(&mut self) -> TseW<'_, GptmMdcfrSpec> {
        TseW::new(self, 0)
    }
    #[doc = "Bits 8:10 - SMSEL"]
    #[inline(always)]
    pub fn smsel(&mut self) -> SmselW<'_, GptmMdcfrSpec> {
        SmselW::new(self, 8)
    }
    #[doc = "Bits 16:18 - MMSEL"]
    #[inline(always)]
    pub fn mmsel(&mut self) -> MmselW<'_, GptmMdcfrSpec> {
        MmselW::new(self, 16)
    }
    #[doc = "Bit 24 - SPMSET"]
    #[inline(always)]
    pub fn spmset(&mut self) -> SpmsetW<'_, GptmMdcfrSpec> {
        SpmsetW::new(self, 24)
    }
}
#[doc = "GPTM_MDCFR\n\nYou can [`read`](crate::Reg::read) this register and get [`gptm_mdcfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gptm_mdcfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptmMdcfrSpec;
impl crate::RegisterSpec for GptmMdcfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptm_mdcfr::R`](R) reader structure"]
impl crate::Readable for GptmMdcfrSpec {}
#[doc = "`write(|w| ..)` method takes [`gptm_mdcfr::W`](W) writer structure"]
impl crate::Writable for GptmMdcfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPTM_MDCFR to value 0"]
impl crate::Resettable for GptmMdcfrSpec {}
