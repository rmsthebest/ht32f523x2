#[doc = "Register `TFR0` reader"]
pub type R = crate::R<Tfr0Spec>;
#[doc = "Register `TFR0` writer"]
pub type W = crate::W<Tfr0Spec>;
#[doc = "Field `CMPFF` reader - CMPFF"]
pub type CmpffR = crate::BitReader;
#[doc = "Field `CMPFF` writer - CMPFF"]
pub type CmpffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPRF` reader - CMPRF"]
pub type CmprfR = crate::BitReader;
#[doc = "Field `CMPRF` writer - CMPRF"]
pub type CmprfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPFDEN` reader - CMPFDEN"]
pub type CmpfdenR = crate::BitReader;
#[doc = "Field `CMPFDEN` writer - CMPFDEN"]
pub type CmpfdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPRDEN` reader - CMPRDEN"]
pub type CmprdenR = crate::BitReader;
#[doc = "Field `CMPRDEN` writer - CMPRDEN"]
pub type CmprdenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CMPFF"]
    #[inline(always)]
    pub fn cmpff(&self) -> CmpffR {
        CmpffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMPRF"]
    #[inline(always)]
    pub fn cmprf(&self) -> CmprfR {
        CmprfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - CMPFDEN"]
    #[inline(always)]
    pub fn cmpfden(&self) -> CmpfdenR {
        CmpfdenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CMPRDEN"]
    #[inline(always)]
    pub fn cmprden(&self) -> CmprdenR {
        CmprdenR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMPFF"]
    #[inline(always)]
    pub fn cmpff(&mut self) -> CmpffW<'_, Tfr0Spec> {
        CmpffW::new(self, 0)
    }
    #[doc = "Bit 1 - CMPRF"]
    #[inline(always)]
    pub fn cmprf(&mut self) -> CmprfW<'_, Tfr0Spec> {
        CmprfW::new(self, 1)
    }
    #[doc = "Bit 8 - CMPFDEN"]
    #[inline(always)]
    pub fn cmpfden(&mut self) -> CmpfdenW<'_, Tfr0Spec> {
        CmpfdenW::new(self, 8)
    }
    #[doc = "Bit 9 - CMPRDEN"]
    #[inline(always)]
    pub fn cmprden(&mut self) -> CmprdenW<'_, Tfr0Spec> {
        CmprdenW::new(self, 9)
    }
}
#[doc = "TFR0\n\nYou can [`read`](crate::Reg::read) this register and get [`tfr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tfr0Spec;
impl crate::RegisterSpec for Tfr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfr0::R`](R) reader structure"]
impl crate::Readable for Tfr0Spec {}
#[doc = "`write(|w| ..)` method takes [`tfr0::W`](W) writer structure"]
impl crate::Writable for Tfr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TFR0 to value 0"]
impl crate::Resettable for Tfr0Spec {}
