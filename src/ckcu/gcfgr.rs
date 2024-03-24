#[doc = "Register `GCFGR` reader"]
pub type R = crate::R<GcfgrSpec>;
#[doc = "Register `GCFGR` writer"]
pub type W = crate::W<GcfgrSpec>;
#[doc = "Field `CKOUTSRC` reader - CKOUTSRC"]
pub type CkoutsrcR = crate::FieldReader;
#[doc = "Field `CKOUTSRC` writer - CKOUTSRC"]
pub type CkoutsrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLLSRC` reader - PLLSRC"]
pub type PllsrcR = crate::BitReader;
#[doc = "Field `PLLSRC` writer - PLLSRC"]
pub type PllsrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKREFPRE` reader - CKREFPRE"]
pub type CkrefpreR = crate::FieldReader;
#[doc = "Field `CKREFPRE` writer - CKREFPRE"]
pub type CkrefpreW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `USBPRE` reader - USBPRE"]
pub type UsbpreR = crate::FieldReader;
#[doc = "Field `USBPRE` writer - USBPRE"]
pub type UsbpreW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPMOD` reader - LPMOD"]
pub type LpmodR = crate::FieldReader;
#[doc = "Field `LPMOD` writer - LPMOD"]
pub type LpmodW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - CKOUTSRC"]
    #[inline(always)]
    pub fn ckoutsrc(&self) -> CkoutsrcR {
        CkoutsrcR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - PLLSRC"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PllsrcR {
        PllsrcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 11:15 - CKREFPRE"]
    #[inline(always)]
    pub fn ckrefpre(&self) -> CkrefpreR {
        CkrefpreR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 22:23 - USBPRE"]
    #[inline(always)]
    pub fn usbpre(&self) -> UsbpreR {
        UsbpreR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 29:31 - LPMOD"]
    #[inline(always)]
    pub fn lpmod(&self) -> LpmodR {
        LpmodR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CKOUTSRC"]
    #[inline(always)]
    #[must_use]
    pub fn ckoutsrc(&mut self) -> CkoutsrcW<GcfgrSpec> {
        CkoutsrcW::new(self, 0)
    }
    #[doc = "Bit 8 - PLLSRC"]
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PllsrcW<GcfgrSpec> {
        PllsrcW::new(self, 8)
    }
    #[doc = "Bits 11:15 - CKREFPRE"]
    #[inline(always)]
    #[must_use]
    pub fn ckrefpre(&mut self) -> CkrefpreW<GcfgrSpec> {
        CkrefpreW::new(self, 11)
    }
    #[doc = "Bits 22:23 - USBPRE"]
    #[inline(always)]
    #[must_use]
    pub fn usbpre(&mut self) -> UsbpreW<GcfgrSpec> {
        UsbpreW::new(self, 22)
    }
    #[doc = "Bits 29:31 - LPMOD"]
    #[inline(always)]
    #[must_use]
    pub fn lpmod(&mut self) -> LpmodW<GcfgrSpec> {
        LpmodW::new(self, 29)
    }
}
#[doc = "GCFGR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcfgrSpec;
impl crate::RegisterSpec for GcfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcfgr::R`](R) reader structure"]
impl crate::Readable for GcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`gcfgr::W`](W) writer structure"]
impl crate::Writable for GcfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCFGR to value 0"]
impl crate::Resettable for GcfgrSpec {
    const RESET_VALUE: u32 = 0;
}
