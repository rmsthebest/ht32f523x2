#[doc = "Register `TR` reader"]
pub type R = crate::R<TrSpec>;
#[doc = "Register `TR` writer"]
pub type W = crate::W<TrSpec>;
#[doc = "Field `ADLT` reader - ADLT"]
pub type AdltR = crate::FieldReader<u16>;
#[doc = "Field `ADLT` writer - ADLT"]
pub type AdltW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ADUT` reader - ADUT"]
pub type AdutR = crate::FieldReader<u16>;
#[doc = "Field `ADUT` writer - ADUT"]
pub type AdutW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - ADLT"]
    #[inline(always)]
    pub fn adlt(&self) -> AdltR {
        AdltR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - ADUT"]
    #[inline(always)]
    pub fn adut(&self) -> AdutR {
        AdutR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADLT"]
    #[inline(always)]
    #[must_use]
    pub fn adlt(&mut self) -> AdltW<TrSpec> {
        AdltW::new(self, 0)
    }
    #[doc = "Bits 16:27 - ADUT"]
    #[inline(always)]
    #[must_use]
    pub fn adut(&mut self) -> AdutW<TrSpec> {
        AdutW::new(self, 16)
    }
}
#[doc = "TR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrSpec;
impl crate::RegisterSpec for TrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr::R`](R) reader structure"]
impl crate::Readable for TrSpec {}
#[doc = "`write(|w| ..)` method takes [`tr::W`](W) writer structure"]
impl crate::Writable for TrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR to value 0"]
impl crate::Resettable for TrSpec {
    const RESET_VALUE: u32 = 0;
}
