#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `CSECFLAG` reader - CSECFLAG"]
pub type CsecflagR = crate::BitReader;
#[doc = "Field `CSECFLAG` writer - CSECFLAG"]
pub type CsecflagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMFLAG` reader - CMFLAG"]
pub type CmflagR = crate::BitReader;
#[doc = "Field `CMFLAG` writer - CMFLAG"]
pub type CmflagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFLAG` reader - OVFLAG"]
pub type OvflagR = crate::BitReader;
#[doc = "Field `OVFLAG` writer - OVFLAG"]
pub type OvflagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CSECFLAG"]
    #[inline(always)]
    pub fn csecflag(&self) -> CsecflagR {
        CsecflagR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMFLAG"]
    #[inline(always)]
    pub fn cmflag(&self) -> CmflagR {
        CmflagR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OVFLAG"]
    #[inline(always)]
    pub fn ovflag(&self) -> OvflagR {
        OvflagR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CSECFLAG"]
    #[inline(always)]
    #[must_use]
    pub fn csecflag(&mut self) -> CsecflagW<SrSpec> {
        CsecflagW::new(self, 0)
    }
    #[doc = "Bit 1 - CMFLAG"]
    #[inline(always)]
    #[must_use]
    pub fn cmflag(&mut self) -> CmflagW<SrSpec> {
        CmflagW::new(self, 1)
    }
    #[doc = "Bit 2 - OVFLAG"]
    #[inline(always)]
    #[must_use]
    pub fn ovflag(&mut self) -> OvflagW<SrSpec> {
        OvflagW::new(self, 2)
    }
}
#[doc = "SR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0;
}
