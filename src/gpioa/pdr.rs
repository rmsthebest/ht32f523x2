#[doc = "Register `PDR` reader"]
pub type R = crate::R<PdrSpec>;
#[doc = "Register `PDR` writer"]
pub type W = crate::W<PdrSpec>;
#[doc = "Field `PD0` reader - PD0"]
pub type Pd0R = crate::BitReader;
#[doc = "Field `PD0` writer - PD0"]
pub type Pd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD1` reader - PD1"]
pub type Pd1R = crate::BitReader;
#[doc = "Field `PD1` writer - PD1"]
pub type Pd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD2` reader - PD2"]
pub type Pd2R = crate::BitReader;
#[doc = "Field `PD2` writer - PD2"]
pub type Pd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD3` reader - PD3"]
pub type Pd3R = crate::BitReader;
#[doc = "Field `PD3` writer - PD3"]
pub type Pd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD4` reader - PD4"]
pub type Pd4R = crate::BitReader;
#[doc = "Field `PD4` writer - PD4"]
pub type Pd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD5` reader - PD5"]
pub type Pd5R = crate::BitReader;
#[doc = "Field `PD5` writer - PD5"]
pub type Pd5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD6` reader - PD6"]
pub type Pd6R = crate::BitReader;
#[doc = "Field `PD6` writer - PD6"]
pub type Pd6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD7` reader - PD7"]
pub type Pd7R = crate::BitReader;
#[doc = "Field `PD7` writer - PD7"]
pub type Pd7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD8` reader - PD8"]
pub type Pd8R = crate::BitReader;
#[doc = "Field `PD8` writer - PD8"]
pub type Pd8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD9` reader - PD9"]
pub type Pd9R = crate::BitReader;
#[doc = "Field `PD9` writer - PD9"]
pub type Pd9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD10` reader - PD10"]
pub type Pd10R = crate::BitReader;
#[doc = "Field `PD10` writer - PD10"]
pub type Pd10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD11` reader - PD11"]
pub type Pd11R = crate::BitReader;
#[doc = "Field `PD11` writer - PD11"]
pub type Pd11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD12` reader - PD12"]
pub type Pd12R = crate::BitReader;
#[doc = "Field `PD12` writer - PD12"]
pub type Pd12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD13` reader - PD13"]
pub type Pd13R = crate::BitReader;
#[doc = "Field `PD13` writer - PD13"]
pub type Pd13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD14` reader - PD14"]
pub type Pd14R = crate::BitReader;
#[doc = "Field `PD14` writer - PD14"]
pub type Pd14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD15` reader - PD15"]
pub type Pd15R = crate::BitReader;
#[doc = "Field `PD15` writer - PD15"]
pub type Pd15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PD0"]
    #[inline(always)]
    pub fn pd0(&self) -> Pd0R {
        Pd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PD1"]
    #[inline(always)]
    pub fn pd1(&self) -> Pd1R {
        Pd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PD2"]
    #[inline(always)]
    pub fn pd2(&self) -> Pd2R {
        Pd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PD3"]
    #[inline(always)]
    pub fn pd3(&self) -> Pd3R {
        Pd3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PD4"]
    #[inline(always)]
    pub fn pd4(&self) -> Pd4R {
        Pd4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PD5"]
    #[inline(always)]
    pub fn pd5(&self) -> Pd5R {
        Pd5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PD6"]
    #[inline(always)]
    pub fn pd6(&self) -> Pd6R {
        Pd6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PD7"]
    #[inline(always)]
    pub fn pd7(&self) -> Pd7R {
        Pd7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PD8"]
    #[inline(always)]
    pub fn pd8(&self) -> Pd8R {
        Pd8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PD9"]
    #[inline(always)]
    pub fn pd9(&self) -> Pd9R {
        Pd9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PD10"]
    #[inline(always)]
    pub fn pd10(&self) -> Pd10R {
        Pd10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PD11"]
    #[inline(always)]
    pub fn pd11(&self) -> Pd11R {
        Pd11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PD12"]
    #[inline(always)]
    pub fn pd12(&self) -> Pd12R {
        Pd12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PD13"]
    #[inline(always)]
    pub fn pd13(&self) -> Pd13R {
        Pd13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PD14"]
    #[inline(always)]
    pub fn pd14(&self) -> Pd14R {
        Pd14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PD15"]
    #[inline(always)]
    pub fn pd15(&self) -> Pd15R {
        Pd15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PD0"]
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> Pd0W<PdrSpec> {
        Pd0W::new(self, 0)
    }
    #[doc = "Bit 1 - PD1"]
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> Pd1W<PdrSpec> {
        Pd1W::new(self, 1)
    }
    #[doc = "Bit 2 - PD2"]
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> Pd2W<PdrSpec> {
        Pd2W::new(self, 2)
    }
    #[doc = "Bit 3 - PD3"]
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> Pd3W<PdrSpec> {
        Pd3W::new(self, 3)
    }
    #[doc = "Bit 4 - PD4"]
    #[inline(always)]
    #[must_use]
    pub fn pd4(&mut self) -> Pd4W<PdrSpec> {
        Pd4W::new(self, 4)
    }
    #[doc = "Bit 5 - PD5"]
    #[inline(always)]
    #[must_use]
    pub fn pd5(&mut self) -> Pd5W<PdrSpec> {
        Pd5W::new(self, 5)
    }
    #[doc = "Bit 6 - PD6"]
    #[inline(always)]
    #[must_use]
    pub fn pd6(&mut self) -> Pd6W<PdrSpec> {
        Pd6W::new(self, 6)
    }
    #[doc = "Bit 7 - PD7"]
    #[inline(always)]
    #[must_use]
    pub fn pd7(&mut self) -> Pd7W<PdrSpec> {
        Pd7W::new(self, 7)
    }
    #[doc = "Bit 8 - PD8"]
    #[inline(always)]
    #[must_use]
    pub fn pd8(&mut self) -> Pd8W<PdrSpec> {
        Pd8W::new(self, 8)
    }
    #[doc = "Bit 9 - PD9"]
    #[inline(always)]
    #[must_use]
    pub fn pd9(&mut self) -> Pd9W<PdrSpec> {
        Pd9W::new(self, 9)
    }
    #[doc = "Bit 10 - PD10"]
    #[inline(always)]
    #[must_use]
    pub fn pd10(&mut self) -> Pd10W<PdrSpec> {
        Pd10W::new(self, 10)
    }
    #[doc = "Bit 11 - PD11"]
    #[inline(always)]
    #[must_use]
    pub fn pd11(&mut self) -> Pd11W<PdrSpec> {
        Pd11W::new(self, 11)
    }
    #[doc = "Bit 12 - PD12"]
    #[inline(always)]
    #[must_use]
    pub fn pd12(&mut self) -> Pd12W<PdrSpec> {
        Pd12W::new(self, 12)
    }
    #[doc = "Bit 13 - PD13"]
    #[inline(always)]
    #[must_use]
    pub fn pd13(&mut self) -> Pd13W<PdrSpec> {
        Pd13W::new(self, 13)
    }
    #[doc = "Bit 14 - PD14"]
    #[inline(always)]
    #[must_use]
    pub fn pd14(&mut self) -> Pd14W<PdrSpec> {
        Pd14W::new(self, 14)
    }
    #[doc = "Bit 15 - PD15"]
    #[inline(always)]
    #[must_use]
    pub fn pd15(&mut self) -> Pd15W<PdrSpec> {
        Pd15W::new(self, 15)
    }
}
#[doc = "PDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdrSpec;
impl crate::RegisterSpec for PdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdr::R`](R) reader structure"]
impl crate::Readable for PdrSpec {}
#[doc = "`write(|w| ..)` method takes [`pdr::W`](W) writer structure"]
impl crate::Writable for PdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDR to value 0"]
impl crate::Resettable for PdrSpec {
    const RESET_VALUE: u32 = 0;
}
