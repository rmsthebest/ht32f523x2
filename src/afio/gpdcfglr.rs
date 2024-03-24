#[doc = "Register `GPDCFGLR` reader"]
pub type R = crate::R<GpdcfglrSpec>;
#[doc = "Register `GPDCFGLR` writer"]
pub type W = crate::W<GpdcfglrSpec>;
#[doc = "Field `CFG0` reader - CFG0"]
pub type Cfg0R = crate::FieldReader;
#[doc = "Field `CFG0` writer - CFG0"]
pub type Cfg0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG1` reader - CFG1"]
pub type Cfg1R = crate::FieldReader;
#[doc = "Field `CFG1` writer - CFG1"]
pub type Cfg1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG2` reader - CFG2"]
pub type Cfg2R = crate::FieldReader;
#[doc = "Field `CFG2` writer - CFG2"]
pub type Cfg2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG3` reader - CFG3"]
pub type Cfg3R = crate::FieldReader;
#[doc = "Field `CFG3` writer - CFG3"]
pub type Cfg3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG4` reader - CFG4"]
pub type Cfg4R = crate::FieldReader;
#[doc = "Field `CFG4` writer - CFG4"]
pub type Cfg4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG5` reader - CFG5"]
pub type Cfg5R = crate::FieldReader;
#[doc = "Field `CFG5` writer - CFG5"]
pub type Cfg5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG6` reader - CFG6"]
pub type Cfg6R = crate::FieldReader;
#[doc = "Field `CFG6` writer - CFG6"]
pub type Cfg6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG7` reader - CFG7"]
pub type Cfg7R = crate::FieldReader;
#[doc = "Field `CFG7` writer - CFG7"]
pub type Cfg7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - CFG0"]
    #[inline(always)]
    pub fn cfg0(&self) -> Cfg0R {
        Cfg0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CFG1"]
    #[inline(always)]
    pub fn cfg1(&self) -> Cfg1R {
        Cfg1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - CFG2"]
    #[inline(always)]
    pub fn cfg2(&self) -> Cfg2R {
        Cfg2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - CFG3"]
    #[inline(always)]
    pub fn cfg3(&self) -> Cfg3R {
        Cfg3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - CFG4"]
    #[inline(always)]
    pub fn cfg4(&self) -> Cfg4R {
        Cfg4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - CFG5"]
    #[inline(always)]
    pub fn cfg5(&self) -> Cfg5R {
        Cfg5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - CFG6"]
    #[inline(always)]
    pub fn cfg6(&self) -> Cfg6R {
        Cfg6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - CFG7"]
    #[inline(always)]
    pub fn cfg7(&self) -> Cfg7R {
        Cfg7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CFG0"]
    #[inline(always)]
    #[must_use]
    pub fn cfg0(&mut self) -> Cfg0W<GpdcfglrSpec> {
        Cfg0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - CFG1"]
    #[inline(always)]
    #[must_use]
    pub fn cfg1(&mut self) -> Cfg1W<GpdcfglrSpec> {
        Cfg1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - CFG2"]
    #[inline(always)]
    #[must_use]
    pub fn cfg2(&mut self) -> Cfg2W<GpdcfglrSpec> {
        Cfg2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - CFG3"]
    #[inline(always)]
    #[must_use]
    pub fn cfg3(&mut self) -> Cfg3W<GpdcfglrSpec> {
        Cfg3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - CFG4"]
    #[inline(always)]
    #[must_use]
    pub fn cfg4(&mut self) -> Cfg4W<GpdcfglrSpec> {
        Cfg4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - CFG5"]
    #[inline(always)]
    #[must_use]
    pub fn cfg5(&mut self) -> Cfg5W<GpdcfglrSpec> {
        Cfg5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - CFG6"]
    #[inline(always)]
    #[must_use]
    pub fn cfg6(&mut self) -> Cfg6W<GpdcfglrSpec> {
        Cfg6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - CFG7"]
    #[inline(always)]
    #[must_use]
    pub fn cfg7(&mut self) -> Cfg7W<GpdcfglrSpec> {
        Cfg7W::new(self, 28)
    }
}
#[doc = "GPDCFGLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdcfglr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdcfglr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpdcfglrSpec;
impl crate::RegisterSpec for GpdcfglrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdcfglr::R`](R) reader structure"]
impl crate::Readable for GpdcfglrSpec {}
#[doc = "`write(|w| ..)` method takes [`gpdcfglr::W`](W) writer structure"]
impl crate::Writable for GpdcfglrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPDCFGLR to value 0"]
impl crate::Resettable for GpdcfglrSpec {
    const RESET_VALUE: u32 = 0;
}
