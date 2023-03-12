#[doc = "Register `CFCR` reader"]
pub struct R(crate::R<CFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFCR` writer"]
pub struct W(crate::W<CFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFCR_SPEC>;
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
impl From<crate::W<CFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAIT` reader - WAIT"]
pub type WAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAIT` writer - WAIT"]
pub type WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `PFBE` reader - PFBE"]
pub type PFBE_R = crate::BitReader<bool>;
#[doc = "Field `PFBE` writer - PFBE"]
pub type PFBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFCR_SPEC, bool, O>;
#[doc = "Field `DCDB` reader - DCDB"]
pub type DCDB_R = crate::BitReader<bool>;
#[doc = "Field `DCDB` writer - DCDB"]
pub type DCDB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFCR_SPEC, bool, O>;
#[doc = "Field `CE` reader - CE"]
pub type CE_R = crate::BitReader<bool>;
#[doc = "Field `CE` writer - CE"]
pub type CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - WAIT"]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - PFBE"]
    #[inline(always)]
    pub fn pfbe(&self) -> PFBE_R {
        PFBE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - DCDB"]
    #[inline(always)]
    pub fn dcdb(&self) -> DCDB_R {
        DCDB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - CE"]
    #[inline(always)]
    pub fn ce(&self) -> CE_R {
        CE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - WAIT"]
    #[inline(always)]
    #[must_use]
    pub fn wait(&mut self) -> WAIT_W<0> {
        WAIT_W::new(self)
    }
    #[doc = "Bit 4 - PFBE"]
    #[inline(always)]
    #[must_use]
    pub fn pfbe(&mut self) -> PFBE_W<4> {
        PFBE_W::new(self)
    }
    #[doc = "Bit 7 - DCDB"]
    #[inline(always)]
    #[must_use]
    pub fn dcdb(&mut self) -> DCDB_W<7> {
        DCDB_W::new(self)
    }
    #[doc = "Bit 12 - CE"]
    #[inline(always)]
    #[must_use]
    pub fn ce(&mut self) -> CE_W<12> {
        CE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CFCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfcr](index.html) module"]
pub struct CFCR_SPEC;
impl crate::RegisterSpec for CFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfcr::R](R) reader structure"]
impl crate::Readable for CFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfcr::W](W) writer structure"]
impl crate::Writable for CFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFCR to value 0"]
impl crate::Resettable for CFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
