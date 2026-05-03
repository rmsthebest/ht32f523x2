#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<IsrSpec>;
#[doc = "Field `ADISRS` reader - ADISRS"]
pub type AdisrsR = crate::BitReader;
#[doc = "Field `ADISRS` writer - ADISRS"]
pub type AdisrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADISRG` reader - ADISRG"]
pub type AdisrgR = crate::BitReader;
#[doc = "Field `ADISRG` writer - ADISRG"]
pub type AdisrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADISRC` reader - ADISRC"]
pub type AdisrcR = crate::BitReader;
#[doc = "Field `ADISRC` writer - ADISRC"]
pub type AdisrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADISRL` reader - ADISRL"]
pub type AdisrlR = crate::BitReader;
#[doc = "Field `ADISRL` writer - ADISRL"]
pub type AdisrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADISRU` reader - ADISRU"]
pub type AdisruR = crate::BitReader;
#[doc = "Field `ADISRU` writer - ADISRU"]
pub type AdisruW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADISRO` reader - ADISRO"]
pub type AdisroR = crate::BitReader;
#[doc = "Field `ADISRO` writer - ADISRO"]
pub type AdisroW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADISRS"]
    #[inline(always)]
    pub fn adisrs(&self) -> AdisrsR {
        AdisrsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADISRG"]
    #[inline(always)]
    pub fn adisrg(&self) -> AdisrgR {
        AdisrgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADISRC"]
    #[inline(always)]
    pub fn adisrc(&self) -> AdisrcR {
        AdisrcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - ADISRL"]
    #[inline(always)]
    pub fn adisrl(&self) -> AdisrlR {
        AdisrlR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADISRU"]
    #[inline(always)]
    pub fn adisru(&self) -> AdisruR {
        AdisruR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - ADISRO"]
    #[inline(always)]
    pub fn adisro(&self) -> AdisroR {
        AdisroR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADISRS"]
    #[inline(always)]
    pub fn adisrs(&mut self) -> AdisrsW<'_, IsrSpec> {
        AdisrsW::new(self, 0)
    }
    #[doc = "Bit 1 - ADISRG"]
    #[inline(always)]
    pub fn adisrg(&mut self) -> AdisrgW<'_, IsrSpec> {
        AdisrgW::new(self, 1)
    }
    #[doc = "Bit 2 - ADISRC"]
    #[inline(always)]
    pub fn adisrc(&mut self) -> AdisrcW<'_, IsrSpec> {
        AdisrcW::new(self, 2)
    }
    #[doc = "Bit 16 - ADISRL"]
    #[inline(always)]
    pub fn adisrl(&mut self) -> AdisrlW<'_, IsrSpec> {
        AdisrlW::new(self, 16)
    }
    #[doc = "Bit 17 - ADISRU"]
    #[inline(always)]
    pub fn adisru(&mut self) -> AdisruW<'_, IsrSpec> {
        AdisruW::new(self, 17)
    }
    #[doc = "Bit 24 - ADISRO"]
    #[inline(always)]
    pub fn adisro(&mut self) -> AdisroW<'_, IsrSpec> {
        AdisroW::new(self, 24)
    }
}
#[doc = "ISR\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for IsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
