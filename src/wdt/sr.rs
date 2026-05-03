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
    pub fn wdtuf(&mut self) -> WdtufW<'_, SrSpec> {
        WdtufW::new(self, 0)
    }
    #[doc = "Bit 1 - WDTERR"]
    #[inline(always)]
    pub fn wdterr(&mut self) -> WdterrW<'_, SrSpec> {
        WdterrW::new(self, 1)
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
