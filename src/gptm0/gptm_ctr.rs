#[doc = "Register `GPTM_CTR` reader"]
pub type R = crate::R<GptmCtrSpec>;
#[doc = "Register `GPTM_CTR` writer"]
pub type W = crate::W<GptmCtrSpec>;
#[doc = "Field `TME` reader - TME"]
pub type TmeR = crate::BitReader;
#[doc = "Field `TME` writer - TME"]
pub type TmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRBE` reader - CRBE"]
pub type CrbeR = crate::BitReader;
#[doc = "Field `CRBE` writer - CRBE"]
pub type CrbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHCCDS` reader - CHCCDS"]
pub type ChccdsR = crate::BitReader;
#[doc = "Field `CHCCDS` writer - CHCCDS"]
pub type ChccdsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TME"]
    #[inline(always)]
    pub fn tme(&self) -> TmeR {
        TmeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CRBE"]
    #[inline(always)]
    pub fn crbe(&self) -> CrbeR {
        CrbeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - CHCCDS"]
    #[inline(always)]
    pub fn chccds(&self) -> ChccdsR {
        ChccdsR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TME"]
    #[inline(always)]
    #[must_use]
    pub fn tme(&mut self) -> TmeW<GptmCtrSpec> {
        TmeW::new(self, 0)
    }
    #[doc = "Bit 1 - CRBE"]
    #[inline(always)]
    #[must_use]
    pub fn crbe(&mut self) -> CrbeW<GptmCtrSpec> {
        CrbeW::new(self, 1)
    }
    #[doc = "Bit 16 - CHCCDS"]
    #[inline(always)]
    #[must_use]
    pub fn chccds(&mut self) -> ChccdsW<GptmCtrSpec> {
        ChccdsW::new(self, 16)
    }
}
#[doc = "GPTM_CTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptm_ctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptm_ctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptmCtrSpec;
impl crate::RegisterSpec for GptmCtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptm_ctr::R`](R) reader structure"]
impl crate::Readable for GptmCtrSpec {}
#[doc = "`write(|w| ..)` method takes [`gptm_ctr::W`](W) writer structure"]
impl crate::Writable for GptmCtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPTM_CTR to value 0"]
impl crate::Resettable for GptmCtrSpec {
    const RESET_VALUE: u32 = 0;
}
