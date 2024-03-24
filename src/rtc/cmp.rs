#[doc = "Register `CMP` reader"]
pub type R = crate::R<CmpSpec>;
#[doc = "Register `CMP` writer"]
pub type W = crate::W<CmpSpec>;
#[doc = "Field `RTCCMP` reader - RTCCMP"]
pub type RtccmpR = crate::FieldReader<u32>;
#[doc = "Field `RTCCMP` writer - RTCCMP"]
pub type RtccmpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RTCCMP"]
    #[inline(always)]
    pub fn rtccmp(&self) -> RtccmpR {
        RtccmpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RTCCMP"]
    #[inline(always)]
    #[must_use]
    pub fn rtccmp(&mut self) -> RtccmpW<CmpSpec> {
        RtccmpW::new(self, 0)
    }
}
#[doc = "CMP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpSpec;
impl crate::RegisterSpec for CmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp::R`](R) reader structure"]
impl crate::Readable for CmpSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp::W`](W) writer structure"]
impl crate::Writable for CmpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP to value 0"]
impl crate::Resettable for CmpSpec {
    const RESET_VALUE: u32 = 0;
}
