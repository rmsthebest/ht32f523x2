#[doc = "Register `PWRCU_BAKCR` reader"]
pub type R = crate::R<PwrcuBakcrSpec>;
#[doc = "Register `PWRCU_BAKCR` writer"]
pub type W = crate::W<PwrcuBakcrSpec>;
#[doc = "Field `BAKRST` reader - BAKRST"]
pub type BakrstR = crate::BitReader;
#[doc = "Field `BAKRST` writer - BAKRST"]
pub type BakrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDOLCM` reader - LDOLCM"]
pub type LdolcmR = crate::BitReader;
#[doc = "Field `LDOLCM` writer - LDOLCM"]
pub type LdolcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDOOFF` reader - LDOOFF"]
pub type LdooffR = crate::BitReader;
#[doc = "Field `LDOOFF` writer - LDOOFF"]
pub type LdooffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMOSON` reader - DMOSON"]
pub type DmosonR = crate::BitReader;
#[doc = "Field `DMOSON` writer - DMOSON"]
pub type DmosonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPEN` reader - WUPEN"]
pub type WupenR = crate::BitReader;
#[doc = "Field `WUPEN` writer - WUPEN"]
pub type WupenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPIEN` reader - WUPIEN"]
pub type WupienR = crate::BitReader;
#[doc = "Field `WUPIEN` writer - WUPIEN"]
pub type WupienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `V15RDYSC` reader - V15RDYSC"]
pub type V15rdyscR = crate::BitReader;
#[doc = "Field `V15RDYSC` writer - V15RDYSC"]
pub type V15rdyscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMOSSTS` reader - DMOSSTS"]
pub type DmosstsR = crate::BitReader;
#[doc = "Field `DMOSSTS` writer - DMOSSTS"]
pub type DmosstsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BAKRST"]
    #[inline(always)]
    pub fn bakrst(&self) -> BakrstR {
        BakrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LDOLCM"]
    #[inline(always)]
    pub fn ldolcm(&self) -> LdolcmR {
        LdolcmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - LDOOFF"]
    #[inline(always)]
    pub fn ldooff(&self) -> LdooffR {
        LdooffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - DMOSON"]
    #[inline(always)]
    pub fn dmoson(&self) -> DmosonR {
        DmosonR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - WUPEN"]
    #[inline(always)]
    pub fn wupen(&self) -> WupenR {
        WupenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WUPIEN"]
    #[inline(always)]
    pub fn wupien(&self) -> WupienR {
        WupienR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - V15RDYSC"]
    #[inline(always)]
    pub fn v15rdysc(&self) -> V15rdyscR {
        V15rdyscR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - DMOSSTS"]
    #[inline(always)]
    pub fn dmossts(&self) -> DmosstsR {
        DmosstsR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BAKRST"]
    #[inline(always)]
    #[must_use]
    pub fn bakrst(&mut self) -> BakrstW<PwrcuBakcrSpec> {
        BakrstW::new(self, 0)
    }
    #[doc = "Bit 1 - LDOLCM"]
    #[inline(always)]
    #[must_use]
    pub fn ldolcm(&mut self) -> LdolcmW<PwrcuBakcrSpec> {
        LdolcmW::new(self, 1)
    }
    #[doc = "Bit 3 - LDOOFF"]
    #[inline(always)]
    #[must_use]
    pub fn ldooff(&mut self) -> LdooffW<PwrcuBakcrSpec> {
        LdooffW::new(self, 3)
    }
    #[doc = "Bit 7 - DMOSON"]
    #[inline(always)]
    #[must_use]
    pub fn dmoson(&mut self) -> DmosonW<PwrcuBakcrSpec> {
        DmosonW::new(self, 7)
    }
    #[doc = "Bit 8 - WUPEN"]
    #[inline(always)]
    #[must_use]
    pub fn wupen(&mut self) -> WupenW<PwrcuBakcrSpec> {
        WupenW::new(self, 8)
    }
    #[doc = "Bit 9 - WUPIEN"]
    #[inline(always)]
    #[must_use]
    pub fn wupien(&mut self) -> WupienW<PwrcuBakcrSpec> {
        WupienW::new(self, 9)
    }
    #[doc = "Bit 12 - V15RDYSC"]
    #[inline(always)]
    #[must_use]
    pub fn v15rdysc(&mut self) -> V15rdyscW<PwrcuBakcrSpec> {
        V15rdyscW::new(self, 12)
    }
    #[doc = "Bit 15 - DMOSSTS"]
    #[inline(always)]
    #[must_use]
    pub fn dmossts(&mut self) -> DmosstsW<PwrcuBakcrSpec> {
        DmosstsW::new(self, 15)
    }
}
#[doc = "PWRCU_BAKCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcu_bakcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcu_bakcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrcuBakcrSpec;
impl crate::RegisterSpec for PwrcuBakcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrcu_bakcr::R`](R) reader structure"]
impl crate::Readable for PwrcuBakcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrcu_bakcr::W`](W) writer structure"]
impl crate::Writable for PwrcuBakcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRCU_BAKCR to value 0"]
impl crate::Resettable for PwrcuBakcrSpec {
    const RESET_VALUE: u32 = 0;
}
