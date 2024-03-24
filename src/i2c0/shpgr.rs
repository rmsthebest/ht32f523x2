#[doc = "Register `SHPGR` reader"]
pub type R = crate::R<ShpgrSpec>;
#[doc = "Register `SHPGR` writer"]
pub type W = crate::W<ShpgrSpec>;
#[doc = "Field `SHPG` reader - SHPG"]
pub type ShpgR = crate::FieldReader<u16>;
#[doc = "Field `SHPG` writer - SHPG"]
pub type ShpgW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SHPG"]
    #[inline(always)]
    pub fn shpg(&self) -> ShpgR {
        ShpgR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SHPG"]
    #[inline(always)]
    #[must_use]
    pub fn shpg(&mut self) -> ShpgW<ShpgrSpec> {
        ShpgW::new(self, 0)
    }
}
#[doc = "SHPGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shpgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shpgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShpgrSpec;
impl crate::RegisterSpec for ShpgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shpgr::R`](R) reader structure"]
impl crate::Readable for ShpgrSpec {}
#[doc = "`write(|w| ..)` method takes [`shpgr::W`](W) writer structure"]
impl crate::Writable for ShpgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHPGR to value 0"]
impl crate::Resettable for ShpgrSpec {
    const RESET_VALUE: u32 = 0;
}
