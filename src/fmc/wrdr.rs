#[doc = "Register `WRDR` reader"]
pub type R = crate::R<WrdrSpec>;
#[doc = "Register `WRDR` writer"]
pub type W = crate::W<WrdrSpec>;
#[doc = "Field `WRDB` reader - WRDB"]
pub type WrdbR = crate::FieldReader<u32>;
#[doc = "Field `WRDB` writer - WRDB"]
pub type WrdbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WRDB"]
    #[inline(always)]
    pub fn wrdb(&self) -> WrdbR {
        WrdbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WRDB"]
    #[inline(always)]
    #[must_use]
    pub fn wrdb(&mut self) -> WrdbW<WrdrSpec> {
        WrdbW::new(self, 0)
    }
}
#[doc = "WRDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrdrSpec;
impl crate::RegisterSpec for WrdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrdr::R`](R) reader structure"]
impl crate::Readable for WrdrSpec {}
#[doc = "`write(|w| ..)` method takes [`wrdr::W`](W) writer structure"]
impl crate::Writable for WrdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRDR to value 0"]
impl crate::Resettable for WrdrSpec {
    const RESET_VALUE: u32 = 0;
}
