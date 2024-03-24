#[doc = "Register `LST1` reader"]
pub type R = crate::R<Lst1Spec>;
#[doc = "Register `LST1` writer"]
pub type W = crate::W<Lst1Spec>;
#[doc = "Field `ADSEQ4` reader - ADSEQ4"]
pub type Adseq4R = crate::FieldReader;
#[doc = "Field `ADSEQ4` writer - ADSEQ4"]
pub type Adseq4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADSEQ5` reader - ADSEQ5"]
pub type Adseq5R = crate::FieldReader;
#[doc = "Field `ADSEQ5` writer - ADSEQ5"]
pub type Adseq5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADSEQ6` reader - ADSEQ6"]
pub type Adseq6R = crate::FieldReader;
#[doc = "Field `ADSEQ6` writer - ADSEQ6"]
pub type Adseq6W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADSEQ7` reader - ADSEQ7"]
pub type Adseq7R = crate::FieldReader;
#[doc = "Field `ADSEQ7` writer - ADSEQ7"]
pub type Adseq7W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - ADSEQ4"]
    #[inline(always)]
    pub fn adseq4(&self) -> Adseq4R {
        Adseq4R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - ADSEQ5"]
    #[inline(always)]
    pub fn adseq5(&self) -> Adseq5R {
        Adseq5R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - ADSEQ6"]
    #[inline(always)]
    pub fn adseq6(&self) -> Adseq6R {
        Adseq6R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - ADSEQ7"]
    #[inline(always)]
    pub fn adseq7(&self) -> Adseq7R {
        Adseq7R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADSEQ4"]
    #[inline(always)]
    #[must_use]
    pub fn adseq4(&mut self) -> Adseq4W<Lst1Spec> {
        Adseq4W::new(self, 0)
    }
    #[doc = "Bits 8:12 - ADSEQ5"]
    #[inline(always)]
    #[must_use]
    pub fn adseq5(&mut self) -> Adseq5W<Lst1Spec> {
        Adseq5W::new(self, 8)
    }
    #[doc = "Bits 16:20 - ADSEQ6"]
    #[inline(always)]
    #[must_use]
    pub fn adseq6(&mut self) -> Adseq6W<Lst1Spec> {
        Adseq6W::new(self, 16)
    }
    #[doc = "Bits 24:28 - ADSEQ7"]
    #[inline(always)]
    #[must_use]
    pub fn adseq7(&mut self) -> Adseq7W<Lst1Spec> {
        Adseq7W::new(self, 24)
    }
}
#[doc = "LST1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lst1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lst1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lst1Spec;
impl crate::RegisterSpec for Lst1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lst1::R`](R) reader structure"]
impl crate::Readable for Lst1Spec {}
#[doc = "`write(|w| ..)` method takes [`lst1::W`](W) writer structure"]
impl crate::Writable for Lst1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LST1 to value 0"]
impl crate::Resettable for Lst1Spec {
    const RESET_VALUE: u32 = 0;
}
