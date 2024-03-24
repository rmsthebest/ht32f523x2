#[doc = "Register `GPDCFGHR` reader"]
pub type R = crate::R<GpdcfghrSpec>;
#[doc = "Register `GPDCFGHR` writer"]
pub type W = crate::W<GpdcfghrSpec>;
#[doc = "Field `CFG8` reader - CFG8"]
pub type Cfg8R = crate::FieldReader;
#[doc = "Field `CFG8` writer - CFG8"]
pub type Cfg8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG9` reader - CFG9"]
pub type Cfg9R = crate::FieldReader;
#[doc = "Field `CFG9` writer - CFG9"]
pub type Cfg9W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG10` reader - CFG10"]
pub type Cfg10R = crate::FieldReader;
#[doc = "Field `CFG10` writer - CFG10"]
pub type Cfg10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG11` reader - CFG11"]
pub type Cfg11R = crate::FieldReader;
#[doc = "Field `CFG11` writer - CFG11"]
pub type Cfg11W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG12` reader - CFG12"]
pub type Cfg12R = crate::FieldReader;
#[doc = "Field `CFG12` writer - CFG12"]
pub type Cfg12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG13` reader - CFG13"]
pub type Cfg13R = crate::FieldReader;
#[doc = "Field `CFG13` writer - CFG13"]
pub type Cfg13W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG14` reader - CFG14"]
pub type Cfg14R = crate::FieldReader;
#[doc = "Field `CFG14` writer - CFG14"]
pub type Cfg14W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG15` reader - CFG15"]
pub type Cfg15R = crate::FieldReader;
#[doc = "Field `CFG15` writer - CFG15"]
pub type Cfg15W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - CFG8"]
    #[inline(always)]
    pub fn cfg8(&self) -> Cfg8R {
        Cfg8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CFG9"]
    #[inline(always)]
    pub fn cfg9(&self) -> Cfg9R {
        Cfg9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - CFG10"]
    #[inline(always)]
    pub fn cfg10(&self) -> Cfg10R {
        Cfg10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - CFG11"]
    #[inline(always)]
    pub fn cfg11(&self) -> Cfg11R {
        Cfg11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - CFG12"]
    #[inline(always)]
    pub fn cfg12(&self) -> Cfg12R {
        Cfg12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - CFG13"]
    #[inline(always)]
    pub fn cfg13(&self) -> Cfg13R {
        Cfg13R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - CFG14"]
    #[inline(always)]
    pub fn cfg14(&self) -> Cfg14R {
        Cfg14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - CFG15"]
    #[inline(always)]
    pub fn cfg15(&self) -> Cfg15R {
        Cfg15R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CFG8"]
    #[inline(always)]
    #[must_use]
    pub fn cfg8(&mut self) -> Cfg8W<GpdcfghrSpec> {
        Cfg8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - CFG9"]
    #[inline(always)]
    #[must_use]
    pub fn cfg9(&mut self) -> Cfg9W<GpdcfghrSpec> {
        Cfg9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - CFG10"]
    #[inline(always)]
    #[must_use]
    pub fn cfg10(&mut self) -> Cfg10W<GpdcfghrSpec> {
        Cfg10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - CFG11"]
    #[inline(always)]
    #[must_use]
    pub fn cfg11(&mut self) -> Cfg11W<GpdcfghrSpec> {
        Cfg11W::new(self, 12)
    }
    #[doc = "Bits 16:19 - CFG12"]
    #[inline(always)]
    #[must_use]
    pub fn cfg12(&mut self) -> Cfg12W<GpdcfghrSpec> {
        Cfg12W::new(self, 16)
    }
    #[doc = "Bits 20:23 - CFG13"]
    #[inline(always)]
    #[must_use]
    pub fn cfg13(&mut self) -> Cfg13W<GpdcfghrSpec> {
        Cfg13W::new(self, 20)
    }
    #[doc = "Bits 24:27 - CFG14"]
    #[inline(always)]
    #[must_use]
    pub fn cfg14(&mut self) -> Cfg14W<GpdcfghrSpec> {
        Cfg14W::new(self, 24)
    }
    #[doc = "Bits 28:31 - CFG15"]
    #[inline(always)]
    #[must_use]
    pub fn cfg15(&mut self) -> Cfg15W<GpdcfghrSpec> {
        Cfg15W::new(self, 28)
    }
}
#[doc = "GPDCFGHR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdcfghr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdcfghr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpdcfghrSpec;
impl crate::RegisterSpec for GpdcfghrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdcfghr::R`](R) reader structure"]
impl crate::Readable for GpdcfghrSpec {}
#[doc = "`write(|w| ..)` method takes [`gpdcfghr::W`](W) writer structure"]
impl crate::Writable for GpdcfghrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPDCFGHR to value 0"]
impl crate::Resettable for GpdcfghrSpec {
    const RESET_VALUE: u32 = 0;
}
