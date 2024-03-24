#[doc = "Register `IWEN` reader"]
pub type R = crate::R<IwenSpec>;
#[doc = "Register `IWEN` writer"]
pub type W = crate::W<IwenSpec>;
#[doc = "Field `CSECIEN` reader - CSECIEN"]
pub type CsecienR = crate::BitReader;
#[doc = "Field `CSECIEN` writer - CSECIEN"]
pub type CsecienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMIEN` reader - CMIEN"]
pub type CmienR = crate::BitReader;
#[doc = "Field `CMIEN` writer - CMIEN"]
pub type CmienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVIEN` reader - OVIEN"]
pub type OvienR = crate::BitReader;
#[doc = "Field `OVIEN` writer - OVIEN"]
pub type OvienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSECWEN` reader - CSECWEN"]
pub type CsecwenR = crate::BitReader;
#[doc = "Field `CSECWEN` writer - CSECWEN"]
pub type CsecwenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMWEN` reader - CMWEN"]
pub type CmwenR = crate::BitReader;
#[doc = "Field `CMWEN` writer - CMWEN"]
pub type CmwenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVWEN` reader - OVWEN"]
pub type OvwenR = crate::BitReader;
#[doc = "Field `OVWEN` writer - OVWEN"]
pub type OvwenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CSECIEN"]
    #[inline(always)]
    pub fn csecien(&self) -> CsecienR {
        CsecienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMIEN"]
    #[inline(always)]
    pub fn cmien(&self) -> CmienR {
        CmienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OVIEN"]
    #[inline(always)]
    pub fn ovien(&self) -> OvienR {
        OvienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - CSECWEN"]
    #[inline(always)]
    pub fn csecwen(&self) -> CsecwenR {
        CsecwenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CMWEN"]
    #[inline(always)]
    pub fn cmwen(&self) -> CmwenR {
        CmwenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OVWEN"]
    #[inline(always)]
    pub fn ovwen(&self) -> OvwenR {
        OvwenR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CSECIEN"]
    #[inline(always)]
    #[must_use]
    pub fn csecien(&mut self) -> CsecienW<IwenSpec> {
        CsecienW::new(self, 0)
    }
    #[doc = "Bit 1 - CMIEN"]
    #[inline(always)]
    #[must_use]
    pub fn cmien(&mut self) -> CmienW<IwenSpec> {
        CmienW::new(self, 1)
    }
    #[doc = "Bit 2 - OVIEN"]
    #[inline(always)]
    #[must_use]
    pub fn ovien(&mut self) -> OvienW<IwenSpec> {
        OvienW::new(self, 2)
    }
    #[doc = "Bit 8 - CSECWEN"]
    #[inline(always)]
    #[must_use]
    pub fn csecwen(&mut self) -> CsecwenW<IwenSpec> {
        CsecwenW::new(self, 8)
    }
    #[doc = "Bit 9 - CMWEN"]
    #[inline(always)]
    #[must_use]
    pub fn cmwen(&mut self) -> CmwenW<IwenSpec> {
        CmwenW::new(self, 9)
    }
    #[doc = "Bit 10 - OVWEN"]
    #[inline(always)]
    #[must_use]
    pub fn ovwen(&mut self) -> OvwenW<IwenSpec> {
        OvwenW::new(self, 10)
    }
}
#[doc = "IWEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwenSpec;
impl crate::RegisterSpec for IwenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwen::R`](R) reader structure"]
impl crate::Readable for IwenSpec {}
#[doc = "`write(|w| ..)` method takes [`iwen::W`](W) writer structure"]
impl crate::Writable for IwenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IWEN to value 0"]
impl crate::Resettable for IwenSpec {
    const RESET_VALUE: u32 = 0;
}
