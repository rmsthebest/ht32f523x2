#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `WDTUF` reader - WDTUF"]
pub type WdtufR = crate::BitReader;
#[doc = "Field `WDTUF` writer - WDTUF"]
pub type WdtufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTERR` reader - WDTERR"]
pub type WdterrR = crate::BitReader;
#[doc = "Field `WDTERR` writer - WDTERR"]
pub type WdterrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - WDTUF"]
    #[inline(always)]
    pub fn wdtuf(&self) -> WdtufR {
        WdtufR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDTERR"]
    #[inline(always)]
    pub fn wdterr(&self) -> WdterrR {
        WdterrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDTUF"]
    #[inline(always)]
    #[must_use]
    pub fn wdtuf(&mut self) -> WdtufW<SrSpec> {
        WdtufW::new(self, 0)
    }
    #[doc = "Bit 1 - WDTERR"]
    #[inline(always)]
    #[must_use]
    pub fn wdterr(&mut self) -> WdterrW<SrSpec> {
        WdterrW::new(self, 1)
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
