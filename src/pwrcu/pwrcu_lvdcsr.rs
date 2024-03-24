#[doc = "Register `PWRCU_LVDCSR` reader"]
pub type R = crate::R<PwrcuLvdcsrSpec>;
#[doc = "Register `PWRCU_LVDCSR` writer"]
pub type W = crate::W<PwrcuLvdcsrSpec>;
#[doc = "Field `BODEN` reader - BODEN"]
pub type BodenR = crate::BitReader;
#[doc = "Field `BODEN` writer - BODEN"]
pub type BodenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODRIS` reader - BODRIS"]
pub type BodrisR = crate::BitReader;
#[doc = "Field `BODRIS` writer - BODRIS"]
pub type BodrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODF` reader - BODF"]
pub type BodfR = crate::BitReader;
#[doc = "Field `BODF` writer - BODF"]
pub type BodfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVDEN` reader - LVDEN"]
pub type LvdenR = crate::BitReader;
#[doc = "Field `LVDEN` writer - LVDEN"]
pub type LvdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVDS01` reader - LVDS01"]
pub type Lvds01R = crate::FieldReader;
#[doc = "Field `LVDS01` writer - LVDS01"]
pub type Lvds01W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LVDF` reader - LVDF"]
pub type LvdfR = crate::BitReader;
#[doc = "Field `LVDF` writer - LVDF"]
pub type LvdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVDIWEN` reader - LVDIWEN"]
pub type LvdiwenR = crate::BitReader;
#[doc = "Field `LVDIWEN` writer - LVDIWEN"]
pub type LvdiwenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVDEWEN` reader - LVDEWEN"]
pub type LvdewenR = crate::BitReader;
#[doc = "Field `LVDEWEN` writer - LVDEWEN"]
pub type LvdewenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVDS2` reader - LVDS2"]
pub type Lvds2R = crate::BitReader;
#[doc = "Field `LVDS2` writer - LVDS2"]
pub type Lvds2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BODEN"]
    #[inline(always)]
    pub fn boden(&self) -> BodenR {
        BodenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BODRIS"]
    #[inline(always)]
    pub fn bodris(&self) -> BodrisR {
        BodrisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - BODF"]
    #[inline(always)]
    pub fn bodf(&self) -> BodfR {
        BodfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - LVDEN"]
    #[inline(always)]
    pub fn lvden(&self) -> LvdenR {
        LvdenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - LVDS01"]
    #[inline(always)]
    pub fn lvds01(&self) -> Lvds01R {
        Lvds01R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - LVDF"]
    #[inline(always)]
    pub fn lvdf(&self) -> LvdfR {
        LvdfR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LVDIWEN"]
    #[inline(always)]
    pub fn lvdiwen(&self) -> LvdiwenR {
        LvdiwenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LVDEWEN"]
    #[inline(always)]
    pub fn lvdewen(&self) -> LvdewenR {
        LvdewenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - LVDS2"]
    #[inline(always)]
    pub fn lvds2(&self) -> Lvds2R {
        Lvds2R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BODEN"]
    #[inline(always)]
    #[must_use]
    pub fn boden(&mut self) -> BodenW<PwrcuLvdcsrSpec> {
        BodenW::new(self, 0)
    }
    #[doc = "Bit 1 - BODRIS"]
    #[inline(always)]
    #[must_use]
    pub fn bodris(&mut self) -> BodrisW<PwrcuLvdcsrSpec> {
        BodrisW::new(self, 1)
    }
    #[doc = "Bit 3 - BODF"]
    #[inline(always)]
    #[must_use]
    pub fn bodf(&mut self) -> BodfW<PwrcuLvdcsrSpec> {
        BodfW::new(self, 3)
    }
    #[doc = "Bit 16 - LVDEN"]
    #[inline(always)]
    #[must_use]
    pub fn lvden(&mut self) -> LvdenW<PwrcuLvdcsrSpec> {
        LvdenW::new(self, 16)
    }
    #[doc = "Bits 17:18 - LVDS01"]
    #[inline(always)]
    #[must_use]
    pub fn lvds01(&mut self) -> Lvds01W<PwrcuLvdcsrSpec> {
        Lvds01W::new(self, 17)
    }
    #[doc = "Bit 19 - LVDF"]
    #[inline(always)]
    #[must_use]
    pub fn lvdf(&mut self) -> LvdfW<PwrcuLvdcsrSpec> {
        LvdfW::new(self, 19)
    }
    #[doc = "Bit 20 - LVDIWEN"]
    #[inline(always)]
    #[must_use]
    pub fn lvdiwen(&mut self) -> LvdiwenW<PwrcuLvdcsrSpec> {
        LvdiwenW::new(self, 20)
    }
    #[doc = "Bit 21 - LVDEWEN"]
    #[inline(always)]
    #[must_use]
    pub fn lvdewen(&mut self) -> LvdewenW<PwrcuLvdcsrSpec> {
        LvdewenW::new(self, 21)
    }
    #[doc = "Bit 22 - LVDS2"]
    #[inline(always)]
    #[must_use]
    pub fn lvds2(&mut self) -> Lvds2W<PwrcuLvdcsrSpec> {
        Lvds2W::new(self, 22)
    }
}
#[doc = "PWRCU_LVDCSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcu_lvdcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcu_lvdcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrcuLvdcsrSpec;
impl crate::RegisterSpec for PwrcuLvdcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrcu_lvdcsr::R`](R) reader structure"]
impl crate::Readable for PwrcuLvdcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrcu_lvdcsr::W`](W) writer structure"]
impl crate::Writable for PwrcuLvdcsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRCU_LVDCSR to value 0"]
impl crate::Resettable for PwrcuLvdcsrSpec {
    const RESET_VALUE: u32 = 0;
}
