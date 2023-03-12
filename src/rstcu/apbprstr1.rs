#[doc = "Register `APBPRSTR1` reader"]
pub struct R(crate::R<APBPRSTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBPRSTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBPRSTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBPRSTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBPRSTR1` writer"]
pub struct W(crate::W<APBPRSTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBPRSTR1_SPEC>;
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
impl From<crate::W<APBPRSTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBPRSTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCTM0RST` reader - MCTM0RST"]
pub type MCTM0RST_R = crate::BitReader<bool>;
#[doc = "Field `MCTM0RST` writer - MCTM0RST"]
pub type MCTM0RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBPRSTR1_SPEC, bool, O>;
#[doc = "Field `WDTRST` reader - WDTRST"]
pub type WDTRST_R = crate::BitReader<bool>;
#[doc = "Field `WDTRST` writer - WDTRST"]
pub type WDTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBPRSTR1_SPEC, bool, O>;
#[doc = "Field `GPTM0RST` reader - GPTM0RST"]
pub type GPTM0RST_R = crate::BitReader<bool>;
#[doc = "Field `GPTM0RST` writer - GPTM0RST"]
pub type GPTM0RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBPRSTR1_SPEC, bool, O>;
#[doc = "Field `GPTM1RST` reader - GPTM1RST"]
pub type GPTM1RST_R = crate::BitReader<bool>;
#[doc = "Field `GPTM1RST` writer - GPTM1RST"]
pub type GPTM1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBPRSTR1_SPEC, bool, O>;
#[doc = "Field `BFTM0RST` reader - BFTM0RST"]
pub type BFTM0RST_R = crate::BitReader<bool>;
#[doc = "Field `BFTM0RST` writer - BFTM0RST"]
pub type BFTM0RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBPRSTR1_SPEC, bool, O>;
#[doc = "Field `BFTM1RST` reader - BFTM1RST"]
pub type BFTM1RST_R = crate::BitReader<bool>;
#[doc = "Field `BFTM1RST` writer - BFTM1RST"]
pub type BFTM1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBPRSTR1_SPEC, bool, O>;
#[doc = "Field `CMPRST` reader - CMPRST"]
pub type CMPRST_R = crate::BitReader<bool>;
#[doc = "Field `CMPRST` writer - CMPRST"]
pub type CMPRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBPRSTR1_SPEC, bool, O>;
#[doc = "Field `ADCRST` reader - ADCRST"]
pub type ADCRST_R = crate::BitReader<bool>;
#[doc = "Field `ADCRST` writer - ADCRST"]
pub type ADCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBPRSTR1_SPEC, bool, O>;
#[doc = "Field `SCTM0RST` reader - SCTM0RST"]
pub type SCTM0RST_R = crate::BitReader<bool>;
#[doc = "Field `SCTM0RST` writer - SCTM0RST"]
pub type SCTM0RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBPRSTR1_SPEC, bool, O>;
#[doc = "Field `SCTM1RST` reader - SCTM1RST"]
pub type SCTM1RST_R = crate::BitReader<bool>;
#[doc = "Field `SCTM1RST` writer - SCTM1RST"]
pub type SCTM1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APBPRSTR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - MCTM0RST"]
    #[inline(always)]
    pub fn mctm0rst(&self) -> MCTM0RST_R {
        MCTM0RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - WDTRST"]
    #[inline(always)]
    pub fn wdtrst(&self) -> WDTRST_R {
        WDTRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM0RST"]
    #[inline(always)]
    pub fn gptm0rst(&self) -> GPTM0RST_R {
        GPTM0RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM1RST"]
    #[inline(always)]
    pub fn gptm1rst(&self) -> GPTM1RST_R {
        GPTM1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - BFTM0RST"]
    #[inline(always)]
    pub fn bftm0rst(&self) -> BFTM0RST_R {
        BFTM0RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - BFTM1RST"]
    #[inline(always)]
    pub fn bftm1rst(&self) -> BFTM1RST_R {
        BFTM1RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 22 - CMPRST"]
    #[inline(always)]
    pub fn cmprst(&self) -> CMPRST_R {
        CMPRST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - ADCRST"]
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - SCTM0RST"]
    #[inline(always)]
    pub fn sctm0rst(&self) -> SCTM0RST_R {
        SCTM0RST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SCTM1RST"]
    #[inline(always)]
    pub fn sctm1rst(&self) -> SCTM1RST_R {
        SCTM1RST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCTM0RST"]
    #[inline(always)]
    #[must_use]
    pub fn mctm0rst(&mut self) -> MCTM0RST_W<0> {
        MCTM0RST_W::new(self)
    }
    #[doc = "Bit 4 - WDTRST"]
    #[inline(always)]
    #[must_use]
    pub fn wdtrst(&mut self) -> WDTRST_W<4> {
        WDTRST_W::new(self)
    }
    #[doc = "Bit 8 - GPTM0RST"]
    #[inline(always)]
    #[must_use]
    pub fn gptm0rst(&mut self) -> GPTM0RST_W<8> {
        GPTM0RST_W::new(self)
    }
    #[doc = "Bit 9 - GPTM1RST"]
    #[inline(always)]
    #[must_use]
    pub fn gptm1rst(&mut self) -> GPTM1RST_W<9> {
        GPTM1RST_W::new(self)
    }
    #[doc = "Bit 16 - BFTM0RST"]
    #[inline(always)]
    #[must_use]
    pub fn bftm0rst(&mut self) -> BFTM0RST_W<16> {
        BFTM0RST_W::new(self)
    }
    #[doc = "Bit 17 - BFTM1RST"]
    #[inline(always)]
    #[must_use]
    pub fn bftm1rst(&mut self) -> BFTM1RST_W<17> {
        BFTM1RST_W::new(self)
    }
    #[doc = "Bit 22 - CMPRST"]
    #[inline(always)]
    #[must_use]
    pub fn cmprst(&mut self) -> CMPRST_W<22> {
        CMPRST_W::new(self)
    }
    #[doc = "Bit 24 - ADCRST"]
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> ADCRST_W<24> {
        ADCRST_W::new(self)
    }
    #[doc = "Bit 28 - SCTM0RST"]
    #[inline(always)]
    #[must_use]
    pub fn sctm0rst(&mut self) -> SCTM0RST_W<28> {
        SCTM0RST_W::new(self)
    }
    #[doc = "Bit 29 - SCTM1RST"]
    #[inline(always)]
    #[must_use]
    pub fn sctm1rst(&mut self) -> SCTM1RST_W<29> {
        SCTM1RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APBPRSTR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbprstr1](index.html) module"]
pub struct APBPRSTR1_SPEC;
impl crate::RegisterSpec for APBPRSTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbprstr1::R](R) reader structure"]
impl crate::Readable for APBPRSTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbprstr1::W](W) writer structure"]
impl crate::Writable for APBPRSTR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBPRSTR1 to value 0"]
impl crate::Resettable for APBPRSTR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
