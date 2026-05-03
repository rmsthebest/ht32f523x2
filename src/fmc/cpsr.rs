#[doc = "Register `CPSR` reader"]
pub type R = crate::R<CpsrSpec>;
#[doc = "Register `CPSR` writer"]
pub type W = crate::W<CpsrSpec>;
#[doc = "Field `CPSB` reader - CPSB"]
pub type CpsbR = crate::BitReader;
#[doc = "Field `CPSB` writer - CPSB"]
pub type CpsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBPSB` reader - OBPSB"]
pub type ObpsbR = crate::BitReader;
#[doc = "Field `OBPSB` writer - OBPSB"]
pub type ObpsbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CPSB"]
    #[inline(always)]
    pub fn cpsb(&self) -> CpsbR {
        CpsbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OBPSB"]
    #[inline(always)]
    pub fn obpsb(&self) -> ObpsbR {
        ObpsbR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPSB"]
    #[inline(always)]
    pub fn cpsb(&mut self) -> CpsbW<'_, CpsrSpec> {
        CpsbW::new(self, 0)
    }
    #[doc = "Bit 1 - OBPSB"]
    #[inline(always)]
    pub fn obpsb(&mut self) -> ObpsbW<'_, CpsrSpec> {
        ObpsbW::new(self, 1)
    }
}
#[doc = "CPSR\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpsrSpec;
impl crate::RegisterSpec for CpsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsr::R`](R) reader structure"]
impl crate::Readable for CpsrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsr::W`](W) writer structure"]
impl crate::Writable for CpsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPSR to value 0"]
impl crate::Resettable for CpsrSpec {}
