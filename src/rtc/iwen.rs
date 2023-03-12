#[doc = "Register `IWEN` reader"]
pub struct R(crate::R<IWEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IWEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IWEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IWEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IWEN` writer"]
pub struct W(crate::W<IWEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IWEN_SPEC>;
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
impl From<crate::W<IWEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IWEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSECIEN` reader - CSECIEN"]
pub type CSECIEN_R = crate::BitReader<bool>;
#[doc = "Field `CSECIEN` writer - CSECIEN"]
pub type CSECIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IWEN_SPEC, bool, O>;
#[doc = "Field `CMIEN` reader - CMIEN"]
pub type CMIEN_R = crate::BitReader<bool>;
#[doc = "Field `CMIEN` writer - CMIEN"]
pub type CMIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IWEN_SPEC, bool, O>;
#[doc = "Field `OVIEN` reader - OVIEN"]
pub type OVIEN_R = crate::BitReader<bool>;
#[doc = "Field `OVIEN` writer - OVIEN"]
pub type OVIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IWEN_SPEC, bool, O>;
#[doc = "Field `CSECWEN` reader - CSECWEN"]
pub type CSECWEN_R = crate::BitReader<bool>;
#[doc = "Field `CSECWEN` writer - CSECWEN"]
pub type CSECWEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IWEN_SPEC, bool, O>;
#[doc = "Field `CMWEN` reader - CMWEN"]
pub type CMWEN_R = crate::BitReader<bool>;
#[doc = "Field `CMWEN` writer - CMWEN"]
pub type CMWEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IWEN_SPEC, bool, O>;
#[doc = "Field `OVWEN` reader - OVWEN"]
pub type OVWEN_R = crate::BitReader<bool>;
#[doc = "Field `OVWEN` writer - OVWEN"]
pub type OVWEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IWEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CSECIEN"]
    #[inline(always)]
    pub fn csecien(&self) -> CSECIEN_R {
        CSECIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMIEN"]
    #[inline(always)]
    pub fn cmien(&self) -> CMIEN_R {
        CMIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OVIEN"]
    #[inline(always)]
    pub fn ovien(&self) -> OVIEN_R {
        OVIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - CSECWEN"]
    #[inline(always)]
    pub fn csecwen(&self) -> CSECWEN_R {
        CSECWEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CMWEN"]
    #[inline(always)]
    pub fn cmwen(&self) -> CMWEN_R {
        CMWEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OVWEN"]
    #[inline(always)]
    pub fn ovwen(&self) -> OVWEN_R {
        OVWEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CSECIEN"]
    #[inline(always)]
    #[must_use]
    pub fn csecien(&mut self) -> CSECIEN_W<0> {
        CSECIEN_W::new(self)
    }
    #[doc = "Bit 1 - CMIEN"]
    #[inline(always)]
    #[must_use]
    pub fn cmien(&mut self) -> CMIEN_W<1> {
        CMIEN_W::new(self)
    }
    #[doc = "Bit 2 - OVIEN"]
    #[inline(always)]
    #[must_use]
    pub fn ovien(&mut self) -> OVIEN_W<2> {
        OVIEN_W::new(self)
    }
    #[doc = "Bit 8 - CSECWEN"]
    #[inline(always)]
    #[must_use]
    pub fn csecwen(&mut self) -> CSECWEN_W<8> {
        CSECWEN_W::new(self)
    }
    #[doc = "Bit 9 - CMWEN"]
    #[inline(always)]
    #[must_use]
    pub fn cmwen(&mut self) -> CMWEN_W<9> {
        CMWEN_W::new(self)
    }
    #[doc = "Bit 10 - OVWEN"]
    #[inline(always)]
    #[must_use]
    pub fn ovwen(&mut self) -> OVWEN_W<10> {
        OVWEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IWEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iwen](index.html) module"]
pub struct IWEN_SPEC;
impl crate::RegisterSpec for IWEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iwen::R](R) reader structure"]
impl crate::Readable for IWEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iwen::W](W) writer structure"]
impl crate::Writable for IWEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IWEN to value 0"]
impl crate::Resettable for IWEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
