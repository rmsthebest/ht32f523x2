#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `DFL` reader - DFL"]
pub type DflR = crate::FieldReader;
#[doc = "Field `DFL` writer - DFL"]
pub type DflW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FORMAT` reader - FORMAT"]
pub type FormatR = crate::FieldReader;
#[doc = "Field `FORMAT` writer - FORMAT"]
pub type FormatW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SELAP` reader - SELAP"]
pub type SelapR = crate::BitReader;
#[doc = "Field `SELAP` writer - SELAP"]
pub type SelapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIRSTBIT` reader - FIRSTBIT"]
pub type FirstbitR = crate::BitReader;
#[doc = "Field `FIRSTBIT` writer - FIRSTBIT"]
pub type FirstbitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SELM` reader - SELM"]
pub type SelmR = crate::BitReader;
#[doc = "Field `SELM` writer - SELM"]
pub type SelmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - MODE"]
pub type ModeR = crate::BitReader;
#[doc = "Field `MODE` writer - MODE"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - DFL"]
    #[inline(always)]
    pub fn dfl(&self) -> DflR {
        DflR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - FORMAT"]
    #[inline(always)]
    pub fn format(&self) -> FormatR {
        FormatR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - SELAP"]
    #[inline(always)]
    pub fn selap(&self) -> SelapR {
        SelapR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FIRSTBIT"]
    #[inline(always)]
    pub fn firstbit(&self) -> FirstbitR {
        FirstbitR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SELM"]
    #[inline(always)]
    pub fn selm(&self) -> SelmR {
        SelmR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MODE"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - DFL"]
    #[inline(always)]
    #[must_use]
    pub fn dfl(&mut self) -> DflW<Cr1Spec> {
        DflW::new(self, 0)
    }
    #[doc = "Bits 8:10 - FORMAT"]
    #[inline(always)]
    #[must_use]
    pub fn format(&mut self) -> FormatW<Cr1Spec> {
        FormatW::new(self, 8)
    }
    #[doc = "Bit 11 - SELAP"]
    #[inline(always)]
    #[must_use]
    pub fn selap(&mut self) -> SelapW<Cr1Spec> {
        SelapW::new(self, 11)
    }
    #[doc = "Bit 12 - FIRSTBIT"]
    #[inline(always)]
    #[must_use]
    pub fn firstbit(&mut self) -> FirstbitW<Cr1Spec> {
        FirstbitW::new(self, 12)
    }
    #[doc = "Bit 13 - SELM"]
    #[inline(always)]
    #[must_use]
    pub fn selm(&mut self) -> SelmW<Cr1Spec> {
        SelmW::new(self, 13)
    }
    #[doc = "Bit 14 - MODE"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<Cr1Spec> {
        ModeW::new(self, 14)
    }
}
#[doc = "CR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {
    const RESET_VALUE: u32 = 0;
}
