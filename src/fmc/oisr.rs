#[doc = "Register `OISR` reader"]
pub struct R(crate::R<OISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OISR` writer"]
pub struct W(crate::W<OISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OISR_SPEC>;
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
impl From<crate::W<OISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ORFF` reader - ORFF"]
pub type ORFF_R = crate::BitReader<bool>;
#[doc = "Field `ORFF` writer - ORFF"]
pub type ORFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, OISR_SPEC, bool, O>;
#[doc = "Field `ITADF` reader - ITADF"]
pub type ITADF_R = crate::BitReader<bool>;
#[doc = "Field `ITADF` writer - ITADF"]
pub type ITADF_W<'a, const O: u8> = crate::BitWriter<'a, u32, OISR_SPEC, bool, O>;
#[doc = "Field `OBEF` reader - OBEF"]
pub type OBEF_R = crate::BitReader<bool>;
#[doc = "Field `OBEF` writer - OBEF"]
pub type OBEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, OISR_SPEC, bool, O>;
#[doc = "Field `IOCMF` reader - IOCMF"]
pub type IOCMF_R = crate::BitReader<bool>;
#[doc = "Field `IOCMF` writer - IOCMF"]
pub type IOCMF_W<'a, const O: u8> = crate::BitWriter<'a, u32, OISR_SPEC, bool, O>;
#[doc = "Field `OREF` reader - OREF"]
pub type OREF_R = crate::BitReader<bool>;
#[doc = "Field `OREF` writer - OREF"]
pub type OREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, OISR_SPEC, bool, O>;
#[doc = "Field `RORFF` reader - RORFF"]
pub type RORFF_R = crate::BitReader<bool>;
#[doc = "Field `RORFF` writer - RORFF"]
pub type RORFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, OISR_SPEC, bool, O>;
#[doc = "Field `PPEF` reader - PPEF"]
pub type PPEF_R = crate::BitReader<bool>;
#[doc = "Field `PPEF` writer - PPEF"]
pub type PPEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, OISR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ORFF"]
    #[inline(always)]
    pub fn orff(&self) -> ORFF_R {
        ORFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ITADF"]
    #[inline(always)]
    pub fn itadf(&self) -> ITADF_R {
        ITADF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OBEF"]
    #[inline(always)]
    pub fn obef(&self) -> OBEF_R {
        OBEF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IOCMF"]
    #[inline(always)]
    pub fn iocmf(&self) -> IOCMF_R {
        IOCMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OREF"]
    #[inline(always)]
    pub fn oref(&self) -> OREF_R {
        OREF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - RORFF"]
    #[inline(always)]
    pub fn rorff(&self) -> RORFF_R {
        RORFF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PPEF"]
    #[inline(always)]
    pub fn ppef(&self) -> PPEF_R {
        PPEF_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ORFF"]
    #[inline(always)]
    #[must_use]
    pub fn orff(&mut self) -> ORFF_W<0> {
        ORFF_W::new(self)
    }
    #[doc = "Bit 1 - ITADF"]
    #[inline(always)]
    #[must_use]
    pub fn itadf(&mut self) -> ITADF_W<1> {
        ITADF_W::new(self)
    }
    #[doc = "Bit 2 - OBEF"]
    #[inline(always)]
    #[must_use]
    pub fn obef(&mut self) -> OBEF_W<2> {
        OBEF_W::new(self)
    }
    #[doc = "Bit 3 - IOCMF"]
    #[inline(always)]
    #[must_use]
    pub fn iocmf(&mut self) -> IOCMF_W<3> {
        IOCMF_W::new(self)
    }
    #[doc = "Bit 4 - OREF"]
    #[inline(always)]
    #[must_use]
    pub fn oref(&mut self) -> OREF_W<4> {
        OREF_W::new(self)
    }
    #[doc = "Bit 16 - RORFF"]
    #[inline(always)]
    #[must_use]
    pub fn rorff(&mut self) -> RORFF_W<16> {
        RORFF_W::new(self)
    }
    #[doc = "Bit 17 - PPEF"]
    #[inline(always)]
    #[must_use]
    pub fn ppef(&mut self) -> PPEF_W<17> {
        PPEF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oisr](index.html) module"]
pub struct OISR_SPEC;
impl crate::RegisterSpec for OISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oisr::R](R) reader structure"]
impl crate::Readable for OISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oisr::W](W) writer structure"]
impl crate::Writable for OISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OISR to value 0"]
impl crate::Resettable for OISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
