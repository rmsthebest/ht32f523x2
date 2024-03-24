#[doc = "Register `DRVR` reader"]
pub type R = crate::R<DrvrSpec>;
#[doc = "Register `DRVR` writer"]
pub type W = crate::W<DrvrSpec>;
#[doc = "Field `DV0` reader - DV0"]
pub type Dv0R = crate::BitReader;
#[doc = "Field `DV0` writer - DV0"]
pub type Dv0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DV1` reader - DV1"]
pub type Dv1R = crate::BitReader;
#[doc = "Field `DV1` writer - DV1"]
pub type Dv1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DV2` reader - DV2"]
pub type Dv2R = crate::BitReader;
#[doc = "Field `DV2` writer - DV2"]
pub type Dv2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DV3` reader - DV3"]
pub type Dv3R = crate::BitReader;
#[doc = "Field `DV3` writer - DV3"]
pub type Dv3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DV4` reader - DV4"]
pub type Dv4R = crate::BitReader;
#[doc = "Field `DV4` writer - DV4"]
pub type Dv4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DV5` reader - DV5"]
pub type Dv5R = crate::BitReader;
#[doc = "Field `DV5` writer - DV5"]
pub type Dv5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DV6` reader - DV6"]
pub type Dv6R = crate::BitReader;
#[doc = "Field `DV6` writer - DV6"]
pub type Dv6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DV7` reader - DV7"]
pub type Dv7R = crate::BitReader;
#[doc = "Field `DV7` writer - DV7"]
pub type Dv7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DV8` reader - DV8"]
pub type Dv8R = crate::BitReader;
#[doc = "Field `DV8` writer - DV8"]
pub type Dv8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DV9` reader - DV9"]
pub type Dv9R = crate::BitReader;
#[doc = "Field `DV9` writer - DV9"]
pub type Dv9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DV10` reader - DV10"]
pub type Dv10R = crate::BitReader;
#[doc = "Field `DV10` writer - DV10"]
pub type Dv10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DV11` reader - DV11"]
pub type Dv11R = crate::BitReader;
#[doc = "Field `DV11` writer - DV11"]
pub type Dv11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DV12` reader - DV12"]
pub type Dv12R = crate::BitReader;
#[doc = "Field `DV12` writer - DV12"]
pub type Dv12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DV13` reader - DV13"]
pub type Dv13R = crate::BitReader;
#[doc = "Field `DV13` writer - DV13"]
pub type Dv13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DV14` reader - DV14"]
pub type Dv14R = crate::BitReader;
#[doc = "Field `DV14` writer - DV14"]
pub type Dv14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DV15` reader - DV15"]
pub type Dv15R = crate::BitReader;
#[doc = "Field `DV15` writer - DV15"]
pub type Dv15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DV0"]
    #[inline(always)]
    pub fn dv0(&self) -> Dv0R {
        Dv0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DV1"]
    #[inline(always)]
    pub fn dv1(&self) -> Dv1R {
        Dv1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DV2"]
    #[inline(always)]
    pub fn dv2(&self) -> Dv2R {
        Dv2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DV3"]
    #[inline(always)]
    pub fn dv3(&self) -> Dv3R {
        Dv3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DV4"]
    #[inline(always)]
    pub fn dv4(&self) -> Dv4R {
        Dv4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DV5"]
    #[inline(always)]
    pub fn dv5(&self) -> Dv5R {
        Dv5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DV6"]
    #[inline(always)]
    pub fn dv6(&self) -> Dv6R {
        Dv6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DV7"]
    #[inline(always)]
    pub fn dv7(&self) -> Dv7R {
        Dv7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DV8"]
    #[inline(always)]
    pub fn dv8(&self) -> Dv8R {
        Dv8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DV9"]
    #[inline(always)]
    pub fn dv9(&self) -> Dv9R {
        Dv9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DV10"]
    #[inline(always)]
    pub fn dv10(&self) -> Dv10R {
        Dv10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DV11"]
    #[inline(always)]
    pub fn dv11(&self) -> Dv11R {
        Dv11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DV12"]
    #[inline(always)]
    pub fn dv12(&self) -> Dv12R {
        Dv12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DV13"]
    #[inline(always)]
    pub fn dv13(&self) -> Dv13R {
        Dv13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DV14"]
    #[inline(always)]
    pub fn dv14(&self) -> Dv14R {
        Dv14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DV15"]
    #[inline(always)]
    pub fn dv15(&self) -> Dv15R {
        Dv15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DV0"]
    #[inline(always)]
    #[must_use]
    pub fn dv0(&mut self) -> Dv0W<DrvrSpec> {
        Dv0W::new(self, 0)
    }
    #[doc = "Bit 1 - DV1"]
    #[inline(always)]
    #[must_use]
    pub fn dv1(&mut self) -> Dv1W<DrvrSpec> {
        Dv1W::new(self, 1)
    }
    #[doc = "Bit 2 - DV2"]
    #[inline(always)]
    #[must_use]
    pub fn dv2(&mut self) -> Dv2W<DrvrSpec> {
        Dv2W::new(self, 2)
    }
    #[doc = "Bit 3 - DV3"]
    #[inline(always)]
    #[must_use]
    pub fn dv3(&mut self) -> Dv3W<DrvrSpec> {
        Dv3W::new(self, 3)
    }
    #[doc = "Bit 4 - DV4"]
    #[inline(always)]
    #[must_use]
    pub fn dv4(&mut self) -> Dv4W<DrvrSpec> {
        Dv4W::new(self, 4)
    }
    #[doc = "Bit 5 - DV5"]
    #[inline(always)]
    #[must_use]
    pub fn dv5(&mut self) -> Dv5W<DrvrSpec> {
        Dv5W::new(self, 5)
    }
    #[doc = "Bit 6 - DV6"]
    #[inline(always)]
    #[must_use]
    pub fn dv6(&mut self) -> Dv6W<DrvrSpec> {
        Dv6W::new(self, 6)
    }
    #[doc = "Bit 7 - DV7"]
    #[inline(always)]
    #[must_use]
    pub fn dv7(&mut self) -> Dv7W<DrvrSpec> {
        Dv7W::new(self, 7)
    }
    #[doc = "Bit 8 - DV8"]
    #[inline(always)]
    #[must_use]
    pub fn dv8(&mut self) -> Dv8W<DrvrSpec> {
        Dv8W::new(self, 8)
    }
    #[doc = "Bit 9 - DV9"]
    #[inline(always)]
    #[must_use]
    pub fn dv9(&mut self) -> Dv9W<DrvrSpec> {
        Dv9W::new(self, 9)
    }
    #[doc = "Bit 10 - DV10"]
    #[inline(always)]
    #[must_use]
    pub fn dv10(&mut self) -> Dv10W<DrvrSpec> {
        Dv10W::new(self, 10)
    }
    #[doc = "Bit 11 - DV11"]
    #[inline(always)]
    #[must_use]
    pub fn dv11(&mut self) -> Dv11W<DrvrSpec> {
        Dv11W::new(self, 11)
    }
    #[doc = "Bit 12 - DV12"]
    #[inline(always)]
    #[must_use]
    pub fn dv12(&mut self) -> Dv12W<DrvrSpec> {
        Dv12W::new(self, 12)
    }
    #[doc = "Bit 13 - DV13"]
    #[inline(always)]
    #[must_use]
    pub fn dv13(&mut self) -> Dv13W<DrvrSpec> {
        Dv13W::new(self, 13)
    }
    #[doc = "Bit 14 - DV14"]
    #[inline(always)]
    #[must_use]
    pub fn dv14(&mut self) -> Dv14W<DrvrSpec> {
        Dv14W::new(self, 14)
    }
    #[doc = "Bit 15 - DV15"]
    #[inline(always)]
    #[must_use]
    pub fn dv15(&mut self) -> Dv15W<DrvrSpec> {
        Dv15W::new(self, 15)
    }
}
#[doc = "DRVR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`drvr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drvr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrvrSpec;
impl crate::RegisterSpec for DrvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`drvr::R`](R) reader structure"]
impl crate::Readable for DrvrSpec {}
#[doc = "`write(|w| ..)` method takes [`drvr::W`](W) writer structure"]
impl crate::Writable for DrvrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DRVR to value 0"]
impl crate::Resettable for DrvrSpec {
    const RESET_VALUE: u32 = 0;
}
