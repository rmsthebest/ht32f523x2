#[doc = "Register `MCUDBGCR` reader"]
pub type R = crate::R<McudbgcrSpec>;
#[doc = "Register `MCUDBGCR` writer"]
pub type W = crate::W<McudbgcrSpec>;
#[doc = "Field `DBSLP` reader - DBSLP"]
pub type DbslpR = crate::BitReader;
#[doc = "Field `DBSLP` writer - DBSLP"]
pub type DbslpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBDSLP1` reader - DBDSLP1"]
pub type Dbdslp1R = crate::BitReader;
#[doc = "Field `DBDSLP1` writer - DBDSLP1"]
pub type Dbdslp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBPD` reader - DBPD"]
pub type DbpdR = crate::BitReader;
#[doc = "Field `DBPD` writer - DBPD"]
pub type DbpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBWDT` reader - DBWDT"]
pub type DbwdtR = crate::BitReader;
#[doc = "Field `DBWDT` writer - DBWDT"]
pub type DbwdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBMCTM0` reader - DBMCTM0"]
pub type Dbmctm0R = crate::BitReader;
#[doc = "Field `DBMCTM0` writer - DBMCTM0"]
pub type Dbmctm0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGPTM0` reader - DBGPTM0"]
pub type Dbgptm0R = crate::BitReader;
#[doc = "Field `DBGPTM0` writer - DBGPTM0"]
pub type Dbgptm0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGPTM1` reader - DBGPTM1"]
pub type Dbgptm1R = crate::BitReader;
#[doc = "Field `DBGPTM1` writer - DBGPTM1"]
pub type Dbgptm1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBUSR0` reader - DBUSR0"]
pub type Dbusr0R = crate::BitReader;
#[doc = "Field `DBUSR0` writer - DBUSR0"]
pub type Dbusr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBUSR1` reader - DBUSR1"]
pub type Dbusr1R = crate::BitReader;
#[doc = "Field `DBUSR1` writer - DBUSR1"]
pub type Dbusr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBSPI0` reader - DBSPI0"]
pub type Dbspi0R = crate::BitReader;
#[doc = "Field `DBSPI0` writer - DBSPI0"]
pub type Dbspi0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBSPI1` reader - DBSPI1"]
pub type Dbspi1R = crate::BitReader;
#[doc = "Field `DBSPI1` writer - DBSPI1"]
pub type Dbspi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBI2C0` reader - DBI2C0"]
pub type Dbi2c0R = crate::BitReader;
#[doc = "Field `DBI2C0` writer - DBI2C0"]
pub type Dbi2c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBI2C1` reader - DBI2C1"]
pub type Dbi2c1R = crate::BitReader;
#[doc = "Field `DBI2C1` writer - DBI2C1"]
pub type Dbi2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBDSLP2` reader - DBDSLP2"]
pub type Dbdslp2R = crate::BitReader;
#[doc = "Field `DBDSLP2` writer - DBDSLP2"]
pub type Dbdslp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBSCI0` reader - DBSCI0"]
pub type Dbsci0R = crate::BitReader;
#[doc = "Field `DBSCI0` writer - DBSCI0"]
pub type Dbsci0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBBFTM0` reader - DBBFTM0"]
pub type Dbbftm0R = crate::BitReader;
#[doc = "Field `DBBFTM0` writer - DBBFTM0"]
pub type Dbbftm0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBBFTM1` reader - DBBFTM1"]
pub type Dbbftm1R = crate::BitReader;
#[doc = "Field `DBBFTM1` writer - DBBFTM1"]
pub type Dbbftm1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBUR0` reader - DBUR0"]
pub type Dbur0R = crate::BitReader;
#[doc = "Field `DBUR0` writer - DBUR0"]
pub type Dbur0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBUR1` reader - DBUR1"]
pub type Dbur1R = crate::BitReader;
#[doc = "Field `DBUR1` writer - DBUR1"]
pub type Dbur1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBTRACE` reader - DBTRACE"]
pub type DbtraceR = crate::BitReader;
#[doc = "Field `DBTRACE` writer - DBTRACE"]
pub type DbtraceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBSCI1` reader - DBSCI1"]
pub type Dbsci1R = crate::BitReader;
#[doc = "Field `DBSCI1` writer - DBSCI1"]
pub type Dbsci1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBSCTM0` reader - DBSCTM0"]
pub type Dbsctm0R = crate::BitReader;
#[doc = "Field `DBSCTM0` writer - DBSCTM0"]
pub type Dbsctm0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBSCTM1` reader - DBSCTM1"]
pub type Dbsctm1R = crate::BitReader;
#[doc = "Field `DBSCTM1` writer - DBSCTM1"]
pub type Dbsctm1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DBSLP"]
    #[inline(always)]
    pub fn dbslp(&self) -> DbslpR {
        DbslpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DBDSLP1"]
    #[inline(always)]
    pub fn dbdslp1(&self) -> Dbdslp1R {
        Dbdslp1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DBPD"]
    #[inline(always)]
    pub fn dbpd(&self) -> DbpdR {
        DbpdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DBWDT"]
    #[inline(always)]
    pub fn dbwdt(&self) -> DbwdtR {
        DbwdtR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DBMCTM0"]
    #[inline(always)]
    pub fn dbmctm0(&self) -> Dbmctm0R {
        Dbmctm0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - DBGPTM0"]
    #[inline(always)]
    pub fn dbgptm0(&self) -> Dbgptm0R {
        Dbgptm0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DBGPTM1"]
    #[inline(always)]
    pub fn dbgptm1(&self) -> Dbgptm1R {
        Dbgptm1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DBUSR0"]
    #[inline(always)]
    pub fn dbusr0(&self) -> Dbusr0R {
        Dbusr0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DBUSR1"]
    #[inline(always)]
    pub fn dbusr1(&self) -> Dbusr1R {
        Dbusr1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DBSPI0"]
    #[inline(always)]
    pub fn dbspi0(&self) -> Dbspi0R {
        Dbspi0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DBSPI1"]
    #[inline(always)]
    pub fn dbspi1(&self) -> Dbspi1R {
        Dbspi1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DBI2C0"]
    #[inline(always)]
    pub fn dbi2c0(&self) -> Dbi2c0R {
        Dbi2c0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DBI2C1"]
    #[inline(always)]
    pub fn dbi2c1(&self) -> Dbi2c1R {
        Dbi2c1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DBDSLP2"]
    #[inline(always)]
    pub fn dbdslp2(&self) -> Dbdslp2R {
        Dbdslp2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DBSCI0"]
    #[inline(always)]
    pub fn dbsci0(&self) -> Dbsci0R {
        Dbsci0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DBBFTM0"]
    #[inline(always)]
    pub fn dbbftm0(&self) -> Dbbftm0R {
        Dbbftm0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DBBFTM1"]
    #[inline(always)]
    pub fn dbbftm1(&self) -> Dbbftm1R {
        Dbbftm1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DBUR0"]
    #[inline(always)]
    pub fn dbur0(&self) -> Dbur0R {
        Dbur0R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DBUR1"]
    #[inline(always)]
    pub fn dbur1(&self) -> Dbur1R {
        Dbur1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DBTRACE"]
    #[inline(always)]
    pub fn dbtrace(&self) -> DbtraceR {
        DbtraceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DBSCI1"]
    #[inline(always)]
    pub fn dbsci1(&self) -> Dbsci1R {
        Dbsci1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DBSCTM0"]
    #[inline(always)]
    pub fn dbsctm0(&self) -> Dbsctm0R {
        Dbsctm0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DBSCTM1"]
    #[inline(always)]
    pub fn dbsctm1(&self) -> Dbsctm1R {
        Dbsctm1R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DBSLP"]
    #[inline(always)]
    #[must_use]
    pub fn dbslp(&mut self) -> DbslpW<McudbgcrSpec> {
        DbslpW::new(self, 0)
    }
    #[doc = "Bit 1 - DBDSLP1"]
    #[inline(always)]
    #[must_use]
    pub fn dbdslp1(&mut self) -> Dbdslp1W<McudbgcrSpec> {
        Dbdslp1W::new(self, 1)
    }
    #[doc = "Bit 2 - DBPD"]
    #[inline(always)]
    #[must_use]
    pub fn dbpd(&mut self) -> DbpdW<McudbgcrSpec> {
        DbpdW::new(self, 2)
    }
    #[doc = "Bit 3 - DBWDT"]
    #[inline(always)]
    #[must_use]
    pub fn dbwdt(&mut self) -> DbwdtW<McudbgcrSpec> {
        DbwdtW::new(self, 3)
    }
    #[doc = "Bit 4 - DBMCTM0"]
    #[inline(always)]
    #[must_use]
    pub fn dbmctm0(&mut self) -> Dbmctm0W<McudbgcrSpec> {
        Dbmctm0W::new(self, 4)
    }
    #[doc = "Bit 6 - DBGPTM0"]
    #[inline(always)]
    #[must_use]
    pub fn dbgptm0(&mut self) -> Dbgptm0W<McudbgcrSpec> {
        Dbgptm0W::new(self, 6)
    }
    #[doc = "Bit 7 - DBGPTM1"]
    #[inline(always)]
    #[must_use]
    pub fn dbgptm1(&mut self) -> Dbgptm1W<McudbgcrSpec> {
        Dbgptm1W::new(self, 7)
    }
    #[doc = "Bit 8 - DBUSR0"]
    #[inline(always)]
    #[must_use]
    pub fn dbusr0(&mut self) -> Dbusr0W<McudbgcrSpec> {
        Dbusr0W::new(self, 8)
    }
    #[doc = "Bit 9 - DBUSR1"]
    #[inline(always)]
    #[must_use]
    pub fn dbusr1(&mut self) -> Dbusr1W<McudbgcrSpec> {
        Dbusr1W::new(self, 9)
    }
    #[doc = "Bit 10 - DBSPI0"]
    #[inline(always)]
    #[must_use]
    pub fn dbspi0(&mut self) -> Dbspi0W<McudbgcrSpec> {
        Dbspi0W::new(self, 10)
    }
    #[doc = "Bit 11 - DBSPI1"]
    #[inline(always)]
    #[must_use]
    pub fn dbspi1(&mut self) -> Dbspi1W<McudbgcrSpec> {
        Dbspi1W::new(self, 11)
    }
    #[doc = "Bit 12 - DBI2C0"]
    #[inline(always)]
    #[must_use]
    pub fn dbi2c0(&mut self) -> Dbi2c0W<McudbgcrSpec> {
        Dbi2c0W::new(self, 12)
    }
    #[doc = "Bit 13 - DBI2C1"]
    #[inline(always)]
    #[must_use]
    pub fn dbi2c1(&mut self) -> Dbi2c1W<McudbgcrSpec> {
        Dbi2c1W::new(self, 13)
    }
    #[doc = "Bit 14 - DBDSLP2"]
    #[inline(always)]
    #[must_use]
    pub fn dbdslp2(&mut self) -> Dbdslp2W<McudbgcrSpec> {
        Dbdslp2W::new(self, 14)
    }
    #[doc = "Bit 15 - DBSCI0"]
    #[inline(always)]
    #[must_use]
    pub fn dbsci0(&mut self) -> Dbsci0W<McudbgcrSpec> {
        Dbsci0W::new(self, 15)
    }
    #[doc = "Bit 16 - DBBFTM0"]
    #[inline(always)]
    #[must_use]
    pub fn dbbftm0(&mut self) -> Dbbftm0W<McudbgcrSpec> {
        Dbbftm0W::new(self, 16)
    }
    #[doc = "Bit 17 - DBBFTM1"]
    #[inline(always)]
    #[must_use]
    pub fn dbbftm1(&mut self) -> Dbbftm1W<McudbgcrSpec> {
        Dbbftm1W::new(self, 17)
    }
    #[doc = "Bit 18 - DBUR0"]
    #[inline(always)]
    #[must_use]
    pub fn dbur0(&mut self) -> Dbur0W<McudbgcrSpec> {
        Dbur0W::new(self, 18)
    }
    #[doc = "Bit 19 - DBUR1"]
    #[inline(always)]
    #[must_use]
    pub fn dbur1(&mut self) -> Dbur1W<McudbgcrSpec> {
        Dbur1W::new(self, 19)
    }
    #[doc = "Bit 20 - DBTRACE"]
    #[inline(always)]
    #[must_use]
    pub fn dbtrace(&mut self) -> DbtraceW<McudbgcrSpec> {
        DbtraceW::new(self, 20)
    }
    #[doc = "Bit 21 - DBSCI1"]
    #[inline(always)]
    #[must_use]
    pub fn dbsci1(&mut self) -> Dbsci1W<McudbgcrSpec> {
        Dbsci1W::new(self, 21)
    }
    #[doc = "Bit 22 - DBSCTM0"]
    #[inline(always)]
    #[must_use]
    pub fn dbsctm0(&mut self) -> Dbsctm0W<McudbgcrSpec> {
        Dbsctm0W::new(self, 22)
    }
    #[doc = "Bit 23 - DBSCTM1"]
    #[inline(always)]
    #[must_use]
    pub fn dbsctm1(&mut self) -> Dbsctm1W<McudbgcrSpec> {
        Dbsctm1W::new(self, 23)
    }
}
#[doc = "MCUDBGCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcudbgcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcudbgcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McudbgcrSpec;
impl crate::RegisterSpec for McudbgcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcudbgcr::R`](R) reader structure"]
impl crate::Readable for McudbgcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcudbgcr::W`](W) writer structure"]
impl crate::Writable for McudbgcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCUDBGCR to value 0"]
impl crate::Resettable for McudbgcrSpec {
    const RESET_VALUE: u32 = 0;
}
