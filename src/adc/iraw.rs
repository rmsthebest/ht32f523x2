#[doc = "Register `IRAW` reader"]
pub struct R(crate::R<IRAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRAW` writer"]
pub struct W(crate::W<IRAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRAW_SPEC>;
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
impl From<crate::W<IRAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADIRAWS` reader - ADIRAWS"]
pub type ADIRAWS_R = crate::BitReader<bool>;
#[doc = "Field `ADIRAWS` writer - ADIRAWS"]
pub type ADIRAWS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRAW_SPEC, bool, O>;
#[doc = "Field `ADIRAWG` reader - ADIRAWG"]
pub type ADIRAWG_R = crate::BitReader<bool>;
#[doc = "Field `ADIRAWG` writer - ADIRAWG"]
pub type ADIRAWG_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRAW_SPEC, bool, O>;
#[doc = "Field `ADIRAWC` reader - ADIRAWC"]
pub type ADIRAWC_R = crate::BitReader<bool>;
#[doc = "Field `ADIRAWC` writer - ADIRAWC"]
pub type ADIRAWC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRAW_SPEC, bool, O>;
#[doc = "Field `ADIRAWL` reader - ADIRAWL"]
pub type ADIRAWL_R = crate::BitReader<bool>;
#[doc = "Field `ADIRAWL` writer - ADIRAWL"]
pub type ADIRAWL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRAW_SPEC, bool, O>;
#[doc = "Field `ADIRAWU` reader - ADIRAWU"]
pub type ADIRAWU_R = crate::BitReader<bool>;
#[doc = "Field `ADIRAWU` writer - ADIRAWU"]
pub type ADIRAWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRAW_SPEC, bool, O>;
#[doc = "Field `ADIRAWO` reader - ADIRAWO"]
pub type ADIRAWO_R = crate::BitReader<bool>;
#[doc = "Field `ADIRAWO` writer - ADIRAWO"]
pub type ADIRAWO_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRAW_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ADIRAWS"]
    #[inline(always)]
    pub fn adiraws(&self) -> ADIRAWS_R {
        ADIRAWS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADIRAWG"]
    #[inline(always)]
    pub fn adirawg(&self) -> ADIRAWG_R {
        ADIRAWG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADIRAWC"]
    #[inline(always)]
    pub fn adirawc(&self) -> ADIRAWC_R {
        ADIRAWC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - ADIRAWL"]
    #[inline(always)]
    pub fn adirawl(&self) -> ADIRAWL_R {
        ADIRAWL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADIRAWU"]
    #[inline(always)]
    pub fn adirawu(&self) -> ADIRAWU_R {
        ADIRAWU_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - ADIRAWO"]
    #[inline(always)]
    pub fn adirawo(&self) -> ADIRAWO_R {
        ADIRAWO_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADIRAWS"]
    #[inline(always)]
    #[must_use]
    pub fn adiraws(&mut self) -> ADIRAWS_W<0> {
        ADIRAWS_W::new(self)
    }
    #[doc = "Bit 1 - ADIRAWG"]
    #[inline(always)]
    #[must_use]
    pub fn adirawg(&mut self) -> ADIRAWG_W<1> {
        ADIRAWG_W::new(self)
    }
    #[doc = "Bit 2 - ADIRAWC"]
    #[inline(always)]
    #[must_use]
    pub fn adirawc(&mut self) -> ADIRAWC_W<2> {
        ADIRAWC_W::new(self)
    }
    #[doc = "Bit 16 - ADIRAWL"]
    #[inline(always)]
    #[must_use]
    pub fn adirawl(&mut self) -> ADIRAWL_W<16> {
        ADIRAWL_W::new(self)
    }
    #[doc = "Bit 17 - ADIRAWU"]
    #[inline(always)]
    #[must_use]
    pub fn adirawu(&mut self) -> ADIRAWU_W<17> {
        ADIRAWU_W::new(self)
    }
    #[doc = "Bit 24 - ADIRAWO"]
    #[inline(always)]
    #[must_use]
    pub fn adirawo(&mut self) -> ADIRAWO_W<24> {
        ADIRAWO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IRAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iraw](index.html) module"]
pub struct IRAW_SPEC;
impl crate::RegisterSpec for IRAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iraw::R](R) reader structure"]
impl crate::Readable for IRAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iraw::W](W) writer structure"]
impl crate::Writable for IRAW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRAW to value 0"]
impl crate::Resettable for IRAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
