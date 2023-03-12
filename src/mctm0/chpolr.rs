#[doc = "Register `CHPOLR` reader"]
pub struct R(crate::R<CHPOLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHPOLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHPOLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHPOLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHPOLR` writer"]
pub struct W(crate::W<CHPOLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHPOLR_SPEC>;
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
impl From<crate::W<CHPOLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHPOLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0P` reader - CH0P"]
pub type CH0P_R = crate::BitReader<bool>;
#[doc = "Field `CH0P` writer - CH0P"]
pub type CH0P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPOLR_SPEC, bool, O>;
#[doc = "Field `CH0NP` reader - CH0NP"]
pub type CH0NP_R = crate::BitReader<bool>;
#[doc = "Field `CH0NP` writer - CH0NP"]
pub type CH0NP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPOLR_SPEC, bool, O>;
#[doc = "Field `CH1P` reader - CH1P"]
pub type CH1P_R = crate::BitReader<bool>;
#[doc = "Field `CH1P` writer - CH1P"]
pub type CH1P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPOLR_SPEC, bool, O>;
#[doc = "Field `CH1NP` reader - CH1NP"]
pub type CH1NP_R = crate::BitReader<bool>;
#[doc = "Field `CH1NP` writer - CH1NP"]
pub type CH1NP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPOLR_SPEC, bool, O>;
#[doc = "Field `CH2P` reader - CH2P"]
pub type CH2P_R = crate::BitReader<bool>;
#[doc = "Field `CH2P` writer - CH2P"]
pub type CH2P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPOLR_SPEC, bool, O>;
#[doc = "Field `CH2NP` reader - CH2NP"]
pub type CH2NP_R = crate::BitReader<bool>;
#[doc = "Field `CH2NP` writer - CH2NP"]
pub type CH2NP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPOLR_SPEC, bool, O>;
#[doc = "Field `CH3P` reader - CH3P"]
pub type CH3P_R = crate::BitReader<bool>;
#[doc = "Field `CH3P` writer - CH3P"]
pub type CH3P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPOLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CH0P"]
    #[inline(always)]
    pub fn ch0p(&self) -> CH0P_R {
        CH0P_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH0NP"]
    #[inline(always)]
    pub fn ch0np(&self) -> CH0NP_R {
        CH0NP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH1P"]
    #[inline(always)]
    pub fn ch1p(&self) -> CH1P_R {
        CH1P_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH1NP"]
    #[inline(always)]
    pub fn ch1np(&self) -> CH1NP_R {
        CH1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH2P"]
    #[inline(always)]
    pub fn ch2p(&self) -> CH2P_R {
        CH2P_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH2NP"]
    #[inline(always)]
    pub fn ch2np(&self) -> CH2NP_R {
        CH2NP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CH3P"]
    #[inline(always)]
    pub fn ch3p(&self) -> CH3P_R {
        CH3P_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0P"]
    #[inline(always)]
    #[must_use]
    pub fn ch0p(&mut self) -> CH0P_W<0> {
        CH0P_W::new(self)
    }
    #[doc = "Bit 1 - CH0NP"]
    #[inline(always)]
    #[must_use]
    pub fn ch0np(&mut self) -> CH0NP_W<1> {
        CH0NP_W::new(self)
    }
    #[doc = "Bit 2 - CH1P"]
    #[inline(always)]
    #[must_use]
    pub fn ch1p(&mut self) -> CH1P_W<2> {
        CH1P_W::new(self)
    }
    #[doc = "Bit 3 - CH1NP"]
    #[inline(always)]
    #[must_use]
    pub fn ch1np(&mut self) -> CH1NP_W<3> {
        CH1NP_W::new(self)
    }
    #[doc = "Bit 4 - CH2P"]
    #[inline(always)]
    #[must_use]
    pub fn ch2p(&mut self) -> CH2P_W<4> {
        CH2P_W::new(self)
    }
    #[doc = "Bit 5 - CH2NP"]
    #[inline(always)]
    #[must_use]
    pub fn ch2np(&mut self) -> CH2NP_W<5> {
        CH2NP_W::new(self)
    }
    #[doc = "Bit 6 - CH3P"]
    #[inline(always)]
    #[must_use]
    pub fn ch3p(&mut self) -> CH3P_W<6> {
        CH3P_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CHPOLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chpolr](index.html) module"]
pub struct CHPOLR_SPEC;
impl crate::RegisterSpec for CHPOLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chpolr::R](R) reader structure"]
impl crate::Readable for CHPOLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chpolr::W](W) writer structure"]
impl crate::Writable for CHPOLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHPOLR to value 0"]
impl crate::Resettable for CHPOLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
