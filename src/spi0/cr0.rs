#[doc = "Register `CR0` reader"]
pub struct R(crate::R<CR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR0` writer"]
pub struct W(crate::W<CR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR0_SPEC>;
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
impl From<crate::W<CR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPIEN` reader - SPIEN"]
pub type SPIEN_R = crate::BitReader<bool>;
#[doc = "Field `SPIEN` writer - SPIEN"]
pub type SPIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `TXDMAE` reader - TXDMAE"]
pub type TXDMAE_R = crate::BitReader<bool>;
#[doc = "Field `TXDMAE` writer - TXDMAE"]
pub type TXDMAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `RXDMAE` reader - RXDMAE"]
pub type RXDMAE_R = crate::BitReader<bool>;
#[doc = "Field `RXDMAE` writer - RXDMAE"]
pub type RXDMAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `SELOEN` reader - SELOEN"]
pub type SELOEN_R = crate::BitReader<bool>;
#[doc = "Field `SELOEN` writer - SELOEN"]
pub type SELOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `SSELC` reader - SSELC"]
pub type SSELC_R = crate::BitReader<bool>;
#[doc = "Field `SSELC` writer - SSELC"]
pub type SSELC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `DUALEN` reader - DUALEN"]
pub type DUALEN_R = crate::BitReader<bool>;
#[doc = "Field `DUALEN` writer - DUALEN"]
pub type DUALEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `GUADTEN` reader - GUADTEN"]
pub type GUADTEN_R = crate::BitReader<bool>;
#[doc = "Field `GUADTEN` writer - GUADTEN"]
pub type GUADTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `GUADT` reader - GUADT"]
pub type GUADT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GUADT` writer - GUADT"]
pub type GUADT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `SELHT` reader - SELHT"]
pub type SELHT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELHT` writer - SELHT"]
pub type SELHT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - SPIEN"]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXDMAE"]
    #[inline(always)]
    pub fn txdmae(&self) -> TXDMAE_R {
        TXDMAE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXDMAE"]
    #[inline(always)]
    pub fn rxdmae(&self) -> RXDMAE_R {
        RXDMAE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SELOEN"]
    #[inline(always)]
    pub fn seloen(&self) -> SELOEN_R {
        SELOEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SSELC"]
    #[inline(always)]
    pub fn sselc(&self) -> SSELC_R {
        SSELC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - DUALEN"]
    #[inline(always)]
    pub fn dualen(&self) -> DUALEN_R {
        DUALEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GUADTEN"]
    #[inline(always)]
    pub fn guadten(&self) -> GUADTEN_R {
        GUADTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - GUADT"]
    #[inline(always)]
    pub fn guadt(&self) -> GUADT_R {
        GUADT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SELHT"]
    #[inline(always)]
    pub fn selht(&self) -> SELHT_R {
        SELHT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SPIEN"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SPIEN_W<0> {
        SPIEN_W::new(self)
    }
    #[doc = "Bit 1 - TXDMAE"]
    #[inline(always)]
    #[must_use]
    pub fn txdmae(&mut self) -> TXDMAE_W<1> {
        TXDMAE_W::new(self)
    }
    #[doc = "Bit 2 - RXDMAE"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmae(&mut self) -> RXDMAE_W<2> {
        RXDMAE_W::new(self)
    }
    #[doc = "Bit 3 - SELOEN"]
    #[inline(always)]
    #[must_use]
    pub fn seloen(&mut self) -> SELOEN_W<3> {
        SELOEN_W::new(self)
    }
    #[doc = "Bit 4 - SSELC"]
    #[inline(always)]
    #[must_use]
    pub fn sselc(&mut self) -> SSELC_W<4> {
        SSELC_W::new(self)
    }
    #[doc = "Bit 6 - DUALEN"]
    #[inline(always)]
    #[must_use]
    pub fn dualen(&mut self) -> DUALEN_W<6> {
        DUALEN_W::new(self)
    }
    #[doc = "Bit 7 - GUADTEN"]
    #[inline(always)]
    #[must_use]
    pub fn guadten(&mut self) -> GUADTEN_W<7> {
        GUADTEN_W::new(self)
    }
    #[doc = "Bits 8:11 - GUADT"]
    #[inline(always)]
    #[must_use]
    pub fn guadt(&mut self) -> GUADT_W<8> {
        GUADT_W::new(self)
    }
    #[doc = "Bits 12:15 - SELHT"]
    #[inline(always)]
    #[must_use]
    pub fn selht(&mut self) -> SELHT_W<12> {
        SELHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr0](index.html) module"]
pub struct CR0_SPEC;
impl crate::RegisterSpec for CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr0::R](R) reader structure"]
impl crate::Readable for CR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr0::W](W) writer structure"]
impl crate::Writable for CR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for CR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
