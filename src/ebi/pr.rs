#[doc = "Register `PR` reader"]
pub struct R(crate::R<PR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PR` writer"]
pub struct W(crate::W<PR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR_SPEC>;
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
impl From<crate::W<PR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSPOL` reader - CSPOL"]
pub type CSPOL_R = crate::BitReader<bool>;
#[doc = "Field `CSPOL` writer - CSPOL"]
pub type CSPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `OEPOL` reader - OEPOL"]
pub type OEPOL_R = crate::BitReader<bool>;
#[doc = "Field `OEPOL` writer - OEPOL"]
pub type OEPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `WEPOL` reader - WEPOL"]
pub type WEPOL_R = crate::BitReader<bool>;
#[doc = "Field `WEPOL` writer - WEPOL"]
pub type WEPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `ALEPOL` reader - ALEPOL"]
pub type ALEPOL_R = crate::BitReader<bool>;
#[doc = "Field `ALEPOL` writer - ALEPOL"]
pub type ALEPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CSPOL"]
    #[inline(always)]
    pub fn cspol(&self) -> CSPOL_R {
        CSPOL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OEPOL"]
    #[inline(always)]
    pub fn oepol(&self) -> OEPOL_R {
        OEPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WEPOL"]
    #[inline(always)]
    pub fn wepol(&self) -> WEPOL_R {
        WEPOL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ALEPOL"]
    #[inline(always)]
    pub fn alepol(&self) -> ALEPOL_R {
        ALEPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CSPOL"]
    #[inline(always)]
    #[must_use]
    pub fn cspol(&mut self) -> CSPOL_W<0> {
        CSPOL_W::new(self)
    }
    #[doc = "Bit 1 - OEPOL"]
    #[inline(always)]
    #[must_use]
    pub fn oepol(&mut self) -> OEPOL_W<1> {
        OEPOL_W::new(self)
    }
    #[doc = "Bit 2 - WEPOL"]
    #[inline(always)]
    #[must_use]
    pub fn wepol(&mut self) -> WEPOL_W<2> {
        WEPOL_W::new(self)
    }
    #[doc = "Bit 3 - ALEPOL"]
    #[inline(always)]
    #[must_use]
    pub fn alepol(&mut self) -> ALEPOL_W<3> {
        ALEPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr](index.html) module"]
pub struct PR_SPEC;
impl crate::RegisterSpec for PR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pr::R](R) reader structure"]
impl crate::Readable for PR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pr::W](W) writer structure"]
impl crate::Writable for PR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PR to value 0"]
impl crate::Resettable for PR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
