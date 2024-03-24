#[doc = "Register `DIRCR` reader"]
pub type R = crate::R<DircrSpec>;
#[doc = "Register `DIRCR` writer"]
pub type W = crate::W<DircrSpec>;
#[doc = "Field `DIR0` reader - DIR0"]
pub type Dir0R = crate::BitReader;
#[doc = "Field `DIR0` writer - DIR0"]
pub type Dir0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR1` reader - DIR1"]
pub type Dir1R = crate::BitReader;
#[doc = "Field `DIR1` writer - DIR1"]
pub type Dir1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR2` reader - DIR2"]
pub type Dir2R = crate::BitReader;
#[doc = "Field `DIR2` writer - DIR2"]
pub type Dir2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR3` reader - DIR3"]
pub type Dir3R = crate::BitReader;
#[doc = "Field `DIR3` writer - DIR3"]
pub type Dir3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR4` reader - DIR4"]
pub type Dir4R = crate::BitReader;
#[doc = "Field `DIR4` writer - DIR4"]
pub type Dir4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR5` reader - DIR5"]
pub type Dir5R = crate::BitReader;
#[doc = "Field `DIR5` writer - DIR5"]
pub type Dir5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR6` reader - DIR6"]
pub type Dir6R = crate::BitReader;
#[doc = "Field `DIR6` writer - DIR6"]
pub type Dir6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR7` reader - DIR7"]
pub type Dir7R = crate::BitReader;
#[doc = "Field `DIR7` writer - DIR7"]
pub type Dir7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR8` reader - DIR8"]
pub type Dir8R = crate::BitReader;
#[doc = "Field `DIR8` writer - DIR8"]
pub type Dir8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR9` reader - DIR9"]
pub type Dir9R = crate::BitReader;
#[doc = "Field `DIR9` writer - DIR9"]
pub type Dir9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR10` reader - DIR10"]
pub type Dir10R = crate::BitReader;
#[doc = "Field `DIR10` writer - DIR10"]
pub type Dir10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR11` reader - DIR11"]
pub type Dir11R = crate::BitReader;
#[doc = "Field `DIR11` writer - DIR11"]
pub type Dir11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR12` reader - DIR12"]
pub type Dir12R = crate::BitReader;
#[doc = "Field `DIR12` writer - DIR12"]
pub type Dir12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR13` reader - DIR13"]
pub type Dir13R = crate::BitReader;
#[doc = "Field `DIR13` writer - DIR13"]
pub type Dir13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR14` reader - DIR14"]
pub type Dir14R = crate::BitReader;
#[doc = "Field `DIR14` writer - DIR14"]
pub type Dir14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR15` reader - DIR15"]
pub type Dir15R = crate::BitReader;
#[doc = "Field `DIR15` writer - DIR15"]
pub type Dir15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DIR0"]
    #[inline(always)]
    pub fn dir0(&self) -> Dir0R {
        Dir0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DIR1"]
    #[inline(always)]
    pub fn dir1(&self) -> Dir1R {
        Dir1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DIR2"]
    #[inline(always)]
    pub fn dir2(&self) -> Dir2R {
        Dir2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DIR3"]
    #[inline(always)]
    pub fn dir3(&self) -> Dir3R {
        Dir3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DIR4"]
    #[inline(always)]
    pub fn dir4(&self) -> Dir4R {
        Dir4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DIR5"]
    #[inline(always)]
    pub fn dir5(&self) -> Dir5R {
        Dir5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DIR6"]
    #[inline(always)]
    pub fn dir6(&self) -> Dir6R {
        Dir6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DIR7"]
    #[inline(always)]
    pub fn dir7(&self) -> Dir7R {
        Dir7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DIR8"]
    #[inline(always)]
    pub fn dir8(&self) -> Dir8R {
        Dir8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DIR9"]
    #[inline(always)]
    pub fn dir9(&self) -> Dir9R {
        Dir9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DIR10"]
    #[inline(always)]
    pub fn dir10(&self) -> Dir10R {
        Dir10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DIR11"]
    #[inline(always)]
    pub fn dir11(&self) -> Dir11R {
        Dir11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DIR12"]
    #[inline(always)]
    pub fn dir12(&self) -> Dir12R {
        Dir12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DIR13"]
    #[inline(always)]
    pub fn dir13(&self) -> Dir13R {
        Dir13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DIR14"]
    #[inline(always)]
    pub fn dir14(&self) -> Dir14R {
        Dir14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DIR15"]
    #[inline(always)]
    pub fn dir15(&self) -> Dir15R {
        Dir15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIR0"]
    #[inline(always)]
    #[must_use]
    pub fn dir0(&mut self) -> Dir0W<DircrSpec> {
        Dir0W::new(self, 0)
    }
    #[doc = "Bit 1 - DIR1"]
    #[inline(always)]
    #[must_use]
    pub fn dir1(&mut self) -> Dir1W<DircrSpec> {
        Dir1W::new(self, 1)
    }
    #[doc = "Bit 2 - DIR2"]
    #[inline(always)]
    #[must_use]
    pub fn dir2(&mut self) -> Dir2W<DircrSpec> {
        Dir2W::new(self, 2)
    }
    #[doc = "Bit 3 - DIR3"]
    #[inline(always)]
    #[must_use]
    pub fn dir3(&mut self) -> Dir3W<DircrSpec> {
        Dir3W::new(self, 3)
    }
    #[doc = "Bit 4 - DIR4"]
    #[inline(always)]
    #[must_use]
    pub fn dir4(&mut self) -> Dir4W<DircrSpec> {
        Dir4W::new(self, 4)
    }
    #[doc = "Bit 5 - DIR5"]
    #[inline(always)]
    #[must_use]
    pub fn dir5(&mut self) -> Dir5W<DircrSpec> {
        Dir5W::new(self, 5)
    }
    #[doc = "Bit 6 - DIR6"]
    #[inline(always)]
    #[must_use]
    pub fn dir6(&mut self) -> Dir6W<DircrSpec> {
        Dir6W::new(self, 6)
    }
    #[doc = "Bit 7 - DIR7"]
    #[inline(always)]
    #[must_use]
    pub fn dir7(&mut self) -> Dir7W<DircrSpec> {
        Dir7W::new(self, 7)
    }
    #[doc = "Bit 8 - DIR8"]
    #[inline(always)]
    #[must_use]
    pub fn dir8(&mut self) -> Dir8W<DircrSpec> {
        Dir8W::new(self, 8)
    }
    #[doc = "Bit 9 - DIR9"]
    #[inline(always)]
    #[must_use]
    pub fn dir9(&mut self) -> Dir9W<DircrSpec> {
        Dir9W::new(self, 9)
    }
    #[doc = "Bit 10 - DIR10"]
    #[inline(always)]
    #[must_use]
    pub fn dir10(&mut self) -> Dir10W<DircrSpec> {
        Dir10W::new(self, 10)
    }
    #[doc = "Bit 11 - DIR11"]
    #[inline(always)]
    #[must_use]
    pub fn dir11(&mut self) -> Dir11W<DircrSpec> {
        Dir11W::new(self, 11)
    }
    #[doc = "Bit 12 - DIR12"]
    #[inline(always)]
    #[must_use]
    pub fn dir12(&mut self) -> Dir12W<DircrSpec> {
        Dir12W::new(self, 12)
    }
    #[doc = "Bit 13 - DIR13"]
    #[inline(always)]
    #[must_use]
    pub fn dir13(&mut self) -> Dir13W<DircrSpec> {
        Dir13W::new(self, 13)
    }
    #[doc = "Bit 14 - DIR14"]
    #[inline(always)]
    #[must_use]
    pub fn dir14(&mut self) -> Dir14W<DircrSpec> {
        Dir14W::new(self, 14)
    }
    #[doc = "Bit 15 - DIR15"]
    #[inline(always)]
    #[must_use]
    pub fn dir15(&mut self) -> Dir15W<DircrSpec> {
        Dir15W::new(self, 15)
    }
}
#[doc = "DIRCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dircr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dircr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DircrSpec;
impl crate::RegisterSpec for DircrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dircr::R`](R) reader structure"]
impl crate::Readable for DircrSpec {}
#[doc = "`write(|w| ..)` method takes [`dircr::W`](W) writer structure"]
impl crate::Writable for DircrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIRCR to value 0"]
impl crate::Resettable for DircrSpec {
    const RESET_VALUE: u32 = 0;
}
