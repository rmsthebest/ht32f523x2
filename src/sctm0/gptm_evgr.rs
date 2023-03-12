#[doc = "Register `GPTM_EVGR` reader"]
pub struct R(crate::R<GPTM_EVGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPTM_EVGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPTM_EVGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPTM_EVGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPTM_EVGR` writer"]
pub struct W(crate::W<GPTM_EVGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPTM_EVGR_SPEC>;
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
impl From<crate::W<GPTM_EVGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPTM_EVGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0CCG` reader - CH0CCG"]
pub type CH0CCG_R = crate::BitReader<bool>;
#[doc = "Field `CH0CCG` writer - CH0CCG"]
pub type CH0CCG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_EVGR_SPEC, bool, O>;
#[doc = "Field `CH1CCG` reader - CH1CCG"]
pub type CH1CCG_R = crate::BitReader<bool>;
#[doc = "Field `CH1CCG` writer - CH1CCG"]
pub type CH1CCG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_EVGR_SPEC, bool, O>;
#[doc = "Field `CH2CCG` reader - CH2CCG"]
pub type CH2CCG_R = crate::BitReader<bool>;
#[doc = "Field `CH2CCG` writer - CH2CCG"]
pub type CH2CCG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_EVGR_SPEC, bool, O>;
#[doc = "Field `CH3CCG` reader - CH3CCG"]
pub type CH3CCG_R = crate::BitReader<bool>;
#[doc = "Field `CH3CCG` writer - CH3CCG"]
pub type CH3CCG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_EVGR_SPEC, bool, O>;
#[doc = "Field `UEVG` reader - UEVG"]
pub type UEVG_R = crate::BitReader<bool>;
#[doc = "Field `UEVG` writer - UEVG"]
pub type UEVG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_EVGR_SPEC, bool, O>;
#[doc = "Field `TEVG` reader - TEVG"]
pub type TEVG_R = crate::BitReader<bool>;
#[doc = "Field `TEVG` writer - TEVG"]
pub type TEVG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_EVGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CH0CCG"]
    #[inline(always)]
    pub fn ch0ccg(&self) -> CH0CCG_R {
        CH0CCG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1CCG"]
    #[inline(always)]
    pub fn ch1ccg(&self) -> CH1CCG_R {
        CH1CCG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH2CCG"]
    #[inline(always)]
    pub fn ch2ccg(&self) -> CH2CCG_R {
        CH2CCG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH3CCG"]
    #[inline(always)]
    pub fn ch3ccg(&self) -> CH3CCG_R {
        CH3CCG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - UEVG"]
    #[inline(always)]
    pub fn uevg(&self) -> UEVG_R {
        UEVG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - TEVG"]
    #[inline(always)]
    pub fn tevg(&self) -> TEVG_R {
        TEVG_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0CCG"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ccg(&mut self) -> CH0CCG_W<0> {
        CH0CCG_W::new(self)
    }
    #[doc = "Bit 1 - CH1CCG"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ccg(&mut self) -> CH1CCG_W<1> {
        CH1CCG_W::new(self)
    }
    #[doc = "Bit 2 - CH2CCG"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ccg(&mut self) -> CH2CCG_W<2> {
        CH2CCG_W::new(self)
    }
    #[doc = "Bit 3 - CH3CCG"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ccg(&mut self) -> CH3CCG_W<3> {
        CH3CCG_W::new(self)
    }
    #[doc = "Bit 8 - UEVG"]
    #[inline(always)]
    #[must_use]
    pub fn uevg(&mut self) -> UEVG_W<8> {
        UEVG_W::new(self)
    }
    #[doc = "Bit 10 - TEVG"]
    #[inline(always)]
    #[must_use]
    pub fn tevg(&mut self) -> TEVG_W<10> {
        TEVG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM_EVGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_evgr](index.html) module"]
pub struct GPTM_EVGR_SPEC;
impl crate::RegisterSpec for GPTM_EVGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gptm_evgr::R](R) reader structure"]
impl crate::Readable for GPTM_EVGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gptm_evgr::W](W) writer structure"]
impl crate::Writable for GPTM_EVGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPTM_EVGR to value 0"]
impl crate::Resettable for GPTM_EVGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
