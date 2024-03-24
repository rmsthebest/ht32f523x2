#[doc = "Register `ODR` reader"]
pub type R = crate::R<OdrSpec>;
#[doc = "Register `ODR` writer"]
pub type W = crate::W<OdrSpec>;
#[doc = "Field `OD0` reader - OD0"]
pub type Od0R = crate::BitReader;
#[doc = "Field `OD0` writer - OD0"]
pub type Od0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD1` reader - OD1"]
pub type Od1R = crate::BitReader;
#[doc = "Field `OD1` writer - OD1"]
pub type Od1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD2` reader - OD2"]
pub type Od2R = crate::BitReader;
#[doc = "Field `OD2` writer - OD2"]
pub type Od2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD3` reader - OD3"]
pub type Od3R = crate::BitReader;
#[doc = "Field `OD3` writer - OD3"]
pub type Od3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD4` reader - OD4"]
pub type Od4R = crate::BitReader;
#[doc = "Field `OD4` writer - OD4"]
pub type Od4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD5` reader - OD5"]
pub type Od5R = crate::BitReader;
#[doc = "Field `OD5` writer - OD5"]
pub type Od5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD6` reader - OD6"]
pub type Od6R = crate::BitReader;
#[doc = "Field `OD6` writer - OD6"]
pub type Od6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD7` reader - OD7"]
pub type Od7R = crate::BitReader;
#[doc = "Field `OD7` writer - OD7"]
pub type Od7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD8` reader - OD8"]
pub type Od8R = crate::BitReader;
#[doc = "Field `OD8` writer - OD8"]
pub type Od8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD9` reader - OD9"]
pub type Od9R = crate::BitReader;
#[doc = "Field `OD9` writer - OD9"]
pub type Od9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD10` reader - OD10"]
pub type Od10R = crate::BitReader;
#[doc = "Field `OD10` writer - OD10"]
pub type Od10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD11` reader - OD11"]
pub type Od11R = crate::BitReader;
#[doc = "Field `OD11` writer - OD11"]
pub type Od11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD12` reader - OD12"]
pub type Od12R = crate::BitReader;
#[doc = "Field `OD12` writer - OD12"]
pub type Od12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD13` reader - OD13"]
pub type Od13R = crate::BitReader;
#[doc = "Field `OD13` writer - OD13"]
pub type Od13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD14` reader - OD14"]
pub type Od14R = crate::BitReader;
#[doc = "Field `OD14` writer - OD14"]
pub type Od14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD15` reader - OD15"]
pub type Od15R = crate::BitReader;
#[doc = "Field `OD15` writer - OD15"]
pub type Od15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - OD0"]
    #[inline(always)]
    pub fn od0(&self) -> Od0R {
        Od0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OD1"]
    #[inline(always)]
    pub fn od1(&self) -> Od1R {
        Od1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OD2"]
    #[inline(always)]
    pub fn od2(&self) -> Od2R {
        Od2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OD3"]
    #[inline(always)]
    pub fn od3(&self) -> Od3R {
        Od3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OD4"]
    #[inline(always)]
    pub fn od4(&self) -> Od4R {
        Od4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OD5"]
    #[inline(always)]
    pub fn od5(&self) -> Od5R {
        Od5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OD6"]
    #[inline(always)]
    pub fn od6(&self) -> Od6R {
        Od6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - OD7"]
    #[inline(always)]
    pub fn od7(&self) -> Od7R {
        Od7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OD8"]
    #[inline(always)]
    pub fn od8(&self) -> Od8R {
        Od8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OD9"]
    #[inline(always)]
    pub fn od9(&self) -> Od9R {
        Od9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OD10"]
    #[inline(always)]
    pub fn od10(&self) -> Od10R {
        Od10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OD11"]
    #[inline(always)]
    pub fn od11(&self) -> Od11R {
        Od11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OD12"]
    #[inline(always)]
    pub fn od12(&self) -> Od12R {
        Od12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - OD13"]
    #[inline(always)]
    pub fn od13(&self) -> Od13R {
        Od13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - OD14"]
    #[inline(always)]
    pub fn od14(&self) -> Od14R {
        Od14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - OD15"]
    #[inline(always)]
    pub fn od15(&self) -> Od15R {
        Od15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OD0"]
    #[inline(always)]
    #[must_use]
    pub fn od0(&mut self) -> Od0W<OdrSpec> {
        Od0W::new(self, 0)
    }
    #[doc = "Bit 1 - OD1"]
    #[inline(always)]
    #[must_use]
    pub fn od1(&mut self) -> Od1W<OdrSpec> {
        Od1W::new(self, 1)
    }
    #[doc = "Bit 2 - OD2"]
    #[inline(always)]
    #[must_use]
    pub fn od2(&mut self) -> Od2W<OdrSpec> {
        Od2W::new(self, 2)
    }
    #[doc = "Bit 3 - OD3"]
    #[inline(always)]
    #[must_use]
    pub fn od3(&mut self) -> Od3W<OdrSpec> {
        Od3W::new(self, 3)
    }
    #[doc = "Bit 4 - OD4"]
    #[inline(always)]
    #[must_use]
    pub fn od4(&mut self) -> Od4W<OdrSpec> {
        Od4W::new(self, 4)
    }
    #[doc = "Bit 5 - OD5"]
    #[inline(always)]
    #[must_use]
    pub fn od5(&mut self) -> Od5W<OdrSpec> {
        Od5W::new(self, 5)
    }
    #[doc = "Bit 6 - OD6"]
    #[inline(always)]
    #[must_use]
    pub fn od6(&mut self) -> Od6W<OdrSpec> {
        Od6W::new(self, 6)
    }
    #[doc = "Bit 7 - OD7"]
    #[inline(always)]
    #[must_use]
    pub fn od7(&mut self) -> Od7W<OdrSpec> {
        Od7W::new(self, 7)
    }
    #[doc = "Bit 8 - OD8"]
    #[inline(always)]
    #[must_use]
    pub fn od8(&mut self) -> Od8W<OdrSpec> {
        Od8W::new(self, 8)
    }
    #[doc = "Bit 9 - OD9"]
    #[inline(always)]
    #[must_use]
    pub fn od9(&mut self) -> Od9W<OdrSpec> {
        Od9W::new(self, 9)
    }
    #[doc = "Bit 10 - OD10"]
    #[inline(always)]
    #[must_use]
    pub fn od10(&mut self) -> Od10W<OdrSpec> {
        Od10W::new(self, 10)
    }
    #[doc = "Bit 11 - OD11"]
    #[inline(always)]
    #[must_use]
    pub fn od11(&mut self) -> Od11W<OdrSpec> {
        Od11W::new(self, 11)
    }
    #[doc = "Bit 12 - OD12"]
    #[inline(always)]
    #[must_use]
    pub fn od12(&mut self) -> Od12W<OdrSpec> {
        Od12W::new(self, 12)
    }
    #[doc = "Bit 13 - OD13"]
    #[inline(always)]
    #[must_use]
    pub fn od13(&mut self) -> Od13W<OdrSpec> {
        Od13W::new(self, 13)
    }
    #[doc = "Bit 14 - OD14"]
    #[inline(always)]
    #[must_use]
    pub fn od14(&mut self) -> Od14W<OdrSpec> {
        Od14W::new(self, 14)
    }
    #[doc = "Bit 15 - OD15"]
    #[inline(always)]
    #[must_use]
    pub fn od15(&mut self) -> Od15W<OdrSpec> {
        Od15W::new(self, 15)
    }
}
#[doc = "ODR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OdrSpec;
impl crate::RegisterSpec for OdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odr::R`](R) reader structure"]
impl crate::Readable for OdrSpec {}
#[doc = "`write(|w| ..)` method takes [`odr::W`](W) writer structure"]
impl crate::Writable for OdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ODR to value 0"]
impl crate::Resettable for OdrSpec {
    const RESET_VALUE: u32 = 0;
}
