#[doc = "Register `PUR` reader"]
pub type R = crate::R<PurSpec>;
#[doc = "Register `PUR` writer"]
pub type W = crate::W<PurSpec>;
#[doc = "Field `PU0` reader - PU0"]
pub type Pu0R = crate::BitReader;
#[doc = "Field `PU0` writer - PU0"]
pub type Pu0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU1` reader - PU1"]
pub type Pu1R = crate::BitReader;
#[doc = "Field `PU1` writer - PU1"]
pub type Pu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU2` reader - PU2"]
pub type Pu2R = crate::BitReader;
#[doc = "Field `PU2` writer - PU2"]
pub type Pu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU3` reader - PU3"]
pub type Pu3R = crate::BitReader;
#[doc = "Field `PU3` writer - PU3"]
pub type Pu3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU4` reader - PU4"]
pub type Pu4R = crate::BitReader;
#[doc = "Field `PU4` writer - PU4"]
pub type Pu4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU5` reader - PU5"]
pub type Pu5R = crate::BitReader;
#[doc = "Field `PU5` writer - PU5"]
pub type Pu5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU6` reader - PU6"]
pub type Pu6R = crate::BitReader;
#[doc = "Field `PU6` writer - PU6"]
pub type Pu6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU7` reader - PU7"]
pub type Pu7R = crate::BitReader;
#[doc = "Field `PU7` writer - PU7"]
pub type Pu7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU8` reader - PU8"]
pub type Pu8R = crate::BitReader;
#[doc = "Field `PU8` writer - PU8"]
pub type Pu8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU9` reader - PU9"]
pub type Pu9R = crate::BitReader;
#[doc = "Field `PU9` writer - PU9"]
pub type Pu9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU10` reader - PU10"]
pub type Pu10R = crate::BitReader;
#[doc = "Field `PU10` writer - PU10"]
pub type Pu10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU11` reader - PU11"]
pub type Pu11R = crate::BitReader;
#[doc = "Field `PU11` writer - PU11"]
pub type Pu11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU12` reader - PU12"]
pub type Pu12R = crate::BitReader;
#[doc = "Field `PU12` writer - PU12"]
pub type Pu12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU13` reader - PU13"]
pub type Pu13R = crate::BitReader;
#[doc = "Field `PU13` writer - PU13"]
pub type Pu13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU14` reader - PU14"]
pub type Pu14R = crate::BitReader;
#[doc = "Field `PU14` writer - PU14"]
pub type Pu14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU15` reader - PU15"]
pub type Pu15R = crate::BitReader;
#[doc = "Field `PU15` writer - PU15"]
pub type Pu15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PU0"]
    #[inline(always)]
    pub fn pu0(&self) -> Pu0R {
        Pu0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PU1"]
    #[inline(always)]
    pub fn pu1(&self) -> Pu1R {
        Pu1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PU2"]
    #[inline(always)]
    pub fn pu2(&self) -> Pu2R {
        Pu2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PU3"]
    #[inline(always)]
    pub fn pu3(&self) -> Pu3R {
        Pu3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PU4"]
    #[inline(always)]
    pub fn pu4(&self) -> Pu4R {
        Pu4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PU5"]
    #[inline(always)]
    pub fn pu5(&self) -> Pu5R {
        Pu5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PU6"]
    #[inline(always)]
    pub fn pu6(&self) -> Pu6R {
        Pu6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PU7"]
    #[inline(always)]
    pub fn pu7(&self) -> Pu7R {
        Pu7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PU8"]
    #[inline(always)]
    pub fn pu8(&self) -> Pu8R {
        Pu8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PU9"]
    #[inline(always)]
    pub fn pu9(&self) -> Pu9R {
        Pu9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PU10"]
    #[inline(always)]
    pub fn pu10(&self) -> Pu10R {
        Pu10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PU11"]
    #[inline(always)]
    pub fn pu11(&self) -> Pu11R {
        Pu11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PU12"]
    #[inline(always)]
    pub fn pu12(&self) -> Pu12R {
        Pu12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PU13"]
    #[inline(always)]
    pub fn pu13(&self) -> Pu13R {
        Pu13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PU14"]
    #[inline(always)]
    pub fn pu14(&self) -> Pu14R {
        Pu14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PU15"]
    #[inline(always)]
    pub fn pu15(&self) -> Pu15R {
        Pu15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PU0"]
    #[inline(always)]
    #[must_use]
    pub fn pu0(&mut self) -> Pu0W<PurSpec> {
        Pu0W::new(self, 0)
    }
    #[doc = "Bit 1 - PU1"]
    #[inline(always)]
    #[must_use]
    pub fn pu1(&mut self) -> Pu1W<PurSpec> {
        Pu1W::new(self, 1)
    }
    #[doc = "Bit 2 - PU2"]
    #[inline(always)]
    #[must_use]
    pub fn pu2(&mut self) -> Pu2W<PurSpec> {
        Pu2W::new(self, 2)
    }
    #[doc = "Bit 3 - PU3"]
    #[inline(always)]
    #[must_use]
    pub fn pu3(&mut self) -> Pu3W<PurSpec> {
        Pu3W::new(self, 3)
    }
    #[doc = "Bit 4 - PU4"]
    #[inline(always)]
    #[must_use]
    pub fn pu4(&mut self) -> Pu4W<PurSpec> {
        Pu4W::new(self, 4)
    }
    #[doc = "Bit 5 - PU5"]
    #[inline(always)]
    #[must_use]
    pub fn pu5(&mut self) -> Pu5W<PurSpec> {
        Pu5W::new(self, 5)
    }
    #[doc = "Bit 6 - PU6"]
    #[inline(always)]
    #[must_use]
    pub fn pu6(&mut self) -> Pu6W<PurSpec> {
        Pu6W::new(self, 6)
    }
    #[doc = "Bit 7 - PU7"]
    #[inline(always)]
    #[must_use]
    pub fn pu7(&mut self) -> Pu7W<PurSpec> {
        Pu7W::new(self, 7)
    }
    #[doc = "Bit 8 - PU8"]
    #[inline(always)]
    #[must_use]
    pub fn pu8(&mut self) -> Pu8W<PurSpec> {
        Pu8W::new(self, 8)
    }
    #[doc = "Bit 9 - PU9"]
    #[inline(always)]
    #[must_use]
    pub fn pu9(&mut self) -> Pu9W<PurSpec> {
        Pu9W::new(self, 9)
    }
    #[doc = "Bit 10 - PU10"]
    #[inline(always)]
    #[must_use]
    pub fn pu10(&mut self) -> Pu10W<PurSpec> {
        Pu10W::new(self, 10)
    }
    #[doc = "Bit 11 - PU11"]
    #[inline(always)]
    #[must_use]
    pub fn pu11(&mut self) -> Pu11W<PurSpec> {
        Pu11W::new(self, 11)
    }
    #[doc = "Bit 12 - PU12"]
    #[inline(always)]
    #[must_use]
    pub fn pu12(&mut self) -> Pu12W<PurSpec> {
        Pu12W::new(self, 12)
    }
    #[doc = "Bit 13 - PU13"]
    #[inline(always)]
    #[must_use]
    pub fn pu13(&mut self) -> Pu13W<PurSpec> {
        Pu13W::new(self, 13)
    }
    #[doc = "Bit 14 - PU14"]
    #[inline(always)]
    #[must_use]
    pub fn pu14(&mut self) -> Pu14W<PurSpec> {
        Pu14W::new(self, 14)
    }
    #[doc = "Bit 15 - PU15"]
    #[inline(always)]
    #[must_use]
    pub fn pu15(&mut self) -> Pu15W<PurSpec> {
        Pu15W::new(self, 15)
    }
}
#[doc = "PUR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PurSpec;
impl crate::RegisterSpec for PurSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pur::R`](R) reader structure"]
impl crate::Readable for PurSpec {}
#[doc = "`write(|w| ..)` method takes [`pur::W`](W) writer structure"]
impl crate::Writable for PurSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUR to value 0"]
impl crate::Resettable for PurSpec {
    const RESET_VALUE: u32 = 0;
}
