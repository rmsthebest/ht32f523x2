#[doc = "Register `INER` reader"]
pub type R = crate::R<InerSpec>;
#[doc = "Register `INER` writer"]
pub type W = crate::W<InerSpec>;
#[doc = "Field `INEN0` reader - INEN0"]
pub type Inen0R = crate::BitReader;
#[doc = "Field `INEN0` writer - INEN0"]
pub type Inen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEN1` reader - INEN1"]
pub type Inen1R = crate::BitReader;
#[doc = "Field `INEN1` writer - INEN1"]
pub type Inen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEN2` reader - INEN2"]
pub type Inen2R = crate::BitReader;
#[doc = "Field `INEN2` writer - INEN2"]
pub type Inen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEN3` reader - INEN3"]
pub type Inen3R = crate::BitReader;
#[doc = "Field `INEN3` writer - INEN3"]
pub type Inen3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEN4` reader - INEN4"]
pub type Inen4R = crate::BitReader;
#[doc = "Field `INEN4` writer - INEN4"]
pub type Inen4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEN5` reader - INEN5"]
pub type Inen5R = crate::BitReader;
#[doc = "Field `INEN5` writer - INEN5"]
pub type Inen5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEN6` reader - INEN6"]
pub type Inen6R = crate::BitReader;
#[doc = "Field `INEN6` writer - INEN6"]
pub type Inen6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEN7` reader - INEN7"]
pub type Inen7R = crate::BitReader;
#[doc = "Field `INEN7` writer - INEN7"]
pub type Inen7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEN8` reader - INEN8"]
pub type Inen8R = crate::BitReader;
#[doc = "Field `INEN8` writer - INEN8"]
pub type Inen8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEN9` reader - INEN9"]
pub type Inen9R = crate::BitReader;
#[doc = "Field `INEN9` writer - INEN9"]
pub type Inen9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEN10` reader - INEN10"]
pub type Inen10R = crate::BitReader;
#[doc = "Field `INEN10` writer - INEN10"]
pub type Inen10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEN11` reader - INEN11"]
pub type Inen11R = crate::BitReader;
#[doc = "Field `INEN11` writer - INEN11"]
pub type Inen11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEN12` reader - INEN12"]
pub type Inen12R = crate::BitReader;
#[doc = "Field `INEN12` writer - INEN12"]
pub type Inen12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEN13` reader - INEN13"]
pub type Inen13R = crate::BitReader;
#[doc = "Field `INEN13` writer - INEN13"]
pub type Inen13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEN14` reader - INEN14"]
pub type Inen14R = crate::BitReader;
#[doc = "Field `INEN14` writer - INEN14"]
pub type Inen14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEN15` reader - INEN15"]
pub type Inen15R = crate::BitReader;
#[doc = "Field `INEN15` writer - INEN15"]
pub type Inen15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - INEN0"]
    #[inline(always)]
    pub fn inen0(&self) -> Inen0R {
        Inen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - INEN1"]
    #[inline(always)]
    pub fn inen1(&self) -> Inen1R {
        Inen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - INEN2"]
    #[inline(always)]
    pub fn inen2(&self) -> Inen2R {
        Inen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - INEN3"]
    #[inline(always)]
    pub fn inen3(&self) -> Inen3R {
        Inen3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - INEN4"]
    #[inline(always)]
    pub fn inen4(&self) -> Inen4R {
        Inen4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - INEN5"]
    #[inline(always)]
    pub fn inen5(&self) -> Inen5R {
        Inen5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - INEN6"]
    #[inline(always)]
    pub fn inen6(&self) -> Inen6R {
        Inen6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - INEN7"]
    #[inline(always)]
    pub fn inen7(&self) -> Inen7R {
        Inen7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - INEN8"]
    #[inline(always)]
    pub fn inen8(&self) -> Inen8R {
        Inen8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - INEN9"]
    #[inline(always)]
    pub fn inen9(&self) -> Inen9R {
        Inen9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - INEN10"]
    #[inline(always)]
    pub fn inen10(&self) -> Inen10R {
        Inen10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - INEN11"]
    #[inline(always)]
    pub fn inen11(&self) -> Inen11R {
        Inen11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - INEN12"]
    #[inline(always)]
    pub fn inen12(&self) -> Inen12R {
        Inen12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - INEN13"]
    #[inline(always)]
    pub fn inen13(&self) -> Inen13R {
        Inen13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - INEN14"]
    #[inline(always)]
    pub fn inen14(&self) -> Inen14R {
        Inen14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - INEN15"]
    #[inline(always)]
    pub fn inen15(&self) -> Inen15R {
        Inen15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INEN0"]
    #[inline(always)]
    #[must_use]
    pub fn inen0(&mut self) -> Inen0W<InerSpec> {
        Inen0W::new(self, 0)
    }
    #[doc = "Bit 1 - INEN1"]
    #[inline(always)]
    #[must_use]
    pub fn inen1(&mut self) -> Inen1W<InerSpec> {
        Inen1W::new(self, 1)
    }
    #[doc = "Bit 2 - INEN2"]
    #[inline(always)]
    #[must_use]
    pub fn inen2(&mut self) -> Inen2W<InerSpec> {
        Inen2W::new(self, 2)
    }
    #[doc = "Bit 3 - INEN3"]
    #[inline(always)]
    #[must_use]
    pub fn inen3(&mut self) -> Inen3W<InerSpec> {
        Inen3W::new(self, 3)
    }
    #[doc = "Bit 4 - INEN4"]
    #[inline(always)]
    #[must_use]
    pub fn inen4(&mut self) -> Inen4W<InerSpec> {
        Inen4W::new(self, 4)
    }
    #[doc = "Bit 5 - INEN5"]
    #[inline(always)]
    #[must_use]
    pub fn inen5(&mut self) -> Inen5W<InerSpec> {
        Inen5W::new(self, 5)
    }
    #[doc = "Bit 6 - INEN6"]
    #[inline(always)]
    #[must_use]
    pub fn inen6(&mut self) -> Inen6W<InerSpec> {
        Inen6W::new(self, 6)
    }
    #[doc = "Bit 7 - INEN7"]
    #[inline(always)]
    #[must_use]
    pub fn inen7(&mut self) -> Inen7W<InerSpec> {
        Inen7W::new(self, 7)
    }
    #[doc = "Bit 8 - INEN8"]
    #[inline(always)]
    #[must_use]
    pub fn inen8(&mut self) -> Inen8W<InerSpec> {
        Inen8W::new(self, 8)
    }
    #[doc = "Bit 9 - INEN9"]
    #[inline(always)]
    #[must_use]
    pub fn inen9(&mut self) -> Inen9W<InerSpec> {
        Inen9W::new(self, 9)
    }
    #[doc = "Bit 10 - INEN10"]
    #[inline(always)]
    #[must_use]
    pub fn inen10(&mut self) -> Inen10W<InerSpec> {
        Inen10W::new(self, 10)
    }
    #[doc = "Bit 11 - INEN11"]
    #[inline(always)]
    #[must_use]
    pub fn inen11(&mut self) -> Inen11W<InerSpec> {
        Inen11W::new(self, 11)
    }
    #[doc = "Bit 12 - INEN12"]
    #[inline(always)]
    #[must_use]
    pub fn inen12(&mut self) -> Inen12W<InerSpec> {
        Inen12W::new(self, 12)
    }
    #[doc = "Bit 13 - INEN13"]
    #[inline(always)]
    #[must_use]
    pub fn inen13(&mut self) -> Inen13W<InerSpec> {
        Inen13W::new(self, 13)
    }
    #[doc = "Bit 14 - INEN14"]
    #[inline(always)]
    #[must_use]
    pub fn inen14(&mut self) -> Inen14W<InerSpec> {
        Inen14W::new(self, 14)
    }
    #[doc = "Bit 15 - INEN15"]
    #[inline(always)]
    #[must_use]
    pub fn inen15(&mut self) -> Inen15W<InerSpec> {
        Inen15W::new(self, 15)
    }
}
#[doc = "INER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iner::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iner::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InerSpec;
impl crate::RegisterSpec for InerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iner::R`](R) reader structure"]
impl crate::Readable for InerSpec {}
#[doc = "`write(|w| ..)` method takes [`iner::W`](W) writer structure"]
impl crate::Writable for InerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INER to value 0"]
impl crate::Resettable for InerSpec {
    const RESET_VALUE: u32 = 0;
}
