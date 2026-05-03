#[doc = "Register `GPTM_CNTCFR` reader"]
pub type R = crate::R<GptmCntcfrSpec>;
#[doc = "Register `GPTM_CNTCFR` writer"]
pub type W = crate::W<GptmCntcfrSpec>;
#[doc = "Field `UEVDIS` reader - UEVDIS"]
pub type UevdisR = crate::BitReader;
#[doc = "Field `UEVDIS` writer - UEVDIS"]
pub type UevdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UGDIS` reader - UGDIS"]
pub type UgdisR = crate::BitReader;
#[doc = "Field `UGDIS` writer - UGDIS"]
pub type UgdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKDIV` reader - CKDIV"]
pub type CkdivR = crate::FieldReader;
#[doc = "Field `CKDIV` writer - CKDIV"]
pub type CkdivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMSEL` reader - CMSEL"]
pub type CmselR = crate::FieldReader;
#[doc = "Field `CMSEL` writer - CMSEL"]
pub type CmselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DIR` reader - DIR"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - DIR"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UEVDIS"]
    #[inline(always)]
    pub fn uevdis(&self) -> UevdisR {
        UevdisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UGDIS"]
    #[inline(always)]
    pub fn ugdis(&self) -> UgdisR {
        UgdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - CKDIV"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CkdivR {
        CkdivR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - CMSEL"]
    #[inline(always)]
    pub fn cmsel(&self) -> CmselR {
        CmselR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UEVDIS"]
    #[inline(always)]
    pub fn uevdis(&mut self) -> UevdisW<'_, GptmCntcfrSpec> {
        UevdisW::new(self, 0)
    }
    #[doc = "Bit 1 - UGDIS"]
    #[inline(always)]
    pub fn ugdis(&mut self) -> UgdisW<'_, GptmCntcfrSpec> {
        UgdisW::new(self, 1)
    }
    #[doc = "Bits 8:9 - CKDIV"]
    #[inline(always)]
    pub fn ckdiv(&mut self) -> CkdivW<'_, GptmCntcfrSpec> {
        CkdivW::new(self, 8)
    }
    #[doc = "Bits 16:17 - CMSEL"]
    #[inline(always)]
    pub fn cmsel(&mut self) -> CmselW<'_, GptmCntcfrSpec> {
        CmselW::new(self, 16)
    }
    #[doc = "Bit 24 - DIR"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<'_, GptmCntcfrSpec> {
        DirW::new(self, 24)
    }
}
#[doc = "GPTM_CNTCFR\n\nYou can [`read`](crate::Reg::read) this register and get [`gptm_cntcfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gptm_cntcfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GptmCntcfrSpec;
impl crate::RegisterSpec for GptmCntcfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptm_cntcfr::R`](R) reader structure"]
impl crate::Readable for GptmCntcfrSpec {}
#[doc = "`write(|w| ..)` method takes [`gptm_cntcfr::W`](W) writer structure"]
impl crate::Writable for GptmCntcfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPTM_CNTCFR to value 0"]
impl crate::Resettable for GptmCntcfrSpec {}
