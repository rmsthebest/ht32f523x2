#[doc = "Register `DICTR` reader"]
pub struct R(crate::R<DICTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DICTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DICTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DICTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DICTR` writer"]
pub struct W(crate::W<DICTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DICTR_SPEC>;
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
impl From<crate::W<DICTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DICTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0CCIE` reader - CH0CCIE"]
pub type CH0CCIE_R = crate::BitReader<bool>;
#[doc = "Field `CH0CCIE` writer - CH0CCIE"]
pub type CH0CCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DICTR_SPEC, bool, O>;
#[doc = "Field `CH1CCIE` reader - CH1CCIE"]
pub type CH1CCIE_R = crate::BitReader<bool>;
#[doc = "Field `CH1CCIE` writer - CH1CCIE"]
pub type CH1CCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DICTR_SPEC, bool, O>;
#[doc = "Field `CH2CCIE` reader - CH2CCIE"]
pub type CH2CCIE_R = crate::BitReader<bool>;
#[doc = "Field `CH2CCIE` writer - CH2CCIE"]
pub type CH2CCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DICTR_SPEC, bool, O>;
#[doc = "Field `CH3CCIE` reader - CH3CCIE"]
pub type CH3CCIE_R = crate::BitReader<bool>;
#[doc = "Field `CH3CCIE` writer - CH3CCIE"]
pub type CH3CCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DICTR_SPEC, bool, O>;
#[doc = "Field `UEV1IE` reader - UEV1IE"]
pub type UEV1IE_R = crate::BitReader<bool>;
#[doc = "Field `UEV1IE` writer - UEV1IE"]
pub type UEV1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DICTR_SPEC, bool, O>;
#[doc = "Field `UEV2IE` reader - UEV2IE"]
pub type UEV2IE_R = crate::BitReader<bool>;
#[doc = "Field `UEV2IE` writer - UEV2IE"]
pub type UEV2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DICTR_SPEC, bool, O>;
#[doc = "Field `TEVIE` reader - TEVIE"]
pub type TEVIE_R = crate::BitReader<bool>;
#[doc = "Field `TEVIE` writer - TEVIE"]
pub type TEVIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DICTR_SPEC, bool, O>;
#[doc = "Field `BRKIE` reader - BRKIE"]
pub type BRKIE_R = crate::BitReader<bool>;
#[doc = "Field `BRKIE` writer - BRKIE"]
pub type BRKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DICTR_SPEC, bool, O>;
#[doc = "Field `CH0CCDE` reader - CH0CCDE"]
pub type CH0CCDE_R = crate::BitReader<bool>;
#[doc = "Field `CH0CCDE` writer - CH0CCDE"]
pub type CH0CCDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DICTR_SPEC, bool, O>;
#[doc = "Field `CH1CCDE` reader - CH1CCDE"]
pub type CH1CCDE_R = crate::BitReader<bool>;
#[doc = "Field `CH1CCDE` writer - CH1CCDE"]
pub type CH1CCDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DICTR_SPEC, bool, O>;
#[doc = "Field `CH2CCDE` reader - CH2CCDE"]
pub type CH2CCDE_R = crate::BitReader<bool>;
#[doc = "Field `CH2CCDE` writer - CH2CCDE"]
pub type CH2CCDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DICTR_SPEC, bool, O>;
#[doc = "Field `CH3CCDE` reader - CH3CCDE"]
pub type CH3CCDE_R = crate::BitReader<bool>;
#[doc = "Field `CH3CCDE` writer - CH3CCDE"]
pub type CH3CCDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DICTR_SPEC, bool, O>;
#[doc = "Field `UEV1DE` reader - UEV1DE"]
pub type UEV1DE_R = crate::BitReader<bool>;
#[doc = "Field `UEV1DE` writer - UEV1DE"]
pub type UEV1DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DICTR_SPEC, bool, O>;
#[doc = "Field `UEV2DE` reader - UEV2DE"]
pub type UEV2DE_R = crate::BitReader<bool>;
#[doc = "Field `UEV2DE` writer - UEV2DE"]
pub type UEV2DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DICTR_SPEC, bool, O>;
#[doc = "Field `TEVDE` reader - TEVDE"]
pub type TEVDE_R = crate::BitReader<bool>;
#[doc = "Field `TEVDE` writer - TEVDE"]
pub type TEVDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DICTR_SPEC, bool, O>;
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
    #[doc = "Bit 8 - UEV1IE"]
    #[inline(always)]
    pub fn uev1ie(&self) -> UEV1IE_R {
        UEV1IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UEV2IE"]
    #[inline(always)]
    pub fn uev2ie(&self) -> UEV2IE_R {
        UEV2IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TEVIE"]
    #[inline(always)]
    pub fn tevie(&self) -> TEVIE_R {
        TEVIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BRKIE"]
    #[inline(always)]
    pub fn brkie(&self) -> BRKIE_R {
        BRKIE_R::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 24 - UEV1DE"]
    #[inline(always)]
    pub fn uev1de(&self) -> UEV1DE_R {
        UEV1DE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - UEV2DE"]
    #[inline(always)]
    pub fn uev2de(&self) -> UEV2DE_R {
        UEV2DE_R::new(((self.bits >> 25) & 1) != 0)
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
    #[doc = "Bit 8 - UEV1IE"]
    #[inline(always)]
    #[must_use]
    pub fn uev1ie(&mut self) -> UEV1IE_W<8> {
        UEV1IE_W::new(self)
    }
    #[doc = "Bit 9 - UEV2IE"]
    #[inline(always)]
    #[must_use]
    pub fn uev2ie(&mut self) -> UEV2IE_W<9> {
        UEV2IE_W::new(self)
    }
    #[doc = "Bit 10 - TEVIE"]
    #[inline(always)]
    #[must_use]
    pub fn tevie(&mut self) -> TEVIE_W<10> {
        TEVIE_W::new(self)
    }
    #[doc = "Bit 11 - BRKIE"]
    #[inline(always)]
    #[must_use]
    pub fn brkie(&mut self) -> BRKIE_W<11> {
        BRKIE_W::new(self)
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
    #[doc = "Bit 24 - UEV1DE"]
    #[inline(always)]
    #[must_use]
    pub fn uev1de(&mut self) -> UEV1DE_W<24> {
        UEV1DE_W::new(self)
    }
    #[doc = "Bit 25 - UEV2DE"]
    #[inline(always)]
    #[must_use]
    pub fn uev2de(&mut self) -> UEV2DE_W<25> {
        UEV2DE_W::new(self)
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
#[doc = "DICTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dictr](index.html) module"]
pub struct DICTR_SPEC;
impl crate::RegisterSpec for DICTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dictr::R](R) reader structure"]
impl crate::Readable for DICTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dictr::W](W) writer structure"]
impl crate::Writable for DICTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DICTR to value 0"]
impl crate::Resettable for DICTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
