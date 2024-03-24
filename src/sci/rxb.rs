#[doc = "Register `RXB` reader"]
pub type R = crate::R<RxbSpec>;
#[doc = "Register `RXB` writer"]
pub type W = crate::W<RxbSpec>;
#[doc = "Field `RB` reader - RB"]
pub type RbR = crate::FieldReader;
#[doc = "Field `RB` writer - RB"]
pub type RbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RB"]
    #[inline(always)]
    pub fn rb(&self) -> RbR {
        RbR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RB"]
    #[inline(always)]
    #[must_use]
    pub fn rb(&mut self) -> RbW<RxbSpec> {
        RbW::new(self, 0)
    }
}
#[doc = "RXB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxbSpec;
impl crate::RegisterSpec for RxbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxb::R`](R) reader structure"]
impl crate::Readable for RxbSpec {}
#[doc = "`write(|w| ..)` method takes [`rxb::W`](W) writer structure"]
impl crate::Writable for RxbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXB to value 0"]
impl crate::Resettable for RxbSpec {
    const RESET_VALUE: u32 = 0;
}
