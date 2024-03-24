#[doc = "Register `DOUTR` reader"]
pub type R = crate::R<DoutrSpec>;
#[doc = "Register `DOUTR` writer"]
pub type W = crate::W<DoutrSpec>;
#[doc = "Field `DOUT0` reader - DOUT0"]
pub type Dout0R = crate::BitReader;
#[doc = "Field `DOUT0` writer - DOUT0"]
pub type Dout0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT1` reader - DOUT1"]
pub type Dout1R = crate::BitReader;
#[doc = "Field `DOUT1` writer - DOUT1"]
pub type Dout1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT2` reader - DOUT2"]
pub type Dout2R = crate::BitReader;
#[doc = "Field `DOUT2` writer - DOUT2"]
pub type Dout2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT3` reader - DOUT3"]
pub type Dout3R = crate::BitReader;
#[doc = "Field `DOUT3` writer - DOUT3"]
pub type Dout3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT4` reader - DOUT4"]
pub type Dout4R = crate::BitReader;
#[doc = "Field `DOUT4` writer - DOUT4"]
pub type Dout4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT5` reader - DOUT5"]
pub type Dout5R = crate::BitReader;
#[doc = "Field `DOUT5` writer - DOUT5"]
pub type Dout5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT6` reader - DOUT6"]
pub type Dout6R = crate::BitReader;
#[doc = "Field `DOUT6` writer - DOUT6"]
pub type Dout6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT7` reader - DOUT7"]
pub type Dout7R = crate::BitReader;
#[doc = "Field `DOUT7` writer - DOUT7"]
pub type Dout7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT8` reader - DOUT8"]
pub type Dout8R = crate::BitReader;
#[doc = "Field `DOUT8` writer - DOUT8"]
pub type Dout8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT9` reader - DOUT9"]
pub type Dout9R = crate::BitReader;
#[doc = "Field `DOUT9` writer - DOUT9"]
pub type Dout9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT10` reader - DOUT10"]
pub type Dout10R = crate::BitReader;
#[doc = "Field `DOUT10` writer - DOUT10"]
pub type Dout10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT11` reader - DOUT11"]
pub type Dout11R = crate::BitReader;
#[doc = "Field `DOUT11` writer - DOUT11"]
pub type Dout11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT12` reader - DOUT12"]
pub type Dout12R = crate::BitReader;
#[doc = "Field `DOUT12` writer - DOUT12"]
pub type Dout12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT13` reader - DOUT13"]
pub type Dout13R = crate::BitReader;
#[doc = "Field `DOUT13` writer - DOUT13"]
pub type Dout13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT14` reader - DOUT14"]
pub type Dout14R = crate::BitReader;
#[doc = "Field `DOUT14` writer - DOUT14"]
pub type Dout14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT15` reader - DOUT15"]
pub type Dout15R = crate::BitReader;
#[doc = "Field `DOUT15` writer - DOUT15"]
pub type Dout15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DOUT0"]
    #[inline(always)]
    pub fn dout0(&self) -> Dout0R {
        Dout0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DOUT1"]
    #[inline(always)]
    pub fn dout1(&self) -> Dout1R {
        Dout1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DOUT2"]
    #[inline(always)]
    pub fn dout2(&self) -> Dout2R {
        Dout2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DOUT3"]
    #[inline(always)]
    pub fn dout3(&self) -> Dout3R {
        Dout3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DOUT4"]
    #[inline(always)]
    pub fn dout4(&self) -> Dout4R {
        Dout4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DOUT5"]
    #[inline(always)]
    pub fn dout5(&self) -> Dout5R {
        Dout5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DOUT6"]
    #[inline(always)]
    pub fn dout6(&self) -> Dout6R {
        Dout6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DOUT7"]
    #[inline(always)]
    pub fn dout7(&self) -> Dout7R {
        Dout7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DOUT8"]
    #[inline(always)]
    pub fn dout8(&self) -> Dout8R {
        Dout8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DOUT9"]
    #[inline(always)]
    pub fn dout9(&self) -> Dout9R {
        Dout9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DOUT10"]
    #[inline(always)]
    pub fn dout10(&self) -> Dout10R {
        Dout10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DOUT11"]
    #[inline(always)]
    pub fn dout11(&self) -> Dout11R {
        Dout11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DOUT12"]
    #[inline(always)]
    pub fn dout12(&self) -> Dout12R {
        Dout12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DOUT13"]
    #[inline(always)]
    pub fn dout13(&self) -> Dout13R {
        Dout13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DOUT14"]
    #[inline(always)]
    pub fn dout14(&self) -> Dout14R {
        Dout14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DOUT15"]
    #[inline(always)]
    pub fn dout15(&self) -> Dout15R {
        Dout15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DOUT0"]
    #[inline(always)]
    #[must_use]
    pub fn dout0(&mut self) -> Dout0W<DoutrSpec> {
        Dout0W::new(self, 0)
    }
    #[doc = "Bit 1 - DOUT1"]
    #[inline(always)]
    #[must_use]
    pub fn dout1(&mut self) -> Dout1W<DoutrSpec> {
        Dout1W::new(self, 1)
    }
    #[doc = "Bit 2 - DOUT2"]
    #[inline(always)]
    #[must_use]
    pub fn dout2(&mut self) -> Dout2W<DoutrSpec> {
        Dout2W::new(self, 2)
    }
    #[doc = "Bit 3 - DOUT3"]
    #[inline(always)]
    #[must_use]
    pub fn dout3(&mut self) -> Dout3W<DoutrSpec> {
        Dout3W::new(self, 3)
    }
    #[doc = "Bit 4 - DOUT4"]
    #[inline(always)]
    #[must_use]
    pub fn dout4(&mut self) -> Dout4W<DoutrSpec> {
        Dout4W::new(self, 4)
    }
    #[doc = "Bit 5 - DOUT5"]
    #[inline(always)]
    #[must_use]
    pub fn dout5(&mut self) -> Dout5W<DoutrSpec> {
        Dout5W::new(self, 5)
    }
    #[doc = "Bit 6 - DOUT6"]
    #[inline(always)]
    #[must_use]
    pub fn dout6(&mut self) -> Dout6W<DoutrSpec> {
        Dout6W::new(self, 6)
    }
    #[doc = "Bit 7 - DOUT7"]
    #[inline(always)]
    #[must_use]
    pub fn dout7(&mut self) -> Dout7W<DoutrSpec> {
        Dout7W::new(self, 7)
    }
    #[doc = "Bit 8 - DOUT8"]
    #[inline(always)]
    #[must_use]
    pub fn dout8(&mut self) -> Dout8W<DoutrSpec> {
        Dout8W::new(self, 8)
    }
    #[doc = "Bit 9 - DOUT9"]
    #[inline(always)]
    #[must_use]
    pub fn dout9(&mut self) -> Dout9W<DoutrSpec> {
        Dout9W::new(self, 9)
    }
    #[doc = "Bit 10 - DOUT10"]
    #[inline(always)]
    #[must_use]
    pub fn dout10(&mut self) -> Dout10W<DoutrSpec> {
        Dout10W::new(self, 10)
    }
    #[doc = "Bit 11 - DOUT11"]
    #[inline(always)]
    #[must_use]
    pub fn dout11(&mut self) -> Dout11W<DoutrSpec> {
        Dout11W::new(self, 11)
    }
    #[doc = "Bit 12 - DOUT12"]
    #[inline(always)]
    #[must_use]
    pub fn dout12(&mut self) -> Dout12W<DoutrSpec> {
        Dout12W::new(self, 12)
    }
    #[doc = "Bit 13 - DOUT13"]
    #[inline(always)]
    #[must_use]
    pub fn dout13(&mut self) -> Dout13W<DoutrSpec> {
        Dout13W::new(self, 13)
    }
    #[doc = "Bit 14 - DOUT14"]
    #[inline(always)]
    #[must_use]
    pub fn dout14(&mut self) -> Dout14W<DoutrSpec> {
        Dout14W::new(self, 14)
    }
    #[doc = "Bit 15 - DOUT15"]
    #[inline(always)]
    #[must_use]
    pub fn dout15(&mut self) -> Dout15W<DoutrSpec> {
        Dout15W::new(self, 15)
    }
}
#[doc = "DOUTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoutrSpec;
impl crate::RegisterSpec for DoutrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutr::R`](R) reader structure"]
impl crate::Readable for DoutrSpec {}
#[doc = "`write(|w| ..)` method takes [`doutr::W`](W) writer structure"]
impl crate::Writable for DoutrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTR to value 0"]
impl crate::Resettable for DoutrSpec {
    const RESET_VALUE: u32 = 0;
}
