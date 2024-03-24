#[doc = "Register `OISR` reader"]
pub type R = crate::R<OisrSpec>;
#[doc = "Register `OISR` writer"]
pub type W = crate::W<OisrSpec>;
#[doc = "Field `ORFF` reader - ORFF"]
pub type OrffR = crate::BitReader;
#[doc = "Field `ORFF` writer - ORFF"]
pub type OrffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITADF` reader - ITADF"]
pub type ItadfR = crate::BitReader;
#[doc = "Field `ITADF` writer - ITADF"]
pub type ItadfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBEF` reader - OBEF"]
pub type ObefR = crate::BitReader;
#[doc = "Field `OBEF` writer - OBEF"]
pub type ObefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCMF` reader - IOCMF"]
pub type IocmfR = crate::BitReader;
#[doc = "Field `IOCMF` writer - IOCMF"]
pub type IocmfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OREF` reader - OREF"]
pub type OrefR = crate::BitReader;
#[doc = "Field `OREF` writer - OREF"]
pub type OrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RORFF` reader - RORFF"]
pub type RorffR = crate::BitReader;
#[doc = "Field `RORFF` writer - RORFF"]
pub type RorffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPEF` reader - PPEF"]
pub type PpefR = crate::BitReader;
#[doc = "Field `PPEF` writer - PPEF"]
pub type PpefW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ORFF"]
    #[inline(always)]
    pub fn orff(&self) -> OrffR {
        OrffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ITADF"]
    #[inline(always)]
    pub fn itadf(&self) -> ItadfR {
        ItadfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OBEF"]
    #[inline(always)]
    pub fn obef(&self) -> ObefR {
        ObefR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IOCMF"]
    #[inline(always)]
    pub fn iocmf(&self) -> IocmfR {
        IocmfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OREF"]
    #[inline(always)]
    pub fn oref(&self) -> OrefR {
        OrefR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - RORFF"]
    #[inline(always)]
    pub fn rorff(&self) -> RorffR {
        RorffR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PPEF"]
    #[inline(always)]
    pub fn ppef(&self) -> PpefR {
        PpefR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ORFF"]
    #[inline(always)]
    #[must_use]
    pub fn orff(&mut self) -> OrffW<OisrSpec> {
        OrffW::new(self, 0)
    }
    #[doc = "Bit 1 - ITADF"]
    #[inline(always)]
    #[must_use]
    pub fn itadf(&mut self) -> ItadfW<OisrSpec> {
        ItadfW::new(self, 1)
    }
    #[doc = "Bit 2 - OBEF"]
    #[inline(always)]
    #[must_use]
    pub fn obef(&mut self) -> ObefW<OisrSpec> {
        ObefW::new(self, 2)
    }
    #[doc = "Bit 3 - IOCMF"]
    #[inline(always)]
    #[must_use]
    pub fn iocmf(&mut self) -> IocmfW<OisrSpec> {
        IocmfW::new(self, 3)
    }
    #[doc = "Bit 4 - OREF"]
    #[inline(always)]
    #[must_use]
    pub fn oref(&mut self) -> OrefW<OisrSpec> {
        OrefW::new(self, 4)
    }
    #[doc = "Bit 16 - RORFF"]
    #[inline(always)]
    #[must_use]
    pub fn rorff(&mut self) -> RorffW<OisrSpec> {
        RorffW::new(self, 16)
    }
    #[doc = "Bit 17 - PPEF"]
    #[inline(always)]
    #[must_use]
    pub fn ppef(&mut self) -> PpefW<OisrSpec> {
        PpefW::new(self, 17)
    }
}
#[doc = "OISR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oisr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oisr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OisrSpec;
impl crate::RegisterSpec for OisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oisr::R`](R) reader structure"]
impl crate::Readable for OisrSpec {}
#[doc = "`write(|w| ..)` method takes [`oisr::W`](W) writer structure"]
impl crate::Writable for OisrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OISR to value 0"]
impl crate::Resettable for OisrSpec {
    const RESET_VALUE: u32 = 0;
}
