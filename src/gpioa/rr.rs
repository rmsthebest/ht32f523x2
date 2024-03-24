#[doc = "Register `RR` reader"]
pub type R = crate::R<RrSpec>;
#[doc = "Register `RR` writer"]
pub type W = crate::W<RrSpec>;
#[doc = "Field `RST0` reader - RST0"]
pub type Rst0R = crate::BitReader;
#[doc = "Field `RST0` writer - RST0"]
pub type Rst0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST1` reader - RST1"]
pub type Rst1R = crate::BitReader;
#[doc = "Field `RST1` writer - RST1"]
pub type Rst1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST2` reader - RST2"]
pub type Rst2R = crate::BitReader;
#[doc = "Field `RST2` writer - RST2"]
pub type Rst2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST3` reader - RST3"]
pub type Rst3R = crate::BitReader;
#[doc = "Field `RST3` writer - RST3"]
pub type Rst3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST4` reader - RST4"]
pub type Rst4R = crate::BitReader;
#[doc = "Field `RST4` writer - RST4"]
pub type Rst4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST5` reader - RST5"]
pub type Rst5R = crate::BitReader;
#[doc = "Field `RST5` writer - RST5"]
pub type Rst5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST6` reader - RST6"]
pub type Rst6R = crate::BitReader;
#[doc = "Field `RST6` writer - RST6"]
pub type Rst6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST7` reader - RST7"]
pub type Rst7R = crate::BitReader;
#[doc = "Field `RST7` writer - RST7"]
pub type Rst7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST8` reader - RST8"]
pub type Rst8R = crate::BitReader;
#[doc = "Field `RST8` writer - RST8"]
pub type Rst8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST9` reader - RST9"]
pub type Rst9R = crate::BitReader;
#[doc = "Field `RST9` writer - RST9"]
pub type Rst9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST10` reader - RST10"]
pub type Rst10R = crate::BitReader;
#[doc = "Field `RST10` writer - RST10"]
pub type Rst10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST11` reader - RST11"]
pub type Rst11R = crate::BitReader;
#[doc = "Field `RST11` writer - RST11"]
pub type Rst11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST12` reader - RST12"]
pub type Rst12R = crate::BitReader;
#[doc = "Field `RST12` writer - RST12"]
pub type Rst12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST13` reader - RST13"]
pub type Rst13R = crate::BitReader;
#[doc = "Field `RST13` writer - RST13"]
pub type Rst13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST14` reader - RST14"]
pub type Rst14R = crate::BitReader;
#[doc = "Field `RST14` writer - RST14"]
pub type Rst14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST15` reader - RST15"]
pub type Rst15R = crate::BitReader;
#[doc = "Field `RST15` writer - RST15"]
pub type Rst15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RST0"]
    #[inline(always)]
    pub fn rst0(&self) -> Rst0R {
        Rst0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RST1"]
    #[inline(always)]
    pub fn rst1(&self) -> Rst1R {
        Rst1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RST2"]
    #[inline(always)]
    pub fn rst2(&self) -> Rst2R {
        Rst2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RST3"]
    #[inline(always)]
    pub fn rst3(&self) -> Rst3R {
        Rst3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RST4"]
    #[inline(always)]
    pub fn rst4(&self) -> Rst4R {
        Rst4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RST5"]
    #[inline(always)]
    pub fn rst5(&self) -> Rst5R {
        Rst5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RST6"]
    #[inline(always)]
    pub fn rst6(&self) -> Rst6R {
        Rst6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RST7"]
    #[inline(always)]
    pub fn rst7(&self) -> Rst7R {
        Rst7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RST8"]
    #[inline(always)]
    pub fn rst8(&self) -> Rst8R {
        Rst8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RST9"]
    #[inline(always)]
    pub fn rst9(&self) -> Rst9R {
        Rst9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RST10"]
    #[inline(always)]
    pub fn rst10(&self) -> Rst10R {
        Rst10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RST11"]
    #[inline(always)]
    pub fn rst11(&self) -> Rst11R {
        Rst11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RST12"]
    #[inline(always)]
    pub fn rst12(&self) -> Rst12R {
        Rst12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RST13"]
    #[inline(always)]
    pub fn rst13(&self) -> Rst13R {
        Rst13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RST14"]
    #[inline(always)]
    pub fn rst14(&self) -> Rst14R {
        Rst14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RST15"]
    #[inline(always)]
    pub fn rst15(&self) -> Rst15R {
        Rst15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RST0"]
    #[inline(always)]
    #[must_use]
    pub fn rst0(&mut self) -> Rst0W<RrSpec> {
        Rst0W::new(self, 0)
    }
    #[doc = "Bit 1 - RST1"]
    #[inline(always)]
    #[must_use]
    pub fn rst1(&mut self) -> Rst1W<RrSpec> {
        Rst1W::new(self, 1)
    }
    #[doc = "Bit 2 - RST2"]
    #[inline(always)]
    #[must_use]
    pub fn rst2(&mut self) -> Rst2W<RrSpec> {
        Rst2W::new(self, 2)
    }
    #[doc = "Bit 3 - RST3"]
    #[inline(always)]
    #[must_use]
    pub fn rst3(&mut self) -> Rst3W<RrSpec> {
        Rst3W::new(self, 3)
    }
    #[doc = "Bit 4 - RST4"]
    #[inline(always)]
    #[must_use]
    pub fn rst4(&mut self) -> Rst4W<RrSpec> {
        Rst4W::new(self, 4)
    }
    #[doc = "Bit 5 - RST5"]
    #[inline(always)]
    #[must_use]
    pub fn rst5(&mut self) -> Rst5W<RrSpec> {
        Rst5W::new(self, 5)
    }
    #[doc = "Bit 6 - RST6"]
    #[inline(always)]
    #[must_use]
    pub fn rst6(&mut self) -> Rst6W<RrSpec> {
        Rst6W::new(self, 6)
    }
    #[doc = "Bit 7 - RST7"]
    #[inline(always)]
    #[must_use]
    pub fn rst7(&mut self) -> Rst7W<RrSpec> {
        Rst7W::new(self, 7)
    }
    #[doc = "Bit 8 - RST8"]
    #[inline(always)]
    #[must_use]
    pub fn rst8(&mut self) -> Rst8W<RrSpec> {
        Rst8W::new(self, 8)
    }
    #[doc = "Bit 9 - RST9"]
    #[inline(always)]
    #[must_use]
    pub fn rst9(&mut self) -> Rst9W<RrSpec> {
        Rst9W::new(self, 9)
    }
    #[doc = "Bit 10 - RST10"]
    #[inline(always)]
    #[must_use]
    pub fn rst10(&mut self) -> Rst10W<RrSpec> {
        Rst10W::new(self, 10)
    }
    #[doc = "Bit 11 - RST11"]
    #[inline(always)]
    #[must_use]
    pub fn rst11(&mut self) -> Rst11W<RrSpec> {
        Rst11W::new(self, 11)
    }
    #[doc = "Bit 12 - RST12"]
    #[inline(always)]
    #[must_use]
    pub fn rst12(&mut self) -> Rst12W<RrSpec> {
        Rst12W::new(self, 12)
    }
    #[doc = "Bit 13 - RST13"]
    #[inline(always)]
    #[must_use]
    pub fn rst13(&mut self) -> Rst13W<RrSpec> {
        Rst13W::new(self, 13)
    }
    #[doc = "Bit 14 - RST14"]
    #[inline(always)]
    #[must_use]
    pub fn rst14(&mut self) -> Rst14W<RrSpec> {
        Rst14W::new(self, 14)
    }
    #[doc = "Bit 15 - RST15"]
    #[inline(always)]
    #[must_use]
    pub fn rst15(&mut self) -> Rst15W<RrSpec> {
        Rst15W::new(self, 15)
    }
}
#[doc = "RR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrSpec;
impl crate::RegisterSpec for RrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rr::R`](R) reader structure"]
impl crate::Readable for RrSpec {}
#[doc = "`write(|w| ..)` method takes [`rr::W`](W) writer structure"]
impl crate::Writable for RrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RR to value 0"]
impl crate::Resettable for RrSpec {
    const RESET_VALUE: u32 = 0;
}
