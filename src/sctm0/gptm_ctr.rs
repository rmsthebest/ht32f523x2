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
    pub fn tme(&mut self) -> TmeW<'_, GptmCtrSpec> {
        TmeW::new(self, 0)
    }
    #[doc = "Bit 1 - CRBE"]
    #[inline(always)]
    pub fn crbe(&mut self) -> CrbeW<'_, GptmCtrSpec> {
        CrbeW::new(self, 1)
    }
    #[doc = "Bit 16 - CHCCDS"]
    #[inline(always)]
    pub fn chccds(&mut self) -> ChccdsW<'_, GptmCtrSpec> {
        ChccdsW::new(self, 16)
    }
}
#[doc = "GPTM_CTR\n\nYou can [`read`](crate::Reg::read) this register and get [`gptm_ctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gptm_ctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptmCtrSpec;
impl crate::RegisterSpec for GptmCtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptm_ctr::R`](R) reader structure"]
impl crate::Readable for GptmCtrSpec {}
#[doc = "`write(|w| ..)` method takes [`gptm_ctr::W`](W) writer structure"]
impl crate::Writable for GptmCtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPTM_CTR to value 0"]
impl crate::Resettable for GptmCtrSpec {}
