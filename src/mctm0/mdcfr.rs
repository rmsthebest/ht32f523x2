#[doc = "Register `MDCFR` reader"]
pub type R = crate::R<MdcfrSpec>;
#[doc = "Register `MDCFR` writer"]
pub type W = crate::W<MdcfrSpec>;
#[doc = "Field `TSE` reader - TSE"]
pub type TseR = crate::BitReader;
#[doc = "Field `TSE` writer - TSE"]
pub type TseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMSEL` reader - SMSEL"]
pub type SmselR = crate::FieldReader;
#[doc = "Field `SMSEL` writer - SMSEL"]
pub type SmselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MMSEL` reader - MMSEL"]
pub type MmselR = crate::FieldReader;
#[doc = "Field `MMSEL` writer - MMSEL"]
pub type MmselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPMSET` reader - SPMSET"]
pub type SpmsetR = crate::BitReader;
#[doc = "Field `SPMSET` writer - SPMSET"]
pub type SpmsetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TSE"]
    #[inline(always)]
    pub fn tse(&self) -> TseR {
        TseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - SMSEL"]
    #[inline(always)]
    pub fn smsel(&self) -> SmselR {
        SmselR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - MMSEL"]
    #[inline(always)]
    pub fn mmsel(&self) -> MmselR {
        MmselR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - SPMSET"]
    #[inline(always)]
    pub fn spmset(&self) -> SpmsetR {
        SpmsetR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TSE"]
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TseW<MdcfrSpec> {
        TseW::new(self, 0)
    }
    #[doc = "Bits 8:10 - SMSEL"]
    #[inline(always)]
    #[must_use]
    pub fn smsel(&mut self) -> SmselW<MdcfrSpec> {
        SmselW::new(self, 8)
    }
    #[doc = "Bits 16:18 - MMSEL"]
    #[inline(always)]
    #[must_use]
    pub fn mmsel(&mut self) -> MmselW<MdcfrSpec> {
        MmselW::new(self, 16)
    }
    #[doc = "Bit 24 - SPMSET"]
    #[inline(always)]
    #[must_use]
    pub fn spmset(&mut self) -> SpmsetW<MdcfrSpec> {
        SpmsetW::new(self, 24)
    }
}
#[doc = "MDCFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdcfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdcfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdcfrSpec;
impl crate::RegisterSpec for MdcfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdcfr::R`](R) reader structure"]
impl crate::Readable for MdcfrSpec {}
#[doc = "`write(|w| ..)` method takes [`mdcfr::W`](W) writer structure"]
impl crate::Writable for MdcfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDCFR to value 0"]
impl crate::Resettable for MdcfrSpec {
    const RESET_VALUE: u32 = 0;
}
