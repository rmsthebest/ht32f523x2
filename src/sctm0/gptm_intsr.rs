#[doc = "Register `GPTM_INTSR` reader"]
pub struct R(crate::R<GPTM_INTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPTM_INTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPTM_INTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPTM_INTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPTM_INTSR` writer"]
pub struct W(crate::W<GPTM_INTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPTM_INTSR_SPEC>;
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
impl From<crate::W<GPTM_INTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPTM_INTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0CCIF` reader - CH0CCIF"]
pub type CH0CCIF_R = crate::BitReader<bool>;
#[doc = "Field `CH0CCIF` writer - CH0CCIF"]
pub type CH0CCIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_INTSR_SPEC, bool, O>;
#[doc = "Field `CH1CCIF` reader - CH1CCIF"]
pub type CH1CCIF_R = crate::BitReader<bool>;
#[doc = "Field `CH1CCIF` writer - CH1CCIF"]
pub type CH1CCIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_INTSR_SPEC, bool, O>;
#[doc = "Field `CH2CCIF` reader - CH2CCIF"]
pub type CH2CCIF_R = crate::BitReader<bool>;
#[doc = "Field `CH2CCIF` writer - CH2CCIF"]
pub type CH2CCIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_INTSR_SPEC, bool, O>;
#[doc = "Field `CH3CCIF` reader - CH3CCIF"]
pub type CH3CCIF_R = crate::BitReader<bool>;
#[doc = "Field `CH3CCIF` writer - CH3CCIF"]
pub type CH3CCIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_INTSR_SPEC, bool, O>;
#[doc = "Field `CH0OCF` reader - CH0OCF"]
pub type CH0OCF_R = crate::BitReader<bool>;
#[doc = "Field `CH0OCF` writer - CH0OCF"]
pub type CH0OCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_INTSR_SPEC, bool, O>;
#[doc = "Field `CH1OCF` reader - CH1OCF"]
pub type CH1OCF_R = crate::BitReader<bool>;
#[doc = "Field `CH1OCF` writer - CH1OCF"]
pub type CH1OCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_INTSR_SPEC, bool, O>;
#[doc = "Field `CH2OCF` reader - CH2OCF"]
pub type CH2OCF_R = crate::BitReader<bool>;
#[doc = "Field `CH2OCF` writer - CH2OCF"]
pub type CH2OCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_INTSR_SPEC, bool, O>;
#[doc = "Field `CH3OCF` reader - CH3OCF"]
pub type CH3OCF_R = crate::BitReader<bool>;
#[doc = "Field `CH3OCF` writer - CH3OCF"]
pub type CH3OCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_INTSR_SPEC, bool, O>;
#[doc = "Field `UEVIF` reader - UEVIF"]
pub type UEVIF_R = crate::BitReader<bool>;
#[doc = "Field `UEVIF` writer - UEVIF"]
pub type UEVIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_INTSR_SPEC, bool, O>;
#[doc = "Field `TEVIF` reader - TEVIF"]
pub type TEVIF_R = crate::BitReader<bool>;
#[doc = "Field `TEVIF` writer - TEVIF"]
pub type TEVIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTM_INTSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CH0CCIF"]
    #[inline(always)]
    pub fn ch0ccif(&self) -> CH0CCIF_R {
        CH0CCIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1CCIF"]
    #[inline(always)]
    pub fn ch1ccif(&self) -> CH1CCIF_R {
        CH1CCIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH2CCIF"]
    #[inline(always)]
    pub fn ch2ccif(&self) -> CH2CCIF_R {
        CH2CCIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH3CCIF"]
    #[inline(always)]
    pub fn ch3ccif(&self) -> CH3CCIF_R {
        CH3CCIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH0OCF"]
    #[inline(always)]
    pub fn ch0ocf(&self) -> CH0OCF_R {
        CH0OCF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH1OCF"]
    #[inline(always)]
    pub fn ch1ocf(&self) -> CH1OCF_R {
        CH1OCF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CH2OCF"]
    #[inline(always)]
    pub fn ch2ocf(&self) -> CH2OCF_R {
        CH2OCF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CH3OCF"]
    #[inline(always)]
    pub fn ch3ocf(&self) -> CH3OCF_R {
        CH3OCF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UEVIF"]
    #[inline(always)]
    pub fn uevif(&self) -> UEVIF_R {
        UEVIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - TEVIF"]
    #[inline(always)]
    pub fn tevif(&self) -> TEVIF_R {
        TEVIF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0CCIF"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ccif(&mut self) -> CH0CCIF_W<0> {
        CH0CCIF_W::new(self)
    }
    #[doc = "Bit 1 - CH1CCIF"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ccif(&mut self) -> CH1CCIF_W<1> {
        CH1CCIF_W::new(self)
    }
    #[doc = "Bit 2 - CH2CCIF"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ccif(&mut self) -> CH2CCIF_W<2> {
        CH2CCIF_W::new(self)
    }
    #[doc = "Bit 3 - CH3CCIF"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ccif(&mut self) -> CH3CCIF_W<3> {
        CH3CCIF_W::new(self)
    }
    #[doc = "Bit 4 - CH0OCF"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ocf(&mut self) -> CH0OCF_W<4> {
        CH0OCF_W::new(self)
    }
    #[doc = "Bit 5 - CH1OCF"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ocf(&mut self) -> CH1OCF_W<5> {
        CH1OCF_W::new(self)
    }
    #[doc = "Bit 6 - CH2OCF"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ocf(&mut self) -> CH2OCF_W<6> {
        CH2OCF_W::new(self)
    }
    #[doc = "Bit 7 - CH3OCF"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ocf(&mut self) -> CH3OCF_W<7> {
        CH3OCF_W::new(self)
    }
    #[doc = "Bit 8 - UEVIF"]
    #[inline(always)]
    #[must_use]
    pub fn uevif(&mut self) -> UEVIF_W<8> {
        UEVIF_W::new(self)
    }
    #[doc = "Bit 10 - TEVIF"]
    #[inline(always)]
    #[must_use]
    pub fn tevif(&mut self) -> TEVIF_W<10> {
        TEVIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM_INTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_intsr](index.html) module"]
pub struct GPTM_INTSR_SPEC;
impl crate::RegisterSpec for GPTM_INTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gptm_intsr::R](R) reader structure"]
impl crate::Readable for GPTM_INTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gptm_intsr::W](W) writer structure"]
impl crate::Writable for GPTM_INTSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPTM_INTSR to value 0"]
impl crate::Resettable for GPTM_INTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
