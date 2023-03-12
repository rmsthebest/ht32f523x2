#[doc = "Register `GPACFGHR` reader"]
pub struct R(crate::R<GPACFGHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPACFGHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPACFGHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPACFGHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPACFGHR` writer"]
pub struct W(crate::W<GPACFGHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPACFGHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GPACFGHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPACFGHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFG8` reader - CFG8"]
pub type CFG8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG8` writer - CFG8"]
pub type CFG8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPACFGHR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CFG9` reader - CFG9"]
pub type CFG9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG9` writer - CFG9"]
pub type CFG9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPACFGHR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CFG10` reader - CFG10"]
pub type CFG10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG10` writer - CFG10"]
pub type CFG10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPACFGHR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CFG11` reader - CFG11"]
pub type CFG11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG11` writer - CFG11"]
pub type CFG11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPACFGHR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CFG12` reader - CFG12"]
pub type CFG12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG12` writer - CFG12"]
pub type CFG12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPACFGHR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CFG13` reader - CFG13"]
pub type CFG13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG13` writer - CFG13"]
pub type CFG13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPACFGHR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CFG14` reader - CFG14"]
pub type CFG14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG14` writer - CFG14"]
pub type CFG14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPACFGHR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CFG15` reader - CFG15"]
pub type CFG15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG15` writer - CFG15"]
pub type CFG15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPACFGHR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - CFG8"]
    #[inline(always)]
    pub fn cfg8(&self) -> CFG8_R {
        CFG8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CFG9"]
    #[inline(always)]
    pub fn cfg9(&self) -> CFG9_R {
        CFG9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - CFG10"]
    #[inline(always)]
    pub fn cfg10(&self) -> CFG10_R {
        CFG10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - CFG11"]
    #[inline(always)]
    pub fn cfg11(&self) -> CFG11_R {
        CFG11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - CFG12"]
    #[inline(always)]
    pub fn cfg12(&self) -> CFG12_R {
        CFG12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - CFG13"]
    #[inline(always)]
    pub fn cfg13(&self) -> CFG13_R {
        CFG13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - CFG14"]
    #[inline(always)]
    pub fn cfg14(&self) -> CFG14_R {
        CFG14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - CFG15"]
    #[inline(always)]
    pub fn cfg15(&self) -> CFG15_R {
        CFG15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CFG8"]
    #[inline(always)]
    #[must_use]
    pub fn cfg8(&mut self) -> CFG8_W<0> {
        CFG8_W::new(self)
    }
    #[doc = "Bits 4:7 - CFG9"]
    #[inline(always)]
    #[must_use]
    pub fn cfg9(&mut self) -> CFG9_W<4> {
        CFG9_W::new(self)
    }
    #[doc = "Bits 8:11 - CFG10"]
    #[inline(always)]
    #[must_use]
    pub fn cfg10(&mut self) -> CFG10_W<8> {
        CFG10_W::new(self)
    }
    #[doc = "Bits 12:15 - CFG11"]
    #[inline(always)]
    #[must_use]
    pub fn cfg11(&mut self) -> CFG11_W<12> {
        CFG11_W::new(self)
    }
    #[doc = "Bits 16:19 - CFG12"]
    #[inline(always)]
    #[must_use]
    pub fn cfg12(&mut self) -> CFG12_W<16> {
        CFG12_W::new(self)
    }
    #[doc = "Bits 20:23 - CFG13"]
    #[inline(always)]
    #[must_use]
    pub fn cfg13(&mut self) -> CFG13_W<20> {
        CFG13_W::new(self)
    }
    #[doc = "Bits 24:27 - CFG14"]
    #[inline(always)]
    #[must_use]
    pub fn cfg14(&mut self) -> CFG14_W<24> {
        CFG14_W::new(self)
    }
    #[doc = "Bits 28:31 - CFG15"]
    #[inline(always)]
    #[must_use]
    pub fn cfg15(&mut self) -> CFG15_W<28> {
        CFG15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPACFGHR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpacfghr](index.html) module"]
pub struct GPACFGHR_SPEC;
impl crate::RegisterSpec for GPACFGHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpacfghr::R](R) reader structure"]
impl crate::Readable for GPACFGHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpacfghr::W](W) writer structure"]
impl crate::Writable for GPACFGHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPACFGHR to value 0"]
impl crate::Resettable for GPACFGHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
