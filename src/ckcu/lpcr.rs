#[doc = "Register `LPCR` reader"]
pub type R = crate::R<LpcrSpec>;
#[doc = "Register `LPCR` writer"]
pub type W = crate::W<LpcrSpec>;
#[doc = "Field `BKISO` reader - BKISO"]
pub type BkisoR = crate::BitReader;
#[doc = "Field `BKISO` writer - BKISO"]
pub type BkisoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSLEEP` reader - USBSLEEP"]
pub type UsbsleepR = crate::BitReader;
#[doc = "Field `USBSLEEP` writer - USBSLEEP"]
pub type UsbsleepW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BKISO"]
    #[inline(always)]
    pub fn bkiso(&self) -> BkisoR {
        BkisoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - USBSLEEP"]
    #[inline(always)]
    pub fn usbsleep(&self) -> UsbsleepR {
        UsbsleepR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BKISO"]
    #[inline(always)]
    pub fn bkiso(&mut self) -> BkisoW<'_, LpcrSpec> {
        BkisoW::new(self, 0)
    }
    #[doc = "Bit 8 - USBSLEEP"]
    #[inline(always)]
    pub fn usbsleep(&mut self) -> UsbsleepW<'_, LpcrSpec> {
        UsbsleepW::new(self, 8)
    }
}
#[doc = "LPCR\n\nYou can [`read`](crate::Reg::read) this register and get [`lpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpcrSpec;
impl crate::RegisterSpec for LpcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpcr::R`](R) reader structure"]
impl crate::Readable for LpcrSpec {}
#[doc = "`write(|w| ..)` method takes [`lpcr::W`](W) writer structure"]
impl crate::Writable for LpcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPCR to value 0"]
impl crate::Resettable for LpcrSpec {}
