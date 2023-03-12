#[doc = "Register `OIER` reader"]
pub struct R(crate::R<OIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OIER` writer"]
pub struct W(crate::W<OIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OIER_SPEC>;
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
impl From<crate::W<OIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ORFIEN` reader - ORFIEN"]
pub type ORFIEN_R = crate::BitReader<bool>;
#[doc = "Field `ORFIEN` writer - ORFIEN"]
pub type ORFIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OIER_SPEC, bool, O>;
#[doc = "Field `ITADIEN` reader - ITADIEN"]
pub type ITADIEN_R = crate::BitReader<bool>;
#[doc = "Field `ITADIEN` writer - ITADIEN"]
pub type ITADIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OIER_SPEC, bool, O>;
#[doc = "Field `OBEIEN` reader - OBEIEN"]
pub type OBEIEN_R = crate::BitReader<bool>;
#[doc = "Field `OBEIEN` writer - OBEIEN"]
pub type OBEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OIER_SPEC, bool, O>;
#[doc = "Field `IOCMIEN` reader - IOCMIEN"]
pub type IOCMIEN_R = crate::BitReader<bool>;
#[doc = "Field `IOCMIEN` writer - IOCMIEN"]
pub type IOCMIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OIER_SPEC, bool, O>;
#[doc = "Field `OREIEN` reader - OREIEN"]
pub type OREIEN_R = crate::BitReader<bool>;
#[doc = "Field `OREIEN` writer - OREIEN"]
pub type OREIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OIER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ORFIEN"]
    #[inline(always)]
    pub fn orfien(&self) -> ORFIEN_R {
        ORFIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ITADIEN"]
    #[inline(always)]
    pub fn itadien(&self) -> ITADIEN_R {
        ITADIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OBEIEN"]
    #[inline(always)]
    pub fn obeien(&self) -> OBEIEN_R {
        OBEIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IOCMIEN"]
    #[inline(always)]
    pub fn iocmien(&self) -> IOCMIEN_R {
        IOCMIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OREIEN"]
    #[inline(always)]
    pub fn oreien(&self) -> OREIEN_R {
        OREIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ORFIEN"]
    #[inline(always)]
    #[must_use]
    pub fn orfien(&mut self) -> ORFIEN_W<0> {
        ORFIEN_W::new(self)
    }
    #[doc = "Bit 1 - ITADIEN"]
    #[inline(always)]
    #[must_use]
    pub fn itadien(&mut self) -> ITADIEN_W<1> {
        ITADIEN_W::new(self)
    }
    #[doc = "Bit 2 - OBEIEN"]
    #[inline(always)]
    #[must_use]
    pub fn obeien(&mut self) -> OBEIEN_W<2> {
        OBEIEN_W::new(self)
    }
    #[doc = "Bit 3 - IOCMIEN"]
    #[inline(always)]
    #[must_use]
    pub fn iocmien(&mut self) -> IOCMIEN_W<3> {
        IOCMIEN_W::new(self)
    }
    #[doc = "Bit 4 - OREIEN"]
    #[inline(always)]
    #[must_use]
    pub fn oreien(&mut self) -> OREIEN_W<4> {
        OREIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OIER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oier](index.html) module"]
pub struct OIER_SPEC;
impl crate::RegisterSpec for OIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oier::R](R) reader structure"]
impl crate::Readable for OIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oier::W](W) writer structure"]
impl crate::Writable for OIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OIER to value 0"]
impl crate::Resettable for OIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
