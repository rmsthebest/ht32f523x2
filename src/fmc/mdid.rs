#[doc = "Register `MDID` reader"]
pub struct R(crate::R<MDID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDID` writer"]
pub struct W(crate::W<MDID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDID_SPEC>;
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
impl From<crate::W<MDID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHIPID` reader - CHIPID"]
pub type CHIPID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CHIPID` writer - CHIPID"]
pub type CHIPID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDID_SPEC, u16, u16, 16, O>;
#[doc = "Field `MFID` reader - MFID"]
pub type MFID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MFID` writer - MFID"]
pub type MFID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDID_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - CHIPID"]
    #[inline(always)]
    pub fn chipid(&self) -> CHIPID_R {
        CHIPID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - MFID"]
    #[inline(always)]
    pub fn mfid(&self) -> MFID_R {
        MFID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CHIPID"]
    #[inline(always)]
    #[must_use]
    pub fn chipid(&mut self) -> CHIPID_W<0> {
        CHIPID_W::new(self)
    }
    #[doc = "Bits 16:31 - MFID"]
    #[inline(always)]
    #[must_use]
    pub fn mfid(&mut self) -> MFID_W<16> {
        MFID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdid](index.html) module"]
pub struct MDID_SPEC;
impl crate::RegisterSpec for MDID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdid::R](R) reader structure"]
impl crate::Readable for MDID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdid::W](W) writer structure"]
impl crate::Writable for MDID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDID to value 0"]
impl crate::Resettable for MDID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
