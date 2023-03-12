#[doc = "Register `GPTM_DICTR` reader"]
pub struct R(crate::R<GPTM_DICTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPTM_DICTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPTM_DICTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPTM_DICTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPTM_DICTR` writer"]
pub struct W(crate::W<GPTM_DICTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPTM_DICTR_SPEC>;
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
impl From<crate::W<GPTM_DICTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPTM_DICTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0CCIE` reader - CH0CCIE"]
pub type CH0CCIE_R = crate::BitReader<bool>;
#[doc = "Field `CH0CCIE` writer - CH0CCIE"]
pub type CH0CCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_DICTR_SPEC, bool, O>;
#[doc = "Field `CH1CCIE` reader - CH1CCIE"]
pub type CH1CCIE_R = crate::BitReader<bool>;
#[doc = "Field `CH1CCIE` writer - CH1CCIE"]
pub type CH1CCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_DICTR_SPEC, bool, O>;
#[doc = "Field `CH2CCIE` reader - CH2CCIE"]
pub type CH2CCIE_R = crate::BitReader<bool>;
#[doc = "Field `CH2CCIE` writer - CH2CCIE"]
pub type CH2CCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_DICTR_SPEC, bool, O>;
#[doc = "Field `CH3CCIE` reader - CH3CCIE"]
pub type CH3CCIE_R = crate::BitReader<bool>;
#[doc = "Field `CH3CCIE` writer - CH3CCIE"]
pub type CH3CCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_DICTR_SPEC, bool, O>;
#[doc = "Field `UEVIE` reader - UEVIE"]
pub type UEVIE_R = crate::BitReader<bool>;
#[doc = "Field `UEVIE` writer - UEVIE"]
pub type UEVIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_DICTR_SPEC, bool, O>;
#[doc = "Field `TEVIE` reader - TEVIE"]
pub type TEVIE_R = crate::BitReader<bool>;
#[doc = "Field `TEVIE` writer - TEVIE"]
pub type TEVIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_DICTR_SPEC, bool, O>;
#[doc = "Field `CH0CCDE` reader - CH0CCDE"]
pub type CH0CCDE_R = crate::BitReader<bool>;
#[doc = "Field `CH0CCDE` writer - CH0CCDE"]
pub type CH0CCDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_DICTR_SPEC, bool, O>;
#[doc = "Field `CH1CCDE` reader - CH1CCDE"]
pub type CH1CCDE_R = crate::BitReader<bool>;
#[doc = "Field `CH1CCDE` writer - CH1CCDE"]
pub type CH1CCDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_DICTR_SPEC, bool, O>;
#[doc = "Field `CH2CCDE` reader - CH2CCDE"]
pub type CH2CCDE_R = crate::BitReader<bool>;
#[doc = "Field `CH2CCDE` writer - CH2CCDE"]
pub type CH2CCDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_DICTR_SPEC, bool, O>;
#[doc = "Field `CH3CCDE` reader - CH3CCDE"]
pub type CH3CCDE_R = crate::BitReader<bool>;
#[doc = "Field `CH3CCDE` writer - CH3CCDE"]
pub type CH3CCDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_DICTR_SPEC, bool, O>;
#[doc = "Field `UEVDE` reader - UEVDE"]
pub type UEVDE_R = crate::BitReader<bool>;
#[doc = "Field `UEVDE` writer - UEVDE"]
pub type UEVDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_DICTR_SPEC, bool, O>;
#[doc = "Field `TEVDE` reader - TEVDE"]
pub type TEVDE_R = crate::BitReader<bool>;
#[doc = "Field `TEVDE` writer - TEVDE"]
pub type TEVDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_DICTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CH0CCIE"]
    #[inline(always)]
    pub fn ch0ccie(&self) -> CH0CCIE_R {
        CH0CCIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1CCIE"]
    #[inline(always)]
    pub fn ch1ccie(&self) -> CH1CCIE_R {
        CH1CCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH2CCIE"]
    #[inline(always)]
    pub fn ch2ccie(&self) -> CH2CCIE_R {
        CH2CCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH3CCIE"]
    #[inline(always)]
    pub fn ch3ccie(&self) -> CH3CCIE_R {
        CH3CCIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - UEVIE"]
    #[inline(always)]
    pub fn uevie(&self) -> UEVIE_R {
        UEVIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - TEVIE"]
    #[inline(always)]
    pub fn tevie(&self) -> TEVIE_R {
        TEVIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - CH0CCDE"]
    #[inline(always)]
    pub fn ch0ccde(&self) -> CH0CCDE_R {
        CH0CCDE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CH1CCDE"]
    #[inline(always)]
    pub fn ch1ccde(&self) -> CH1CCDE_R {
        CH1CCDE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CH2CCDE"]
    #[inline(always)]
    pub fn ch2ccde(&self) -> CH2CCDE_R {
        CH2CCDE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CH3CCDE"]
    #[inline(always)]
    pub fn ch3ccde(&self) -> CH3CCDE_R {
        CH3CCDE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - UEVDE"]
    #[inline(always)]
    pub fn uevde(&self) -> UEVDE_R {
        UEVDE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - TEVDE"]
    #[inline(always)]
    pub fn tevde(&self) -> TEVDE_R {
        TEVDE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0CCIE"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ccie(&mut self) -> CH0CCIE_W<0> {
        CH0CCIE_W::new(self)
    }
    #[doc = "Bit 1 - CH1CCIE"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ccie(&mut self) -> CH1CCIE_W<1> {
        CH1CCIE_W::new(self)
    }
    #[doc = "Bit 2 - CH2CCIE"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ccie(&mut self) -> CH2CCIE_W<2> {
        CH2CCIE_W::new(self)
    }
    #[doc = "Bit 3 - CH3CCIE"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ccie(&mut self) -> CH3CCIE_W<3> {
        CH3CCIE_W::new(self)
    }
    #[doc = "Bit 8 - UEVIE"]
    #[inline(always)]
    #[must_use]
    pub fn uevie(&mut self) -> UEVIE_W<8> {
        UEVIE_W::new(self)
    }
    #[doc = "Bit 10 - TEVIE"]
    #[inline(always)]
    #[must_use]
    pub fn tevie(&mut self) -> TEVIE_W<10> {
        TEVIE_W::new(self)
    }
    #[doc = "Bit 16 - CH0CCDE"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ccde(&mut self) -> CH0CCDE_W<16> {
        CH0CCDE_W::new(self)
    }
    #[doc = "Bit 17 - CH1CCDE"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ccde(&mut self) -> CH1CCDE_W<17> {
        CH1CCDE_W::new(self)
    }
    #[doc = "Bit 18 - CH2CCDE"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ccde(&mut self) -> CH2CCDE_W<18> {
        CH2CCDE_W::new(self)
    }
    #[doc = "Bit 19 - CH3CCDE"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ccde(&mut self) -> CH3CCDE_W<19> {
        CH3CCDE_W::new(self)
    }
    #[doc = "Bit 24 - UEVDE"]
    #[inline(always)]
    #[must_use]
    pub fn uevde(&mut self) -> UEVDE_W<24> {
        UEVDE_W::new(self)
    }
    #[doc = "Bit 26 - TEVDE"]
    #[inline(always)]
    #[must_use]
    pub fn tevde(&mut self) -> TEVDE_W<26> {
        TEVDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM_DICTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_dictr](index.html) module"]
pub struct GPTM_DICTR_SPEC;
impl crate::RegisterSpec for GPTM_DICTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gptm_dictr::R](R) reader structure"]
impl crate::Readable for GPTM_DICTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gptm_dictr::W](W) writer structure"]
impl crate::Writable for GPTM_DICTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPTM_DICTR to value 0"]
impl crate::Resettable for GPTM_DICTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
