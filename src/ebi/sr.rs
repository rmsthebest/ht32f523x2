#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `EBIBUSY` reader - EBIBUSY"]
pub type EbibusyR = crate::BitReader;
#[doc = "Field `EBIBUSY` writer - EBIBUSY"]
pub type EbibusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EBISMRST` reader - EBISMRST"]
pub type EbismrstR = crate::BitReader;
#[doc = "Field `EBISMRST` writer - EBISMRST"]
pub type EbismrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EBIBUSY"]
    #[inline(always)]
    pub fn ebibusy(&self) -> EbibusyR {
        EbibusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - EBISMRST"]
    #[inline(always)]
    pub fn ebismrst(&self) -> EbismrstR {
        EbismrstR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EBIBUSY"]
    #[inline(always)]
    #[must_use]
    pub fn ebibusy(&mut self) -> EbibusyW<SrSpec> {
        EbibusyW::new(self, 0)
    }
    #[doc = "Bit 8 - EBISMRST"]
    #[inline(always)]
    #[must_use]
    pub fn ebismrst(&mut self) -> EbismrstW<SrSpec> {
        EbismrstW::new(self, 8)
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
