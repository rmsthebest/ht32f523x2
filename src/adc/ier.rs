#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `ADIES` reader - ADIES"]
pub type AdiesR = crate::BitReader;
#[doc = "Field `ADIES` writer - ADIES"]
pub type AdiesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADIEG` reader - ADIEG"]
pub type AdiegR = crate::BitReader;
#[doc = "Field `ADIEG` writer - ADIEG"]
pub type AdiegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADIEC` reader - ADIEC"]
pub type AdiecR = crate::BitReader;
#[doc = "Field `ADIEC` writer - ADIEC"]
pub type AdiecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADIEL` reader - ADIEL"]
pub type AdielR = crate::BitReader;
#[doc = "Field `ADIEL` writer - ADIEL"]
pub type AdielW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADIEU` reader - ADIEU"]
pub type AdieuR = crate::BitReader;
#[doc = "Field `ADIEU` writer - ADIEU"]
pub type AdieuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADIEO` reader - ADIEO"]
pub type AdieoR = crate::BitReader;
#[doc = "Field `ADIEO` writer - ADIEO"]
pub type AdieoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADIES"]
    #[inline(always)]
    pub fn adies(&self) -> AdiesR {
        AdiesR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADIEG"]
    #[inline(always)]
    pub fn adieg(&self) -> AdiegR {
        AdiegR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADIEC"]
    #[inline(always)]
    pub fn adiec(&self) -> AdiecR {
        AdiecR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - ADIEL"]
    #[inline(always)]
    pub fn adiel(&self) -> AdielR {
        AdielR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADIEU"]
    #[inline(always)]
    pub fn adieu(&self) -> AdieuR {
        AdieuR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - ADIEO"]
    #[inline(always)]
    pub fn adieo(&self) -> AdieoR {
        AdieoR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADIES"]
    #[inline(always)]
    #[must_use]
    pub fn adies(&mut self) -> AdiesW<IerSpec> {
        AdiesW::new(self, 0)
    }
    #[doc = "Bit 1 - ADIEG"]
    #[inline(always)]
    #[must_use]
    pub fn adieg(&mut self) -> AdiegW<IerSpec> {
        AdiegW::new(self, 1)
    }
    #[doc = "Bit 2 - ADIEC"]
    #[inline(always)]
    #[must_use]
    pub fn adiec(&mut self) -> AdiecW<IerSpec> {
        AdiecW::new(self, 2)
    }
    #[doc = "Bit 16 - ADIEL"]
    #[inline(always)]
    #[must_use]
    pub fn adiel(&mut self) -> AdielW<IerSpec> {
        AdielW::new(self, 16)
    }
    #[doc = "Bit 17 - ADIEU"]
    #[inline(always)]
    #[must_use]
    pub fn adieu(&mut self) -> AdieuW<IerSpec> {
        AdieuW::new(self, 17)
    }
    #[doc = "Bit 24 - ADIEO"]
    #[inline(always)]
    #[must_use]
    pub fn adieo(&mut self) -> AdieoW<IerSpec> {
        AdieoW::new(self, 24)
    }
}
#[doc = "IMR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
