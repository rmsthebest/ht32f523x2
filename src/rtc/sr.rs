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
    pub fn csecflag(&mut self) -> CsecflagW<'_, SrSpec> {
        CsecflagW::new(self, 0)
    }
    #[doc = "Bit 1 - CMFLAG"]
    #[inline(always)]
    pub fn cmflag(&mut self) -> CmflagW<'_, SrSpec> {
        CmflagW::new(self, 1)
    }
    #[doc = "Bit 2 - OVFLAG"]
    #[inline(always)]
    pub fn ovflag(&mut self) -> OvflagW<'_, SrSpec> {
        OvflagW::new(self, 2)
    }
}
#[doc = "SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
