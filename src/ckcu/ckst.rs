#[doc = "Register `CKST` reader"]
pub type R = crate::R<CkstSpec>;
#[doc = "Register `CKST` writer"]
pub type W = crate::W<CkstSpec>;
#[doc = "Field `CKSWST` reader - CKSWST"]
pub type CkswstR = crate::FieldReader;
#[doc = "Field `CKSWST` writer - CKSWST"]
pub type CkswstW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLLST` reader - PLLST"]
pub type PllstR = crate::FieldReader;
#[doc = "Field `PLLST` writer - PLLST"]
pub type PllstW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HSEST` reader - HSEST"]
pub type HsestR = crate::FieldReader;
#[doc = "Field `HSEST` writer - HSEST"]
pub type HsestW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HSIST` reader - HSIST"]
pub type HsistR = crate::FieldReader;
#[doc = "Field `HSIST` writer - HSIST"]
pub type HsistW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - CKSWST"]
    #[inline(always)]
    pub fn ckswst(&self) -> CkswstR {
        CkswstR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:11 - PLLST"]
    #[inline(always)]
    pub fn pllst(&self) -> PllstR {
        PllstR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - HSEST"]
    #[inline(always)]
    pub fn hsest(&self) -> HsestR {
        HsestR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:26 - HSIST"]
    #[inline(always)]
    pub fn hsist(&self) -> HsistR {
        HsistR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CKSWST"]
    #[inline(always)]
    pub fn ckswst(&mut self) -> CkswstW<'_, CkstSpec> {
        CkswstW::new(self, 0)
    }
    #[doc = "Bits 8:11 - PLLST"]
    #[inline(always)]
    pub fn pllst(&mut self) -> PllstW<'_, CkstSpec> {
        PllstW::new(self, 8)
    }
    #[doc = "Bits 16:17 - HSEST"]
    #[inline(always)]
    pub fn hsest(&mut self) -> HsestW<'_, CkstSpec> {
        HsestW::new(self, 16)
    }
    #[doc = "Bits 24:26 - HSIST"]
    #[inline(always)]
    pub fn hsist(&mut self) -> HsistW<'_, CkstSpec> {
        HsistW::new(self, 24)
    }
}
#[doc = "CKST\n\nYou can [`read`](crate::Reg::read) this register and get [`ckst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CkstSpec;
impl crate::RegisterSpec for CkstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckst::R`](R) reader structure"]
impl crate::Readable for CkstSpec {}
#[doc = "`write(|w| ..)` method takes [`ckst::W`](W) writer structure"]
impl crate::Writable for CkstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CKST to value 0"]
impl crate::Resettable for CkstSpec {}
