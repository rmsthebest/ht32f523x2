#[doc = "Register `LST0` reader"]
pub type R = crate::R<Lst0Spec>;
#[doc = "Register `LST0` writer"]
pub type W = crate::W<Lst0Spec>;
#[doc = "Field `ADSEQ0` reader - ADSEQ0"]
pub type Adseq0R = crate::FieldReader;
#[doc = "Field `ADSEQ0` writer - ADSEQ0"]
pub type Adseq0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADSEQ1` reader - ADSEQ1"]
pub type Adseq1R = crate::FieldReader;
#[doc = "Field `ADSEQ1` writer - ADSEQ1"]
pub type Adseq1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADSEQ2` reader - ADSEQ2"]
pub type Adseq2R = crate::FieldReader;
#[doc = "Field `ADSEQ2` writer - ADSEQ2"]
pub type Adseq2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADSEQ3` reader - ADSEQ3"]
pub type Adseq3R = crate::FieldReader;
#[doc = "Field `ADSEQ3` writer - ADSEQ3"]
pub type Adseq3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - ADSEQ0"]
    #[inline(always)]
    pub fn adseq0(&self) -> Adseq0R {
        Adseq0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - ADSEQ1"]
    #[inline(always)]
    pub fn adseq1(&self) -> Adseq1R {
        Adseq1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - ADSEQ2"]
    #[inline(always)]
    pub fn adseq2(&self) -> Adseq2R {
        Adseq2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - ADSEQ3"]
    #[inline(always)]
    pub fn adseq3(&self) -> Adseq3R {
        Adseq3R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADSEQ0"]
    #[inline(always)]
    #[must_use]
    pub fn adseq0(&mut self) -> Adseq0W<Lst0Spec> {
        Adseq0W::new(self, 0)
    }
    #[doc = "Bits 8:12 - ADSEQ1"]
    #[inline(always)]
    #[must_use]
    pub fn adseq1(&mut self) -> Adseq1W<Lst0Spec> {
        Adseq1W::new(self, 8)
    }
    #[doc = "Bits 16:20 - ADSEQ2"]
    #[inline(always)]
    #[must_use]
    pub fn adseq2(&mut self) -> Adseq2W<Lst0Spec> {
        Adseq2W::new(self, 16)
    }
    #[doc = "Bits 24:28 - ADSEQ3"]
    #[inline(always)]
    #[must_use]
    pub fn adseq3(&mut self) -> Adseq3W<Lst0Spec> {
        Adseq3W::new(self, 24)
    }
}
#[doc = "LST0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lst0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lst0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lst0Spec;
impl crate::RegisterSpec for Lst0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lst0::R`](R) reader structure"]
impl crate::Readable for Lst0Spec {}
#[doc = "`write(|w| ..)` method takes [`lst0::W`](W) writer structure"]
impl crate::Writable for Lst0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LST0 to value 0"]
impl crate::Resettable for Lst0Spec {
    const RESET_VALUE: u32 = 0;
}
