#[doc = "Register `DINR` reader"]
pub type R = crate::R<DinrSpec>;
#[doc = "Register `DINR` writer"]
pub type W = crate::W<DinrSpec>;
#[doc = "Field `DIN0` reader - DIN0"]
pub type Din0R = crate::BitReader;
#[doc = "Field `DIN0` writer - DIN0"]
pub type Din0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIN1` reader - DIN1"]
pub type Din1R = crate::BitReader;
#[doc = "Field `DIN1` writer - DIN1"]
pub type Din1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIN2` reader - DIN2"]
pub type Din2R = crate::BitReader;
#[doc = "Field `DIN2` writer - DIN2"]
pub type Din2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIN3` reader - DIN3"]
pub type Din3R = crate::BitReader;
#[doc = "Field `DIN3` writer - DIN3"]
pub type Din3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIN4` reader - DIN4"]
pub type Din4R = crate::BitReader;
#[doc = "Field `DIN4` writer - DIN4"]
pub type Din4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIN5` reader - DIN5"]
pub type Din5R = crate::BitReader;
#[doc = "Field `DIN5` writer - DIN5"]
pub type Din5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIN6` reader - DIN6"]
pub type Din6R = crate::BitReader;
#[doc = "Field `DIN6` writer - DIN6"]
pub type Din6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIN7` reader - DIN7"]
pub type Din7R = crate::BitReader;
#[doc = "Field `DIN7` writer - DIN7"]
pub type Din7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIN8` reader - DIN8"]
pub type Din8R = crate::BitReader;
#[doc = "Field `DIN8` writer - DIN8"]
pub type Din8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIN9` reader - DIN9"]
pub type Din9R = crate::BitReader;
#[doc = "Field `DIN9` writer - DIN9"]
pub type Din9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIN10` reader - DIN10"]
pub type Din10R = crate::BitReader;
#[doc = "Field `DIN10` writer - DIN10"]
pub type Din10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIN11` reader - DIN11"]
pub type Din11R = crate::BitReader;
#[doc = "Field `DIN11` writer - DIN11"]
pub type Din11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIN12` reader - DIN12"]
pub type Din12R = crate::BitReader;
#[doc = "Field `DIN12` writer - DIN12"]
pub type Din12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIN13` reader - DIN13"]
pub type Din13R = crate::BitReader;
#[doc = "Field `DIN13` writer - DIN13"]
pub type Din13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIN14` reader - DIN14"]
pub type Din14R = crate::BitReader;
#[doc = "Field `DIN14` writer - DIN14"]
pub type Din14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIN15` reader - DIN15"]
pub type Din15R = crate::BitReader;
#[doc = "Field `DIN15` writer - DIN15"]
pub type Din15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DIN0"]
    #[inline(always)]
    pub fn din0(&self) -> Din0R {
        Din0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DIN1"]
    #[inline(always)]
    pub fn din1(&self) -> Din1R {
        Din1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DIN2"]
    #[inline(always)]
    pub fn din2(&self) -> Din2R {
        Din2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DIN3"]
    #[inline(always)]
    pub fn din3(&self) -> Din3R {
        Din3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DIN4"]
    #[inline(always)]
    pub fn din4(&self) -> Din4R {
        Din4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DIN5"]
    #[inline(always)]
    pub fn din5(&self) -> Din5R {
        Din5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DIN6"]
    #[inline(always)]
    pub fn din6(&self) -> Din6R {
        Din6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DIN7"]
    #[inline(always)]
    pub fn din7(&self) -> Din7R {
        Din7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DIN8"]
    #[inline(always)]
    pub fn din8(&self) -> Din8R {
        Din8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DIN9"]
    #[inline(always)]
    pub fn din9(&self) -> Din9R {
        Din9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DIN10"]
    #[inline(always)]
    pub fn din10(&self) -> Din10R {
        Din10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DIN11"]
    #[inline(always)]
    pub fn din11(&self) -> Din11R {
        Din11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DIN12"]
    #[inline(always)]
    pub fn din12(&self) -> Din12R {
        Din12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DIN13"]
    #[inline(always)]
    pub fn din13(&self) -> Din13R {
        Din13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DIN14"]
    #[inline(always)]
    pub fn din14(&self) -> Din14R {
        Din14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DIN15"]
    #[inline(always)]
    pub fn din15(&self) -> Din15R {
        Din15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIN0"]
    #[inline(always)]
    #[must_use]
    pub fn din0(&mut self) -> Din0W<DinrSpec> {
        Din0W::new(self, 0)
    }
    #[doc = "Bit 1 - DIN1"]
    #[inline(always)]
    #[must_use]
    pub fn din1(&mut self) -> Din1W<DinrSpec> {
        Din1W::new(self, 1)
    }
    #[doc = "Bit 2 - DIN2"]
    #[inline(always)]
    #[must_use]
    pub fn din2(&mut self) -> Din2W<DinrSpec> {
        Din2W::new(self, 2)
    }
    #[doc = "Bit 3 - DIN3"]
    #[inline(always)]
    #[must_use]
    pub fn din3(&mut self) -> Din3W<DinrSpec> {
        Din3W::new(self, 3)
    }
    #[doc = "Bit 4 - DIN4"]
    #[inline(always)]
    #[must_use]
    pub fn din4(&mut self) -> Din4W<DinrSpec> {
        Din4W::new(self, 4)
    }
    #[doc = "Bit 5 - DIN5"]
    #[inline(always)]
    #[must_use]
    pub fn din5(&mut self) -> Din5W<DinrSpec> {
        Din5W::new(self, 5)
    }
    #[doc = "Bit 6 - DIN6"]
    #[inline(always)]
    #[must_use]
    pub fn din6(&mut self) -> Din6W<DinrSpec> {
        Din6W::new(self, 6)
    }
    #[doc = "Bit 7 - DIN7"]
    #[inline(always)]
    #[must_use]
    pub fn din7(&mut self) -> Din7W<DinrSpec> {
        Din7W::new(self, 7)
    }
    #[doc = "Bit 8 - DIN8"]
    #[inline(always)]
    #[must_use]
    pub fn din8(&mut self) -> Din8W<DinrSpec> {
        Din8W::new(self, 8)
    }
    #[doc = "Bit 9 - DIN9"]
    #[inline(always)]
    #[must_use]
    pub fn din9(&mut self) -> Din9W<DinrSpec> {
        Din9W::new(self, 9)
    }
    #[doc = "Bit 10 - DIN10"]
    #[inline(always)]
    #[must_use]
    pub fn din10(&mut self) -> Din10W<DinrSpec> {
        Din10W::new(self, 10)
    }
    #[doc = "Bit 11 - DIN11"]
    #[inline(always)]
    #[must_use]
    pub fn din11(&mut self) -> Din11W<DinrSpec> {
        Din11W::new(self, 11)
    }
    #[doc = "Bit 12 - DIN12"]
    #[inline(always)]
    #[must_use]
    pub fn din12(&mut self) -> Din12W<DinrSpec> {
        Din12W::new(self, 12)
    }
    #[doc = "Bit 13 - DIN13"]
    #[inline(always)]
    #[must_use]
    pub fn din13(&mut self) -> Din13W<DinrSpec> {
        Din13W::new(self, 13)
    }
    #[doc = "Bit 14 - DIN14"]
    #[inline(always)]
    #[must_use]
    pub fn din14(&mut self) -> Din14W<DinrSpec> {
        Din14W::new(self, 14)
    }
    #[doc = "Bit 15 - DIN15"]
    #[inline(always)]
    #[must_use]
    pub fn din15(&mut self) -> Din15W<DinrSpec> {
        Din15W::new(self, 15)
    }
}
#[doc = "DINR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dinr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DinrSpec;
impl crate::RegisterSpec for DinrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr::R`](R) reader structure"]
impl crate::Readable for DinrSpec {}
#[doc = "`write(|w| ..)` method takes [`dinr::W`](W) writer structure"]
impl crate::Writable for DinrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DINR to value 0"]
impl crate::Resettable for DinrSpec {
    const RESET_VALUE: u32 = 0;
}
