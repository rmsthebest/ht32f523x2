#[doc = "Register `ICLR` reader"]
pub type R = crate::R<IclrSpec>;
#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "Field `ADICLRS` reader - ADICLRS"]
pub type AdiclrsR = crate::BitReader;
#[doc = "Field `ADICLRS` writer - ADICLRS"]
pub type AdiclrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADICLRG` reader - ADICLRG"]
pub type AdiclrgR = crate::BitReader;
#[doc = "Field `ADICLRG` writer - ADICLRG"]
pub type AdiclrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADICLRC` reader - ADICLRC"]
pub type AdiclrcR = crate::BitReader;
#[doc = "Field `ADICLRC` writer - ADICLRC"]
pub type AdiclrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADICLRL` reader - ADICLRL"]
pub type AdiclrlR = crate::BitReader;
#[doc = "Field `ADICLRL` writer - ADICLRL"]
pub type AdiclrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADICLRU` reader - ADICLRU"]
pub type AdiclruR = crate::BitReader;
#[doc = "Field `ADICLRU` writer - ADICLRU"]
pub type AdiclruW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADICLRO` reader - ADICLRO"]
pub type AdiclroR = crate::BitReader;
#[doc = "Field `ADICLRO` writer - ADICLRO"]
pub type AdiclroW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADICLRS"]
    #[inline(always)]
    pub fn adiclrs(&self) -> AdiclrsR {
        AdiclrsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADICLRG"]
    #[inline(always)]
    pub fn adiclrg(&self) -> AdiclrgR {
        AdiclrgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADICLRC"]
    #[inline(always)]
    pub fn adiclrc(&self) -> AdiclrcR {
        AdiclrcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - ADICLRL"]
    #[inline(always)]
    pub fn adiclrl(&self) -> AdiclrlR {
        AdiclrlR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADICLRU"]
    #[inline(always)]
    pub fn adiclru(&self) -> AdiclruR {
        AdiclruR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - ADICLRO"]
    #[inline(always)]
    pub fn adiclro(&self) -> AdiclroR {
        AdiclroR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADICLRS"]
    #[inline(always)]
    pub fn adiclrs(&mut self) -> AdiclrsW<'_, IclrSpec> {
        AdiclrsW::new(self, 0)
    }
    #[doc = "Bit 1 - ADICLRG"]
    #[inline(always)]
    pub fn adiclrg(&mut self) -> AdiclrgW<'_, IclrSpec> {
        AdiclrgW::new(self, 1)
    }
    #[doc = "Bit 2 - ADICLRC"]
    #[inline(always)]
    pub fn adiclrc(&mut self) -> AdiclrcW<'_, IclrSpec> {
        AdiclrcW::new(self, 2)
    }
    #[doc = "Bit 16 - ADICLRL"]
    #[inline(always)]
    pub fn adiclrl(&mut self) -> AdiclrlW<'_, IclrSpec> {
        AdiclrlW::new(self, 16)
    }
    #[doc = "Bit 17 - ADICLRU"]
    #[inline(always)]
    pub fn adiclru(&mut self) -> AdiclruW<'_, IclrSpec> {
        AdiclruW::new(self, 17)
    }
    #[doc = "Bit 24 - ADICLRO"]
    #[inline(always)]
    pub fn adiclro(&mut self) -> AdiclroW<'_, IclrSpec> {
        AdiclroW::new(self, 24)
    }
}
#[doc = "ICLR\n\nYou can [`read`](crate::Reg::read) this register and get [`iclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IclrSpec;
impl crate::RegisterSpec for IclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iclr::R`](R) reader structure"]
impl crate::Readable for IclrSpec {}
#[doc = "`write(|w| ..)` method takes [`iclr::W`](W) writer structure"]
impl crate::Writable for IclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICLR to value 0"]
impl crate::Resettable for IclrSpec {}
