#[doc = "Register `TSR` reader"]
pub struct R(crate::R<TSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSR` writer"]
pub struct W(crate::W<TSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSR_SPEC>;
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
impl From<crate::W<TSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADSC` reader - ADSC"]
pub type ADSC_R = crate::BitReader<bool>;
#[doc = "Field `ADSC` writer - ADSC"]
pub type ADSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
#[doc = "Field `ADEXTIS` reader - ADEXTIS"]
pub type ADEXTIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADEXTIS` writer - ADEXTIS"]
pub type ADEXTIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSR_SPEC, u8, u8, 4, O>;
#[doc = "Field `TMS` reader - TMS"]
pub type TMS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMS` writer - TMS"]
pub type TMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `BFTMS` reader - BFTMS"]
pub type BFTMS_R = crate::BitReader<bool>;
#[doc = "Field `BFTMS` writer - BFTMS"]
pub type BFTMS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
#[doc = "Field `CMPS` reader - CMPS"]
pub type CMPS_R = crate::BitReader<bool>;
#[doc = "Field `CMPS` writer - CMPS"]
pub type CMPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
#[doc = "Field `TME` reader - TME"]
pub type TME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TME` writer - TME"]
pub type TME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - ADSC"]
    #[inline(always)]
    pub fn adsc(&self) -> ADSC_R {
        ADSC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - ADEXTIS"]
    #[inline(always)]
    pub fn adextis(&self) -> ADEXTIS_R {
        ADEXTIS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - TMS"]
    #[inline(always)]
    pub fn tms(&self) -> TMS_R {
        TMS_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - BFTMS"]
    #[inline(always)]
    pub fn bftms(&self) -> BFTMS_R {
        BFTMS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CMPS"]
    #[inline(always)]
    pub fn cmps(&self) -> CMPS_R {
        CMPS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:26 - TME"]
    #[inline(always)]
    pub fn tme(&self) -> TME_R {
        TME_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADSC"]
    #[inline(always)]
    #[must_use]
    pub fn adsc(&mut self) -> ADSC_W<0> {
        ADSC_W::new(self)
    }
    #[doc = "Bits 8:11 - ADEXTIS"]
    #[inline(always)]
    #[must_use]
    pub fn adextis(&mut self) -> ADEXTIS_W<8> {
        ADEXTIS_W::new(self)
    }
    #[doc = "Bits 16:18 - TMS"]
    #[inline(always)]
    #[must_use]
    pub fn tms(&mut self) -> TMS_W<16> {
        TMS_W::new(self)
    }
    #[doc = "Bit 19 - BFTMS"]
    #[inline(always)]
    #[must_use]
    pub fn bftms(&mut self) -> BFTMS_W<19> {
        BFTMS_W::new(self)
    }
    #[doc = "Bit 20 - CMPS"]
    #[inline(always)]
    #[must_use]
    pub fn cmps(&mut self) -> CMPS_W<20> {
        CMPS_W::new(self)
    }
    #[doc = "Bits 24:26 - TME"]
    #[inline(always)]
    #[must_use]
    pub fn tme(&mut self) -> TME_W<24> {
        TME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsr](index.html) module"]
pub struct TSR_SPEC;
impl crate::RegisterSpec for TSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsr::R](R) reader structure"]
impl crate::Readable for TSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsr::W](W) writer structure"]
impl crate::Writable for TSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSR to value 0"]
impl crate::Resettable for TSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
