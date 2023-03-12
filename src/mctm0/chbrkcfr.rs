#[doc = "Register `CHBRKCFR` reader"]
pub struct R(crate::R<CHBRKCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHBRKCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHBRKCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHBRKCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHBRKCFR` writer"]
pub struct W(crate::W<CHBRKCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHBRKCFR_SPEC>;
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
impl From<crate::W<CHBRKCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHBRKCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0OIS` reader - CH0OIS"]
pub type CH0OIS_R = crate::BitReader<bool>;
#[doc = "Field `CH0OIS` writer - CH0OIS"]
pub type CH0OIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHBRKCFR_SPEC, bool, O>;
#[doc = "Field `CH0OISN` reader - CH0OISN"]
pub type CH0OISN_R = crate::BitReader<bool>;
#[doc = "Field `CH0OISN` writer - CH0OISN"]
pub type CH0OISN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHBRKCFR_SPEC, bool, O>;
#[doc = "Field `CH1OIS` reader - CH1OIS"]
pub type CH1OIS_R = crate::BitReader<bool>;
#[doc = "Field `CH1OIS` writer - CH1OIS"]
pub type CH1OIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHBRKCFR_SPEC, bool, O>;
#[doc = "Field `CH1OISN` reader - CH1OISN"]
pub type CH1OISN_R = crate::BitReader<bool>;
#[doc = "Field `CH1OISN` writer - CH1OISN"]
pub type CH1OISN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHBRKCFR_SPEC, bool, O>;
#[doc = "Field `CH2OIS` reader - CH2OIS"]
pub type CH2OIS_R = crate::BitReader<bool>;
#[doc = "Field `CH2OIS` writer - CH2OIS"]
pub type CH2OIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHBRKCFR_SPEC, bool, O>;
#[doc = "Field `CH2OISN` reader - CH2OISN"]
pub type CH2OISN_R = crate::BitReader<bool>;
#[doc = "Field `CH2OISN` writer - CH2OISN"]
pub type CH2OISN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHBRKCFR_SPEC, bool, O>;
#[doc = "Field `CH3OIS` reader - CH3OIS"]
pub type CH3OIS_R = crate::BitReader<bool>;
#[doc = "Field `CH3OIS` writer - CH3OIS"]
pub type CH3OIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHBRKCFR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CH0OIS"]
    #[inline(always)]
    pub fn ch0ois(&self) -> CH0OIS_R {
        CH0OIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH0OISN"]
    #[inline(always)]
    pub fn ch0oisn(&self) -> CH0OISN_R {
        CH0OISN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH1OIS"]
    #[inline(always)]
    pub fn ch1ois(&self) -> CH1OIS_R {
        CH1OIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH1OISN"]
    #[inline(always)]
    pub fn ch1oisn(&self) -> CH1OISN_R {
        CH1OISN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH2OIS"]
    #[inline(always)]
    pub fn ch2ois(&self) -> CH2OIS_R {
        CH2OIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH2OISN"]
    #[inline(always)]
    pub fn ch2oisn(&self) -> CH2OISN_R {
        CH2OISN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CH3OIS"]
    #[inline(always)]
    pub fn ch3ois(&self) -> CH3OIS_R {
        CH3OIS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0OIS"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ois(&mut self) -> CH0OIS_W<0> {
        CH0OIS_W::new(self)
    }
    #[doc = "Bit 1 - CH0OISN"]
    #[inline(always)]
    #[must_use]
    pub fn ch0oisn(&mut self) -> CH0OISN_W<1> {
        CH0OISN_W::new(self)
    }
    #[doc = "Bit 2 - CH1OIS"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ois(&mut self) -> CH1OIS_W<2> {
        CH1OIS_W::new(self)
    }
    #[doc = "Bit 3 - CH1OISN"]
    #[inline(always)]
    #[must_use]
    pub fn ch1oisn(&mut self) -> CH1OISN_W<3> {
        CH1OISN_W::new(self)
    }
    #[doc = "Bit 4 - CH2OIS"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ois(&mut self) -> CH2OIS_W<4> {
        CH2OIS_W::new(self)
    }
    #[doc = "Bit 5 - CH2OISN"]
    #[inline(always)]
    #[must_use]
    pub fn ch2oisn(&mut self) -> CH2OISN_W<5> {
        CH2OISN_W::new(self)
    }
    #[doc = "Bit 6 - CH3OIS"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ois(&mut self) -> CH3OIS_W<6> {
        CH3OIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CHBRKCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chbrkcfr](index.html) module"]
pub struct CHBRKCFR_SPEC;
impl crate::RegisterSpec for CHBRKCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chbrkcfr::R](R) reader structure"]
impl crate::Readable for CHBRKCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chbrkcfr::W](W) writer structure"]
impl crate::Writable for CHBRKCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHBRKCFR to value 0"]
impl crate::Resettable for CHBRKCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
