#[doc = "Register `LOCKR` reader"]
pub type R = crate::R<LockrSpec>;
#[doc = "Register `LOCKR` writer"]
pub type W = crate::W<LockrSpec>;
#[doc = "Field `LOCK0` reader - LOCK0"]
pub type Lock0R = crate::BitReader;
#[doc = "Field `LOCK0` writer - LOCK0"]
pub type Lock0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK1` reader - LOCK1"]
pub type Lock1R = crate::BitReader;
#[doc = "Field `LOCK1` writer - LOCK1"]
pub type Lock1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK2` reader - LOCK2"]
pub type Lock2R = crate::BitReader;
#[doc = "Field `LOCK2` writer - LOCK2"]
pub type Lock2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK3` reader - LOCK3"]
pub type Lock3R = crate::BitReader;
#[doc = "Field `LOCK3` writer - LOCK3"]
pub type Lock3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK4` reader - LOCK4"]
pub type Lock4R = crate::BitReader;
#[doc = "Field `LOCK4` writer - LOCK4"]
pub type Lock4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK5` reader - LOCK5"]
pub type Lock5R = crate::BitReader;
#[doc = "Field `LOCK5` writer - LOCK5"]
pub type Lock5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK6` reader - LOCK6"]
pub type Lock6R = crate::BitReader;
#[doc = "Field `LOCK6` writer - LOCK6"]
pub type Lock6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK7` reader - LOCK7"]
pub type Lock7R = crate::BitReader;
#[doc = "Field `LOCK7` writer - LOCK7"]
pub type Lock7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK8` reader - LOCK8"]
pub type Lock8R = crate::BitReader;
#[doc = "Field `LOCK8` writer - LOCK8"]
pub type Lock8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK9` reader - LOCK9"]
pub type Lock9R = crate::BitReader;
#[doc = "Field `LOCK9` writer - LOCK9"]
pub type Lock9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK10` reader - LOCK10"]
pub type Lock10R = crate::BitReader;
#[doc = "Field `LOCK10` writer - LOCK10"]
pub type Lock10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK11` reader - LOCK11"]
pub type Lock11R = crate::BitReader;
#[doc = "Field `LOCK11` writer - LOCK11"]
pub type Lock11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK12` reader - LOCK12"]
pub type Lock12R = crate::BitReader;
#[doc = "Field `LOCK12` writer - LOCK12"]
pub type Lock12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK13` reader - LOCK13"]
pub type Lock13R = crate::BitReader;
#[doc = "Field `LOCK13` writer - LOCK13"]
pub type Lock13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK14` reader - LOCK14"]
pub type Lock14R = crate::BitReader;
#[doc = "Field `LOCK14` writer - LOCK14"]
pub type Lock14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK15` reader - LOCK15"]
pub type Lock15R = crate::BitReader;
#[doc = "Field `LOCK15` writer - LOCK15"]
pub type Lock15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LKEY` reader - LKEY"]
pub type LkeyR = crate::FieldReader<u16>;
#[doc = "Field `LKEY` writer - LKEY"]
pub type LkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - LOCK0"]
    #[inline(always)]
    pub fn lock0(&self) -> Lock0R {
        Lock0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LOCK1"]
    #[inline(always)]
    pub fn lock1(&self) -> Lock1R {
        Lock1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LOCK2"]
    #[inline(always)]
    pub fn lock2(&self) -> Lock2R {
        Lock2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LOCK3"]
    #[inline(always)]
    pub fn lock3(&self) -> Lock3R {
        Lock3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LOCK4"]
    #[inline(always)]
    pub fn lock4(&self) -> Lock4R {
        Lock4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LOCK5"]
    #[inline(always)]
    pub fn lock5(&self) -> Lock5R {
        Lock5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LOCK6"]
    #[inline(always)]
    pub fn lock6(&self) -> Lock6R {
        Lock6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LOCK7"]
    #[inline(always)]
    pub fn lock7(&self) -> Lock7R {
        Lock7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LOCK8"]
    #[inline(always)]
    pub fn lock8(&self) -> Lock8R {
        Lock8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LOCK9"]
    #[inline(always)]
    pub fn lock9(&self) -> Lock9R {
        Lock9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LOCK10"]
    #[inline(always)]
    pub fn lock10(&self) -> Lock10R {
        Lock10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LOCK11"]
    #[inline(always)]
    pub fn lock11(&self) -> Lock11R {
        Lock11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LOCK12"]
    #[inline(always)]
    pub fn lock12(&self) -> Lock12R {
        Lock12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LOCK13"]
    #[inline(always)]
    pub fn lock13(&self) -> Lock13R {
        Lock13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LOCK14"]
    #[inline(always)]
    pub fn lock14(&self) -> Lock14R {
        Lock14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LOCK15"]
    #[inline(always)]
    pub fn lock15(&self) -> Lock15R {
        Lock15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - LKEY"]
    #[inline(always)]
    pub fn lkey(&self) -> LkeyR {
        LkeyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - LOCK0"]
    #[inline(always)]
    #[must_use]
    pub fn lock0(&mut self) -> Lock0W<LockrSpec> {
        Lock0W::new(self, 0)
    }
    #[doc = "Bit 1 - LOCK1"]
    #[inline(always)]
    #[must_use]
    pub fn lock1(&mut self) -> Lock1W<LockrSpec> {
        Lock1W::new(self, 1)
    }
    #[doc = "Bit 2 - LOCK2"]
    #[inline(always)]
    #[must_use]
    pub fn lock2(&mut self) -> Lock2W<LockrSpec> {
        Lock2W::new(self, 2)
    }
    #[doc = "Bit 3 - LOCK3"]
    #[inline(always)]
    #[must_use]
    pub fn lock3(&mut self) -> Lock3W<LockrSpec> {
        Lock3W::new(self, 3)
    }
    #[doc = "Bit 4 - LOCK4"]
    #[inline(always)]
    #[must_use]
    pub fn lock4(&mut self) -> Lock4W<LockrSpec> {
        Lock4W::new(self, 4)
    }
    #[doc = "Bit 5 - LOCK5"]
    #[inline(always)]
    #[must_use]
    pub fn lock5(&mut self) -> Lock5W<LockrSpec> {
        Lock5W::new(self, 5)
    }
    #[doc = "Bit 6 - LOCK6"]
    #[inline(always)]
    #[must_use]
    pub fn lock6(&mut self) -> Lock6W<LockrSpec> {
        Lock6W::new(self, 6)
    }
    #[doc = "Bit 7 - LOCK7"]
    #[inline(always)]
    #[must_use]
    pub fn lock7(&mut self) -> Lock7W<LockrSpec> {
        Lock7W::new(self, 7)
    }
    #[doc = "Bit 8 - LOCK8"]
    #[inline(always)]
    #[must_use]
    pub fn lock8(&mut self) -> Lock8W<LockrSpec> {
        Lock8W::new(self, 8)
    }
    #[doc = "Bit 9 - LOCK9"]
    #[inline(always)]
    #[must_use]
    pub fn lock9(&mut self) -> Lock9W<LockrSpec> {
        Lock9W::new(self, 9)
    }
    #[doc = "Bit 10 - LOCK10"]
    #[inline(always)]
    #[must_use]
    pub fn lock10(&mut self) -> Lock10W<LockrSpec> {
        Lock10W::new(self, 10)
    }
    #[doc = "Bit 11 - LOCK11"]
    #[inline(always)]
    #[must_use]
    pub fn lock11(&mut self) -> Lock11W<LockrSpec> {
        Lock11W::new(self, 11)
    }
    #[doc = "Bit 12 - LOCK12"]
    #[inline(always)]
    #[must_use]
    pub fn lock12(&mut self) -> Lock12W<LockrSpec> {
        Lock12W::new(self, 12)
    }
    #[doc = "Bit 13 - LOCK13"]
    #[inline(always)]
    #[must_use]
    pub fn lock13(&mut self) -> Lock13W<LockrSpec> {
        Lock13W::new(self, 13)
    }
    #[doc = "Bit 14 - LOCK14"]
    #[inline(always)]
    #[must_use]
    pub fn lock14(&mut self) -> Lock14W<LockrSpec> {
        Lock14W::new(self, 14)
    }
    #[doc = "Bit 15 - LOCK15"]
    #[inline(always)]
    #[must_use]
    pub fn lock15(&mut self) -> Lock15W<LockrSpec> {
        Lock15W::new(self, 15)
    }
    #[doc = "Bits 16:31 - LKEY"]
    #[inline(always)]
    #[must_use]
    pub fn lkey(&mut self) -> LkeyW<LockrSpec> {
        LkeyW::new(self, 16)
    }
}
#[doc = "LOCKR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lockr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lockr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockrSpec;
impl crate::RegisterSpec for LockrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lockr::R`](R) reader structure"]
impl crate::Readable for LockrSpec {}
#[doc = "`write(|w| ..)` method takes [`lockr::W`](W) writer structure"]
impl crate::Writable for LockrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOCKR to value 0"]
impl crate::Resettable for LockrSpec {
    const RESET_VALUE: u32 = 0;
}
