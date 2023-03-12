#[doc = "Register `MR0` reader"]
pub struct R(crate::R<MR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR0` writer"]
pub struct W(crate::W<MR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR0_SPEC>;
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
impl From<crate::W<MR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTV` reader - WDTV"]
pub type WDTV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WDTV` writer - WDTV"]
pub type WDTV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR0_SPEC, u16, u16, 12, O>;
#[doc = "Field `WDTFIEN` reader - WDTFIEN"]
pub type WDTFIEN_R = crate::BitReader<bool>;
#[doc = "Field `WDTFIEN` writer - WDTFIEN"]
pub type WDTFIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR0_SPEC, bool, O>;
#[doc = "Field `WDTRSTEN` reader - WDTRSTEN"]
pub type WDTRSTEN_R = crate::BitReader<bool>;
#[doc = "Field `WDTRSTEN` writer - WDTRSTEN"]
pub type WDTRSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR0_SPEC, bool, O>;
#[doc = "Field `WDTSHLT` reader - WDTSHLT"]
pub type WDTSHLT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDTSHLT` writer - WDTSHLT"]
pub type WDTSHLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `WDTEN` reader - WDTEN"]
pub type WDTEN_R = crate::BitReader<bool>;
#[doc = "Field `WDTEN` writer - WDTEN"]
pub type WDTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11 - WDTV"]
    #[inline(always)]
    pub fn wdtv(&self) -> WDTV_R {
        WDTV_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - WDTFIEN"]
    #[inline(always)]
    pub fn wdtfien(&self) -> WDTFIEN_R {
        WDTFIEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - WDTRSTEN"]
    #[inline(always)]
    pub fn wdtrsten(&self) -> WDTRSTEN_R {
        WDTRSTEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - WDTSHLT"]
    #[inline(always)]
    pub fn wdtshlt(&self) -> WDTSHLT_R {
        WDTSHLT_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - WDTEN"]
    #[inline(always)]
    pub fn wdten(&self) -> WDTEN_R {
        WDTEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - WDTV"]
    #[inline(always)]
    #[must_use]
    pub fn wdtv(&mut self) -> WDTV_W<0> {
        WDTV_W::new(self)
    }
    #[doc = "Bit 12 - WDTFIEN"]
    #[inline(always)]
    #[must_use]
    pub fn wdtfien(&mut self) -> WDTFIEN_W<12> {
        WDTFIEN_W::new(self)
    }
    #[doc = "Bit 13 - WDTRSTEN"]
    #[inline(always)]
    #[must_use]
    pub fn wdtrsten(&mut self) -> WDTRSTEN_W<13> {
        WDTRSTEN_W::new(self)
    }
    #[doc = "Bits 14:15 - WDTSHLT"]
    #[inline(always)]
    #[must_use]
    pub fn wdtshlt(&mut self) -> WDTSHLT_W<14> {
        WDTSHLT_W::new(self)
    }
    #[doc = "Bit 16 - WDTEN"]
    #[inline(always)]
    #[must_use]
    pub fn wdten(&mut self) -> WDTEN_W<16> {
        WDTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr0](index.html) module"]
pub struct MR0_SPEC;
impl crate::RegisterSpec for MR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr0::R](R) reader structure"]
impl crate::Readable for MR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr0::W](W) writer structure"]
impl crate::Writable for MR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR0 to value 0"]
impl crate::Resettable for MR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
