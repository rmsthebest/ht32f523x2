#[doc = "Register `PWRCU_BAKSR` reader"]
pub type R = crate::R<PwrcuBaksrSpec>;
#[doc = "Register `PWRCU_BAKSR` writer"]
pub type W = crate::W<PwrcuBaksrSpec>;
#[doc = "Field `BAKPORF` reader - BAKPORF"]
pub type BakporfR = crate::BitReader;
#[doc = "Field `BAKPORF` writer - BAKPORF"]
pub type BakporfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDF` reader - PDF"]
pub type PdfR = crate::BitReader;
#[doc = "Field `PDF` writer - PDF"]
pub type PdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPF` reader - WUPF"]
pub type WupfR = crate::BitReader;
#[doc = "Field `WUPF` writer - WUPF"]
pub type WupfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BAKPORF"]
    #[inline(always)]
    pub fn bakporf(&self) -> BakporfR {
        BakporfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDF"]
    #[inline(always)]
    pub fn pdf(&self) -> PdfR {
        PdfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - WUPF"]
    #[inline(always)]
    pub fn wupf(&self) -> WupfR {
        WupfR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BAKPORF"]
    #[inline(always)]
    #[must_use]
    pub fn bakporf(&mut self) -> BakporfW<PwrcuBaksrSpec> {
        BakporfW::new(self, 0)
    }
    #[doc = "Bit 1 - PDF"]
    #[inline(always)]
    #[must_use]
    pub fn pdf(&mut self) -> PdfW<PwrcuBaksrSpec> {
        PdfW::new(self, 1)
    }
    #[doc = "Bit 8 - WUPF"]
    #[inline(always)]
    #[must_use]
    pub fn wupf(&mut self) -> WupfW<PwrcuBaksrSpec> {
        WupfW::new(self, 8)
    }
}
#[doc = "PWRCU_BAKSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcu_baksr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcu_baksr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrcuBaksrSpec;
impl crate::RegisterSpec for PwrcuBaksrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrcu_baksr::R`](R) reader structure"]
impl crate::Readable for PwrcuBaksrSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrcu_baksr::W`](W) writer structure"]
impl crate::Writable for PwrcuBaksrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRCU_BAKSR to value 0"]
impl crate::Resettable for PwrcuBaksrSpec {
    const RESET_VALUE: u32 = 0;
}
