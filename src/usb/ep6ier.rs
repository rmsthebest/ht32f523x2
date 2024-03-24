#[doc = "Register `EP6IER` reader"]
pub type R = crate::R<Ep6ierSpec>;
#[doc = "Register `EP6IER` writer"]
pub type W = crate::W<Ep6ierSpec>;
#[doc = "Field `OTRXIE` reader - OTRXIE"]
pub type OtrxieR = crate::BitReader;
#[doc = "Field `OTRXIE` writer - OTRXIE"]
pub type OtrxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODRXIE` reader - ODRXIE"]
pub type OdrxieR = crate::BitReader;
#[doc = "Field `ODRXIE` writer - ODRXIE"]
pub type OdrxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODOVIE` reader - ODOVIE"]
pub type OdovieR = crate::BitReader;
#[doc = "Field `ODOVIE` writer - ODOVIE"]
pub type OdovieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITRXIE` reader - ITRXIE"]
pub type ItrxieR = crate::BitReader;
#[doc = "Field `ITRXIE` writer - ITRXIE"]
pub type ItrxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDTXIE` reader - IDTXIE"]
pub type IdtxieR = crate::BitReader;
#[doc = "Field `IDTXIE` writer - IDTXIE"]
pub type IdtxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKIE` reader - NAKIE"]
pub type NakieR = crate::BitReader;
#[doc = "Field `NAKIE` writer - NAKIE"]
pub type NakieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STLIE` reader - STLIE"]
pub type StlieR = crate::BitReader;
#[doc = "Field `STLIE` writer - STLIE"]
pub type StlieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UERIE` reader - UERIE"]
pub type UerieR = crate::BitReader;
#[doc = "Field `UERIE` writer - UERIE"]
pub type UerieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - OTRXIE"]
    #[inline(always)]
    pub fn otrxie(&self) -> OtrxieR {
        OtrxieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ODRXIE"]
    #[inline(always)]
    pub fn odrxie(&self) -> OdrxieR {
        OdrxieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ODOVIE"]
    #[inline(always)]
    pub fn odovie(&self) -> OdovieR {
        OdovieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ITRXIE"]
    #[inline(always)]
    pub fn itrxie(&self) -> ItrxieR {
        ItrxieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDTXIE"]
    #[inline(always)]
    pub fn idtxie(&self) -> IdtxieR {
        IdtxieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NAKIE"]
    #[inline(always)]
    pub fn nakie(&self) -> NakieR {
        NakieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - STLIE"]
    #[inline(always)]
    pub fn stlie(&self) -> StlieR {
        StlieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UERIE"]
    #[inline(always)]
    pub fn uerie(&self) -> UerieR {
        UerieR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OTRXIE"]
    #[inline(always)]
    #[must_use]
    pub fn otrxie(&mut self) -> OtrxieW<Ep6ierSpec> {
        OtrxieW::new(self, 0)
    }
    #[doc = "Bit 1 - ODRXIE"]
    #[inline(always)]
    #[must_use]
    pub fn odrxie(&mut self) -> OdrxieW<Ep6ierSpec> {
        OdrxieW::new(self, 1)
    }
    #[doc = "Bit 2 - ODOVIE"]
    #[inline(always)]
    #[must_use]
    pub fn odovie(&mut self) -> OdovieW<Ep6ierSpec> {
        OdovieW::new(self, 2)
    }
    #[doc = "Bit 3 - ITRXIE"]
    #[inline(always)]
    #[must_use]
    pub fn itrxie(&mut self) -> ItrxieW<Ep6ierSpec> {
        ItrxieW::new(self, 3)
    }
    #[doc = "Bit 4 - IDTXIE"]
    #[inline(always)]
    #[must_use]
    pub fn idtxie(&mut self) -> IdtxieW<Ep6ierSpec> {
        IdtxieW::new(self, 4)
    }
    #[doc = "Bit 5 - NAKIE"]
    #[inline(always)]
    #[must_use]
    pub fn nakie(&mut self) -> NakieW<Ep6ierSpec> {
        NakieW::new(self, 5)
    }
    #[doc = "Bit 6 - STLIE"]
    #[inline(always)]
    #[must_use]
    pub fn stlie(&mut self) -> StlieW<Ep6ierSpec> {
        StlieW::new(self, 6)
    }
    #[doc = "Bit 7 - UERIE"]
    #[inline(always)]
    #[must_use]
    pub fn uerie(&mut self) -> UerieW<Ep6ierSpec> {
        UerieW::new(self, 7)
    }
}
#[doc = "EP6IER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep6ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep6ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep6ierSpec;
impl crate::RegisterSpec for Ep6ierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep6ier::R`](R) reader structure"]
impl crate::Readable for Ep6ierSpec {}
#[doc = "`write(|w| ..)` method takes [`ep6ier::W`](W) writer structure"]
impl crate::Writable for Ep6ierSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EP6IER to value 0"]
impl crate::Resettable for Ep6ierSpec {
    const RESET_VALUE: u32 = 0;
}
