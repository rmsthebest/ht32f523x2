#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `MODE0` reader - MODE0"]
pub type Mode0R = crate::FieldReader;
#[doc = "Field `MODE0` writer - MODE0"]
pub type Mode0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE1` reader - MODE1"]
pub type Mode1R = crate::FieldReader;
#[doc = "Field `MODE1` writer - MODE1"]
pub type Mode1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE2` reader - MODE2"]
pub type Mode2R = crate::FieldReader;
#[doc = "Field `MODE2` writer - MODE2"]
pub type Mode2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE3` reader - MODE3"]
pub type Mode3R = crate::FieldReader;
#[doc = "Field `MODE3` writer - MODE3"]
pub type Mode3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BANKEN0` reader - BANKEN0"]
pub type Banken0R = crate::BitReader;
#[doc = "Field `BANKEN0` writer - BANKEN0"]
pub type Banken0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BANKEN1` reader - BANKEN1"]
pub type Banken1R = crate::BitReader;
#[doc = "Field `BANKEN1` writer - BANKEN1"]
pub type Banken1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BANKEN2` reader - BANKEN2"]
pub type Banken2R = crate::BitReader;
#[doc = "Field `BANKEN2` writer - BANKEN2"]
pub type Banken2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BANKEN3` reader - BANKEN3"]
pub type Banken3R = crate::BitReader;
#[doc = "Field `BANKEN3` writer - BANKEN3"]
pub type Banken3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOIDLE0` reader - NOIDLE0"]
pub type Noidle0R = crate::BitReader;
#[doc = "Field `NOIDLE0` writer - NOIDLE0"]
pub type Noidle0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOIDLE1` reader - NOIDLE1"]
pub type Noidle1R = crate::BitReader;
#[doc = "Field `NOIDLE1` writer - NOIDLE1"]
pub type Noidle1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOIDLE2` reader - NOIDLE2"]
pub type Noidle2R = crate::BitReader;
#[doc = "Field `NOIDLE2` writer - NOIDLE2"]
pub type Noidle2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOIDLE3` reader - NOIDLE3"]
pub type Noidle3R = crate::BitReader;
#[doc = "Field `NOIDLE3` writer - NOIDLE3"]
pub type Noidle3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLET` reader - IDLET"]
pub type IdletR = crate::FieldReader;
#[doc = "Field `IDLET` writer - IDLET"]
pub type IdletW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - MODE0"]
    #[inline(always)]
    pub fn mode0(&self) -> Mode0R {
        Mode0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - MODE1"]
    #[inline(always)]
    pub fn mode1(&self) -> Mode1R {
        Mode1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - MODE2"]
    #[inline(always)]
    pub fn mode2(&self) -> Mode2R {
        Mode2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - MODE3"]
    #[inline(always)]
    pub fn mode3(&self) -> Mode3R {
        Mode3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - BANKEN0"]
    #[inline(always)]
    pub fn banken0(&self) -> Banken0R {
        Banken0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BANKEN1"]
    #[inline(always)]
    pub fn banken1(&self) -> Banken1R {
        Banken1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BANKEN2"]
    #[inline(always)]
    pub fn banken2(&self) -> Banken2R {
        Banken2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BANKEN3"]
    #[inline(always)]
    pub fn banken3(&self) -> Banken3R {
        Banken3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NOIDLE0"]
    #[inline(always)]
    pub fn noidle0(&self) -> Noidle0R {
        Noidle0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NOIDLE1"]
    #[inline(always)]
    pub fn noidle1(&self) -> Noidle1R {
        Noidle1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NOIDLE2"]
    #[inline(always)]
    pub fn noidle2(&self) -> Noidle2R {
        Noidle2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NOIDLE3"]
    #[inline(always)]
    pub fn noidle3(&self) -> Noidle3R {
        Noidle3R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 28:31 - IDLET"]
    #[inline(always)]
    pub fn idlet(&self) -> IdletR {
        IdletR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - MODE0"]
    #[inline(always)]
    #[must_use]
    pub fn mode0(&mut self) -> Mode0W<CrSpec> {
        Mode0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - MODE1"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> Mode1W<CrSpec> {
        Mode1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - MODE2"]
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> Mode2W<CrSpec> {
        Mode2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - MODE3"]
    #[inline(always)]
    #[must_use]
    pub fn mode3(&mut self) -> Mode3W<CrSpec> {
        Mode3W::new(self, 6)
    }
    #[doc = "Bit 8 - BANKEN0"]
    #[inline(always)]
    #[must_use]
    pub fn banken0(&mut self) -> Banken0W<CrSpec> {
        Banken0W::new(self, 8)
    }
    #[doc = "Bit 9 - BANKEN1"]
    #[inline(always)]
    #[must_use]
    pub fn banken1(&mut self) -> Banken1W<CrSpec> {
        Banken1W::new(self, 9)
    }
    #[doc = "Bit 10 - BANKEN2"]
    #[inline(always)]
    #[must_use]
    pub fn banken2(&mut self) -> Banken2W<CrSpec> {
        Banken2W::new(self, 10)
    }
    #[doc = "Bit 11 - BANKEN3"]
    #[inline(always)]
    #[must_use]
    pub fn banken3(&mut self) -> Banken3W<CrSpec> {
        Banken3W::new(self, 11)
    }
    #[doc = "Bit 12 - NOIDLE0"]
    #[inline(always)]
    #[must_use]
    pub fn noidle0(&mut self) -> Noidle0W<CrSpec> {
        Noidle0W::new(self, 12)
    }
    #[doc = "Bit 13 - NOIDLE1"]
    #[inline(always)]
    #[must_use]
    pub fn noidle1(&mut self) -> Noidle1W<CrSpec> {
        Noidle1W::new(self, 13)
    }
    #[doc = "Bit 14 - NOIDLE2"]
    #[inline(always)]
    #[must_use]
    pub fn noidle2(&mut self) -> Noidle2W<CrSpec> {
        Noidle2W::new(self, 14)
    }
    #[doc = "Bit 15 - NOIDLE3"]
    #[inline(always)]
    #[must_use]
    pub fn noidle3(&mut self) -> Noidle3W<CrSpec> {
        Noidle3W::new(self, 15)
    }
    #[doc = "Bits 28:31 - IDLET"]
    #[inline(always)]
    #[must_use]
    pub fn idlet(&mut self) -> IdletW<CrSpec> {
        IdletW::new(self, 28)
    }
}
#[doc = "CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}
