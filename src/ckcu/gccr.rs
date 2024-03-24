#[doc = "Register `GCCR` reader"]
pub type R = crate::R<GccrSpec>;
#[doc = "Register `GCCR` writer"]
pub type W = crate::W<GccrSpec>;
#[doc = "Field `SW` reader - SW"]
pub type SwR = crate::FieldReader;
#[doc = "Field `SW` writer - SW"]
pub type SwW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HSEGAIN` reader - HSEGAIN"]
pub type HsegainR = crate::BitReader;
#[doc = "Field `HSEGAIN` writer - HSEGAIN"]
pub type HsegainW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLEN` reader - PLLEN"]
pub type PllenR = crate::BitReader;
#[doc = "Field `PLLEN` writer - PLLEN"]
pub type PllenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEEN` reader - HSEEN"]
pub type HseenR = crate::BitReader;
#[doc = "Field `HSEEN` writer - HSEEN"]
pub type HseenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIEN` reader - HSIEN"]
pub type HsienR = crate::BitReader;
#[doc = "Field `HSIEN` writer - HSIEN"]
pub type HsienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKMEN` reader - CKMEN"]
pub type CkmenR = crate::BitReader;
#[doc = "Field `CKMEN` writer - CKMEN"]
pub type CkmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSRCEN` reader - PSRCEN"]
pub type PsrcenR = crate::BitReader;
#[doc = "Field `PSRCEN` writer - PSRCEN"]
pub type PsrcenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - SW"]
    #[inline(always)]
    pub fn sw(&self) -> SwR {
        SwR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - HSEGAIN"]
    #[inline(always)]
    pub fn hsegain(&self) -> HsegainR {
        HsegainR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PLLEN"]
    #[inline(always)]
    pub fn pllen(&self) -> PllenR {
        PllenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSEEN"]
    #[inline(always)]
    pub fn hseen(&self) -> HseenR {
        HseenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSIEN"]
    #[inline(always)]
    pub fn hsien(&self) -> HsienR {
        HsienR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - CKMEN"]
    #[inline(always)]
    pub fn ckmen(&self) -> CkmenR {
        CkmenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PSRCEN"]
    #[inline(always)]
    pub fn psrcen(&self) -> PsrcenR {
        PsrcenR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SW"]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SwW<GccrSpec> {
        SwW::new(self, 0)
    }
    #[doc = "Bit 8 - HSEGAIN"]
    #[inline(always)]
    #[must_use]
    pub fn hsegain(&mut self) -> HsegainW<GccrSpec> {
        HsegainW::new(self, 8)
    }
    #[doc = "Bit 9 - PLLEN"]
    #[inline(always)]
    #[must_use]
    pub fn pllen(&mut self) -> PllenW<GccrSpec> {
        PllenW::new(self, 9)
    }
    #[doc = "Bit 10 - HSEEN"]
    #[inline(always)]
    #[must_use]
    pub fn hseen(&mut self) -> HseenW<GccrSpec> {
        HseenW::new(self, 10)
    }
    #[doc = "Bit 11 - HSIEN"]
    #[inline(always)]
    #[must_use]
    pub fn hsien(&mut self) -> HsienW<GccrSpec> {
        HsienW::new(self, 11)
    }
    #[doc = "Bit 16 - CKMEN"]
    #[inline(always)]
    #[must_use]
    pub fn ckmen(&mut self) -> CkmenW<GccrSpec> {
        CkmenW::new(self, 16)
    }
    #[doc = "Bit 17 - PSRCEN"]
    #[inline(always)]
    #[must_use]
    pub fn psrcen(&mut self) -> PsrcenW<GccrSpec> {
        PsrcenW::new(self, 17)
    }
}
#[doc = "GCCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GccrSpec;
impl crate::RegisterSpec for GccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gccr::R`](R) reader structure"]
impl crate::Readable for GccrSpec {}
#[doc = "`write(|w| ..)` method takes [`gccr::W`](W) writer structure"]
impl crate::Writable for GccrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCCR to value 0"]
impl crate::Resettable for GccrSpec {
    const RESET_VALUE: u32 = 0;
}
