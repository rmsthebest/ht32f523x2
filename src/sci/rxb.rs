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
    pub fn rb(&mut self) -> RbW<'_, RxbSpec> {
        RbW::new(self, 0)
    }
}
#[doc = "RXB\n\nYou can [`read`](crate::Reg::read) this register and get [`rxb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxbSpec;
impl crate::RegisterSpec for RxbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxb::R`](R) reader structure"]
impl crate::Readable for RxbSpec {}
#[doc = "`write(|w| ..)` method takes [`rxb::W`](W) writer structure"]
impl crate::Writable for RxbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXB to value 0"]
impl crate::Resettable for RxbSpec {}
