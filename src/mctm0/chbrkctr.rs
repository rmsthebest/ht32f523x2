#[doc = "Register `CHBRKCTR` reader"]
pub type R = crate::R<ChbrkctrSpec>;
#[doc = "Register `CHBRKCTR` writer"]
pub type W = crate::W<ChbrkctrSpec>;
#[doc = "Field `BKE0` reader - BKE0"]
pub type Bke0R = crate::BitReader;
#[doc = "Field `BKE0` writer - BKE0"]
pub type Bke0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKP0` reader - BKP0"]
pub type Bkp0R = crate::BitReader;
#[doc = "Field `BKP0` writer - BKP0"]
pub type Bkp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHMOE` reader - CHMOE"]
pub type ChmoeR = crate::BitReader;
#[doc = "Field `CHMOE` writer - CHMOE"]
pub type ChmoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHAOE` reader - CHAOE"]
pub type ChaoeR = crate::BitReader;
#[doc = "Field `CHAOE` writer - CHAOE"]
pub type ChaoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKF0` reader - BKF0"]
pub type Bkf0R = crate::FieldReader;
#[doc = "Field `BKF0` writer - BKF0"]
pub type Bkf0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LOCKLV` reader - LOCKLV"]
pub type LocklvR = crate::FieldReader;
#[doc = "Field `LOCKLV` writer - LOCKLV"]
pub type LocklvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GFSEL0` reader - GFSEL0"]
pub type Gfsel0R = crate::BitReader;
#[doc = "Field `GFSEL0` writer - GFSEL0"]
pub type Gfsel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOSSI` reader - CHOSSI"]
pub type ChossiR = crate::BitReader;
#[doc = "Field `CHOSSI` writer - CHOSSI"]
pub type ChossiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOSSR` reader - CHOSSR"]
pub type ChossrR = crate::BitReader;
#[doc = "Field `CHOSSR` writer - CHOSSR"]
pub type ChossrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDTG` reader - CHDTG"]
pub type ChdtgR = crate::FieldReader;
#[doc = "Field `CHDTG` writer - CHDTG"]
pub type ChdtgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - BKE0"]
    #[inline(always)]
    pub fn bke0(&self) -> Bke0R {
        Bke0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BKP0"]
    #[inline(always)]
    pub fn bkp0(&self) -> Bkp0R {
        Bkp0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - CHMOE"]
    #[inline(always)]
    pub fn chmoe(&self) -> ChmoeR {
        ChmoeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CHAOE"]
    #[inline(always)]
    pub fn chaoe(&self) -> ChaoeR {
        ChaoeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - BKF0"]
    #[inline(always)]
    pub fn bkf0(&self) -> Bkf0R {
        Bkf0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - LOCKLV"]
    #[inline(always)]
    pub fn locklv(&self) -> LocklvR {
        LocklvR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - GFSEL0"]
    #[inline(always)]
    pub fn gfsel0(&self) -> Gfsel0R {
        Gfsel0R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - CHOSSI"]
    #[inline(always)]
    pub fn chossi(&self) -> ChossiR {
        ChossiR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CHOSSR"]
    #[inline(always)]
    pub fn chossr(&self) -> ChossrR {
        ChossrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:31 - CHDTG"]
    #[inline(always)]
    pub fn chdtg(&self) -> ChdtgR {
        ChdtgR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - BKE0"]
    #[inline(always)]
    #[must_use]
    pub fn bke0(&mut self) -> Bke0W<ChbrkctrSpec> {
        Bke0W::new(self, 0)
    }
    #[doc = "Bit 1 - BKP0"]
    #[inline(always)]
    #[must_use]
    pub fn bkp0(&mut self) -> Bkp0W<ChbrkctrSpec> {
        Bkp0W::new(self, 1)
    }
    #[doc = "Bit 4 - CHMOE"]
    #[inline(always)]
    #[must_use]
    pub fn chmoe(&mut self) -> ChmoeW<ChbrkctrSpec> {
        ChmoeW::new(self, 4)
    }
    #[doc = "Bit 5 - CHAOE"]
    #[inline(always)]
    #[must_use]
    pub fn chaoe(&mut self) -> ChaoeW<ChbrkctrSpec> {
        ChaoeW::new(self, 5)
    }
    #[doc = "Bits 8:11 - BKF0"]
    #[inline(always)]
    #[must_use]
    pub fn bkf0(&mut self) -> Bkf0W<ChbrkctrSpec> {
        Bkf0W::new(self, 8)
    }
    #[doc = "Bits 16:17 - LOCKLV"]
    #[inline(always)]
    #[must_use]
    pub fn locklv(&mut self) -> LocklvW<ChbrkctrSpec> {
        LocklvW::new(self, 16)
    }
    #[doc = "Bit 18 - GFSEL0"]
    #[inline(always)]
    #[must_use]
    pub fn gfsel0(&mut self) -> Gfsel0W<ChbrkctrSpec> {
        Gfsel0W::new(self, 18)
    }
    #[doc = "Bit 20 - CHOSSI"]
    #[inline(always)]
    #[must_use]
    pub fn chossi(&mut self) -> ChossiW<ChbrkctrSpec> {
        ChossiW::new(self, 20)
    }
    #[doc = "Bit 21 - CHOSSR"]
    #[inline(always)]
    #[must_use]
    pub fn chossr(&mut self) -> ChossrW<ChbrkctrSpec> {
        ChossrW::new(self, 21)
    }
    #[doc = "Bits 24:31 - CHDTG"]
    #[inline(always)]
    #[must_use]
    pub fn chdtg(&mut self) -> ChdtgW<ChbrkctrSpec> {
        ChdtgW::new(self, 24)
    }
}
#[doc = "CHBRKCTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chbrkctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chbrkctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChbrkctrSpec;
impl crate::RegisterSpec for ChbrkctrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chbrkctr::R`](R) reader structure"]
impl crate::Readable for ChbrkctrSpec {}
#[doc = "`write(|w| ..)` method takes [`chbrkctr::W`](W) writer structure"]
impl crate::Writable for ChbrkctrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHBRKCTR to value 0"]
impl crate::Resettable for ChbrkctrSpec {
    const RESET_VALUE: u32 = 0;
}
