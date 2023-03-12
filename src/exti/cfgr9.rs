#[doc = "Register `CFGR9` reader"]
pub struct R(crate::R<CFGR9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR9` writer"]
pub struct W(crate::W<CFGR9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR9_SPEC>;
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
impl From<crate::W<CFGR9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBCNT` reader - DBCNT"]
pub type DBCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DBCNT` writer - DBCNT"]
pub type DBCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR9_SPEC, u16, u16, 16, O>;
#[doc = "Field `SRCTYPE` reader - SRCTYPE"]
pub type SRCTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRCTYPE` writer - SRCTYPE"]
pub type SRCTYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR9_SPEC, u8, u8, 3, O>;
#[doc = "Field `DBEN` reader - DBEN"]
pub type DBEN_R = crate::BitReader<bool>;
#[doc = "Field `DBEN` writer - DBEN"]
pub type DBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR9_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - DBCNT"]
    #[inline(always)]
    pub fn dbcnt(&self) -> DBCNT_R {
        DBCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 28:30 - SRCTYPE"]
    #[inline(always)]
    pub fn srctype(&self) -> SRCTYPE_R {
        SRCTYPE_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - DBEN"]
    #[inline(always)]
    pub fn dben(&self) -> DBEN_R {
        DBEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - DBCNT"]
    #[inline(always)]
    #[must_use]
    pub fn dbcnt(&mut self) -> DBCNT_W<0> {
        DBCNT_W::new(self)
    }
    #[doc = "Bits 28:30 - SRCTYPE"]
    #[inline(always)]
    #[must_use]
    pub fn srctype(&mut self) -> SRCTYPE_W<28> {
        SRCTYPE_W::new(self)
    }
    #[doc = "Bit 31 - DBEN"]
    #[inline(always)]
    #[must_use]
    pub fn dben(&mut self) -> DBEN_W<31> {
        DBEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CFGR9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr9](index.html) module"]
pub struct CFGR9_SPEC;
impl crate::RegisterSpec for CFGR9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr9::R](R) reader structure"]
impl crate::Readable for CFGR9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr9::W](W) writer structure"]
impl crate::Writable for CFGR9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR9 to value 0"]
impl crate::Resettable for CFGR9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
