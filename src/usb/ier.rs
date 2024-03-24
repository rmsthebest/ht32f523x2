#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `UGIE` reader - UGIE"]
pub type UgieR = crate::BitReader;
#[doc = "Field `UGIE` writer - UGIE"]
pub type UgieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFIE` reader - SOFIE"]
pub type SofieR = crate::BitReader;
#[doc = "Field `SOFIE` writer - SOFIE"]
pub type SofieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URSTIE` reader - URSTIE"]
pub type UrstieR = crate::BitReader;
#[doc = "Field `URSTIE` writer - URSTIE"]
pub type UrstieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSMIE` reader - RSMIE"]
pub type RsmieR = crate::BitReader;
#[doc = "Field `RSMIE` writer - RSMIE"]
pub type RsmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPIE` reader - SUSPIE"]
pub type SuspieR = crate::BitReader;
#[doc = "Field `SUSPIE` writer - SUSPIE"]
pub type SuspieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESOFIE` reader - ESOFIE"]
pub type EsofieR = crate::BitReader;
#[doc = "Field `ESOFIE` writer - ESOFIE"]
pub type EsofieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP0IE` reader - EP0IE"]
pub type Ep0ieR = crate::BitReader;
#[doc = "Field `EP0IE` writer - EP0IE"]
pub type Ep0ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP1IE` reader - EP1IE"]
pub type Ep1ieR = crate::BitReader;
#[doc = "Field `EP1IE` writer - EP1IE"]
pub type Ep1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2IE` reader - EP2IE"]
pub type Ep2ieR = crate::BitReader;
#[doc = "Field `EP2IE` writer - EP2IE"]
pub type Ep2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3IE` reader - EP3IE"]
pub type Ep3ieR = crate::BitReader;
#[doc = "Field `EP3IE` writer - EP3IE"]
pub type Ep3ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP4IE` reader - EP4IE"]
pub type Ep4ieR = crate::BitReader;
#[doc = "Field `EP4IE` writer - EP4IE"]
pub type Ep4ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP5IE` reader - EP5IE"]
pub type Ep5ieR = crate::BitReader;
#[doc = "Field `EP5IE` writer - EP5IE"]
pub type Ep5ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP6IE` reader - EP6IE"]
pub type Ep6ieR = crate::BitReader;
#[doc = "Field `EP6IE` writer - EP6IE"]
pub type Ep6ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP7IE` reader - EP7IE"]
pub type Ep7ieR = crate::BitReader;
#[doc = "Field `EP7IE` writer - EP7IE"]
pub type Ep7ieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UGIE"]
    #[inline(always)]
    pub fn ugie(&self) -> UgieR {
        UgieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SOFIE"]
    #[inline(always)]
    pub fn sofie(&self) -> SofieR {
        SofieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - URSTIE"]
    #[inline(always)]
    pub fn urstie(&self) -> UrstieR {
        UrstieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RSMIE"]
    #[inline(always)]
    pub fn rsmie(&self) -> RsmieR {
        RsmieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SUSPIE"]
    #[inline(always)]
    pub fn suspie(&self) -> SuspieR {
        SuspieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ESOFIE"]
    #[inline(always)]
    pub fn esofie(&self) -> EsofieR {
        EsofieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - EP0IE"]
    #[inline(always)]
    pub fn ep0ie(&self) -> Ep0ieR {
        Ep0ieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EP1IE"]
    #[inline(always)]
    pub fn ep1ie(&self) -> Ep1ieR {
        Ep1ieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EP2IE"]
    #[inline(always)]
    pub fn ep2ie(&self) -> Ep2ieR {
        Ep2ieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EP3IE"]
    #[inline(always)]
    pub fn ep3ie(&self) -> Ep3ieR {
        Ep3ieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EP4IE"]
    #[inline(always)]
    pub fn ep4ie(&self) -> Ep4ieR {
        Ep4ieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EP5IE"]
    #[inline(always)]
    pub fn ep5ie(&self) -> Ep5ieR {
        Ep5ieR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EP6IE"]
    #[inline(always)]
    pub fn ep6ie(&self) -> Ep6ieR {
        Ep6ieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EP7IE"]
    #[inline(always)]
    pub fn ep7ie(&self) -> Ep7ieR {
        Ep7ieR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UGIE"]
    #[inline(always)]
    #[must_use]
    pub fn ugie(&mut self) -> UgieW<IerSpec> {
        UgieW::new(self, 0)
    }
    #[doc = "Bit 1 - SOFIE"]
    #[inline(always)]
    #[must_use]
    pub fn sofie(&mut self) -> SofieW<IerSpec> {
        SofieW::new(self, 1)
    }
    #[doc = "Bit 2 - URSTIE"]
    #[inline(always)]
    #[must_use]
    pub fn urstie(&mut self) -> UrstieW<IerSpec> {
        UrstieW::new(self, 2)
    }
    #[doc = "Bit 3 - RSMIE"]
    #[inline(always)]
    #[must_use]
    pub fn rsmie(&mut self) -> RsmieW<IerSpec> {
        RsmieW::new(self, 3)
    }
    #[doc = "Bit 4 - SUSPIE"]
    #[inline(always)]
    #[must_use]
    pub fn suspie(&mut self) -> SuspieW<IerSpec> {
        SuspieW::new(self, 4)
    }
    #[doc = "Bit 5 - ESOFIE"]
    #[inline(always)]
    #[must_use]
    pub fn esofie(&mut self) -> EsofieW<IerSpec> {
        EsofieW::new(self, 5)
    }
    #[doc = "Bit 8 - EP0IE"]
    #[inline(always)]
    #[must_use]
    pub fn ep0ie(&mut self) -> Ep0ieW<IerSpec> {
        Ep0ieW::new(self, 8)
    }
    #[doc = "Bit 9 - EP1IE"]
    #[inline(always)]
    #[must_use]
    pub fn ep1ie(&mut self) -> Ep1ieW<IerSpec> {
        Ep1ieW::new(self, 9)
    }
    #[doc = "Bit 10 - EP2IE"]
    #[inline(always)]
    #[must_use]
    pub fn ep2ie(&mut self) -> Ep2ieW<IerSpec> {
        Ep2ieW::new(self, 10)
    }
    #[doc = "Bit 11 - EP3IE"]
    #[inline(always)]
    #[must_use]
    pub fn ep3ie(&mut self) -> Ep3ieW<IerSpec> {
        Ep3ieW::new(self, 11)
    }
    #[doc = "Bit 12 - EP4IE"]
    #[inline(always)]
    #[must_use]
    pub fn ep4ie(&mut self) -> Ep4ieW<IerSpec> {
        Ep4ieW::new(self, 12)
    }
    #[doc = "Bit 13 - EP5IE"]
    #[inline(always)]
    #[must_use]
    pub fn ep5ie(&mut self) -> Ep5ieW<IerSpec> {
        Ep5ieW::new(self, 13)
    }
    #[doc = "Bit 14 - EP6IE"]
    #[inline(always)]
    #[must_use]
    pub fn ep6ie(&mut self) -> Ep6ieW<IerSpec> {
        Ep6ieW::new(self, 14)
    }
    #[doc = "Bit 15 - EP7IE"]
    #[inline(always)]
    #[must_use]
    pub fn ep7ie(&mut self) -> Ep7ieW<IerSpec> {
        Ep7ieW::new(self, 15)
    }
}
#[doc = "IER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {
    const RESET_VALUE: u32 = 0;
}
