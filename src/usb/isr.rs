#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<IsrSpec>;
#[doc = "Field `SOFIF` reader - SOFIF"]
pub type SofifR = crate::BitReader;
#[doc = "Field `SOFIF` writer - SOFIF"]
pub type SofifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URSTIF` reader - URSTIF"]
pub type UrstifR = crate::BitReader;
#[doc = "Field `URSTIF` writer - URSTIF"]
pub type UrstifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSMIF` reader - RSMIF"]
pub type RsmifR = crate::BitReader;
#[doc = "Field `RSMIF` writer - RSMIF"]
pub type RsmifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPIF` reader - SUSPIF"]
pub type SuspifR = crate::BitReader;
#[doc = "Field `SUSPIF` writer - SUSPIF"]
pub type SuspifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESOFIF` reader - ESOFIF"]
pub type EsofifR = crate::BitReader;
#[doc = "Field `ESOFIF` writer - ESOFIF"]
pub type EsofifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP0IF` reader - EP0IF"]
pub type Ep0ifR = crate::BitReader;
#[doc = "Field `EP0IF` writer - EP0IF"]
pub type Ep0ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP1IF` reader - EP1IF"]
pub type Ep1ifR = crate::BitReader;
#[doc = "Field `EP1IF` writer - EP1IF"]
pub type Ep1ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2IF` reader - EP2IF"]
pub type Ep2ifR = crate::BitReader;
#[doc = "Field `EP2IF` writer - EP2IF"]
pub type Ep2ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3IF` reader - EP3IF"]
pub type Ep3ifR = crate::BitReader;
#[doc = "Field `EP3IF` writer - EP3IF"]
pub type Ep3ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP4IF` reader - EP4IF"]
pub type Ep4ifR = crate::BitReader;
#[doc = "Field `EP4IF` writer - EP4IF"]
pub type Ep4ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP5IF` reader - EP5IF"]
pub type Ep5ifR = crate::BitReader;
#[doc = "Field `EP5IF` writer - EP5IF"]
pub type Ep5ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP6IF` reader - EP6IF"]
pub type Ep6ifR = crate::BitReader;
#[doc = "Field `EP6IF` writer - EP6IF"]
pub type Ep6ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP7IF` reader - EP7IF"]
pub type Ep7ifR = crate::BitReader;
#[doc = "Field `EP7IF` writer - EP7IF"]
pub type Ep7ifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - SOFIF"]
    #[inline(always)]
    pub fn sofif(&self) -> SofifR {
        SofifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - URSTIF"]
    #[inline(always)]
    pub fn urstif(&self) -> UrstifR {
        UrstifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RSMIF"]
    #[inline(always)]
    pub fn rsmif(&self) -> RsmifR {
        RsmifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SUSPIF"]
    #[inline(always)]
    pub fn suspif(&self) -> SuspifR {
        SuspifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ESOFIF"]
    #[inline(always)]
    pub fn esofif(&self) -> EsofifR {
        EsofifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - EP0IF"]
    #[inline(always)]
    pub fn ep0if(&self) -> Ep0ifR {
        Ep0ifR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EP1IF"]
    #[inline(always)]
    pub fn ep1if(&self) -> Ep1ifR {
        Ep1ifR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EP2IF"]
    #[inline(always)]
    pub fn ep2if(&self) -> Ep2ifR {
        Ep2ifR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EP3IF"]
    #[inline(always)]
    pub fn ep3if(&self) -> Ep3ifR {
        Ep3ifR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EP4IF"]
    #[inline(always)]
    pub fn ep4if(&self) -> Ep4ifR {
        Ep4ifR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EP5IF"]
    #[inline(always)]
    pub fn ep5if(&self) -> Ep5ifR {
        Ep5ifR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EP6IF"]
    #[inline(always)]
    pub fn ep6if(&self) -> Ep6ifR {
        Ep6ifR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EP7IF"]
    #[inline(always)]
    pub fn ep7if(&self) -> Ep7ifR {
        Ep7ifR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SOFIF"]
    #[inline(always)]
    #[must_use]
    pub fn sofif(&mut self) -> SofifW<IsrSpec> {
        SofifW::new(self, 1)
    }
    #[doc = "Bit 2 - URSTIF"]
    #[inline(always)]
    #[must_use]
    pub fn urstif(&mut self) -> UrstifW<IsrSpec> {
        UrstifW::new(self, 2)
    }
    #[doc = "Bit 3 - RSMIF"]
    #[inline(always)]
    #[must_use]
    pub fn rsmif(&mut self) -> RsmifW<IsrSpec> {
        RsmifW::new(self, 3)
    }
    #[doc = "Bit 4 - SUSPIF"]
    #[inline(always)]
    #[must_use]
    pub fn suspif(&mut self) -> SuspifW<IsrSpec> {
        SuspifW::new(self, 4)
    }
    #[doc = "Bit 5 - ESOFIF"]
    #[inline(always)]
    #[must_use]
    pub fn esofif(&mut self) -> EsofifW<IsrSpec> {
        EsofifW::new(self, 5)
    }
    #[doc = "Bit 8 - EP0IF"]
    #[inline(always)]
    #[must_use]
    pub fn ep0if(&mut self) -> Ep0ifW<IsrSpec> {
        Ep0ifW::new(self, 8)
    }
    #[doc = "Bit 9 - EP1IF"]
    #[inline(always)]
    #[must_use]
    pub fn ep1if(&mut self) -> Ep1ifW<IsrSpec> {
        Ep1ifW::new(self, 9)
    }
    #[doc = "Bit 10 - EP2IF"]
    #[inline(always)]
    #[must_use]
    pub fn ep2if(&mut self) -> Ep2ifW<IsrSpec> {
        Ep2ifW::new(self, 10)
    }
    #[doc = "Bit 11 - EP3IF"]
    #[inline(always)]
    #[must_use]
    pub fn ep3if(&mut self) -> Ep3ifW<IsrSpec> {
        Ep3ifW::new(self, 11)
    }
    #[doc = "Bit 12 - EP4IF"]
    #[inline(always)]
    #[must_use]
    pub fn ep4if(&mut self) -> Ep4ifW<IsrSpec> {
        Ep4ifW::new(self, 12)
    }
    #[doc = "Bit 13 - EP5IF"]
    #[inline(always)]
    #[must_use]
    pub fn ep5if(&mut self) -> Ep5ifW<IsrSpec> {
        Ep5ifW::new(self, 13)
    }
    #[doc = "Bit 14 - EP6IF"]
    #[inline(always)]
    #[must_use]
    pub fn ep6if(&mut self) -> Ep6ifW<IsrSpec> {
        Ep6ifW::new(self, 14)
    }
    #[doc = "Bit 15 - EP7IF"]
    #[inline(always)]
    #[must_use]
    pub fn ep7if(&mut self) -> Ep7ifW<IsrSpec> {
        Ep7ifW::new(self, 15)
    }
}
#[doc = "ISR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for IsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {
    const RESET_VALUE: u32 = 0;
}
