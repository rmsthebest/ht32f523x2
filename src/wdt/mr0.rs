#[doc = "Register `MR0` reader"]
pub type R = crate::R<Mr0Spec>;
#[doc = "Register `MR0` writer"]
pub type W = crate::W<Mr0Spec>;
#[doc = "Field `WDTV` reader - WDTV"]
pub type WdtvR = crate::FieldReader<u16>;
#[doc = "Field `WDTV` writer - WDTV"]
pub type WdtvW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WDTFIEN` reader - WDTFIEN"]
pub type WdtfienR = crate::BitReader;
#[doc = "Field `WDTFIEN` writer - WDTFIEN"]
pub type WdtfienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTRSTEN` reader - WDTRSTEN"]
pub type WdtrstenR = crate::BitReader;
#[doc = "Field `WDTRSTEN` writer - WDTRSTEN"]
pub type WdtrstenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTSHLT` reader - WDTSHLT"]
pub type WdtshltR = crate::FieldReader;
#[doc = "Field `WDTSHLT` writer - WDTSHLT"]
pub type WdtshltW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WDTEN` reader - WDTEN"]
pub type WdtenR = crate::BitReader;
#[doc = "Field `WDTEN` writer - WDTEN"]
pub type WdtenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - WDTV"]
    #[inline(always)]
    pub fn wdtv(&self) -> WdtvR {
        WdtvR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - WDTFIEN"]
    #[inline(always)]
    pub fn wdtfien(&self) -> WdtfienR {
        WdtfienR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - WDTRSTEN"]
    #[inline(always)]
    pub fn wdtrsten(&self) -> WdtrstenR {
        WdtrstenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - WDTSHLT"]
    #[inline(always)]
    pub fn wdtshlt(&self) -> WdtshltR {
        WdtshltR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - WDTEN"]
    #[inline(always)]
    pub fn wdten(&self) -> WdtenR {
        WdtenR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - WDTV"]
    #[inline(always)]
    #[must_use]
    pub fn wdtv(&mut self) -> WdtvW<Mr0Spec> {
        WdtvW::new(self, 0)
    }
    #[doc = "Bit 12 - WDTFIEN"]
    #[inline(always)]
    #[must_use]
    pub fn wdtfien(&mut self) -> WdtfienW<Mr0Spec> {
        WdtfienW::new(self, 12)
    }
    #[doc = "Bit 13 - WDTRSTEN"]
    #[inline(always)]
    #[must_use]
    pub fn wdtrsten(&mut self) -> WdtrstenW<Mr0Spec> {
        WdtrstenW::new(self, 13)
    }
    #[doc = "Bits 14:15 - WDTSHLT"]
    #[inline(always)]
    #[must_use]
    pub fn wdtshlt(&mut self) -> WdtshltW<Mr0Spec> {
        WdtshltW::new(self, 14)
    }
    #[doc = "Bit 16 - WDTEN"]
    #[inline(always)]
    #[must_use]
    pub fn wdten(&mut self) -> WdtenW<Mr0Spec> {
        WdtenW::new(self, 16)
    }
}
#[doc = "MR0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mr0Spec;
impl crate::RegisterSpec for Mr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr0::R`](R) reader structure"]
impl crate::Readable for Mr0Spec {}
#[doc = "`write(|w| ..)` method takes [`mr0::W`](W) writer structure"]
impl crate::Writable for Mr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MR0 to value 0"]
impl crate::Resettable for Mr0Spec {
    const RESET_VALUE: u32 = 0;
}
