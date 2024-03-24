#[doc = "Register `SLPGR` reader"]
pub type R = crate::R<SlpgrSpec>;
#[doc = "Register `SLPGR` writer"]
pub type W = crate::W<SlpgrSpec>;
#[doc = "Field `SLPG` reader - SLPG"]
pub type SlpgR = crate::FieldReader<u16>;
#[doc = "Field `SLPG` writer - SLPG"]
pub type SlpgW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SLPG"]
    #[inline(always)]
    pub fn slpg(&self) -> SlpgR {
        SlpgR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SLPG"]
    #[inline(always)]
    #[must_use]
    pub fn slpg(&mut self) -> SlpgW<SlpgrSpec> {
        SlpgW::new(self, 0)
    }
}
#[doc = "SLPGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slpgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slpgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlpgrSpec;
impl crate::RegisterSpec for SlpgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slpgr::R`](R) reader structure"]
impl crate::Readable for SlpgrSpec {}
#[doc = "`write(|w| ..)` method takes [`slpgr::W`](W) writer structure"]
impl crate::Writable for SlpgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLPGR to value 0"]
impl crate::Resettable for SlpgrSpec {
    const RESET_VALUE: u32 = 0;
}
