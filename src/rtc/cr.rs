#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCEN` reader - RTCEN"]
pub type RTCEN_R = crate::BitReader<bool>;
#[doc = "Field `RTCEN` writer - RTCEN"]
pub type RTCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RTCSRC` reader - RTCSRC"]
pub type RTCSRC_R = crate::BitReader<bool>;
#[doc = "Field `RTCSRC` writer - RTCSRC"]
pub type RTCSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `LSIEN` reader - LSIEN"]
pub type LSIEN_R = crate::BitReader<bool>;
#[doc = "Field `LSIEN` writer - LSIEN"]
pub type LSIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `LSEEN` reader - LSEEN"]
pub type LSEEN_R = crate::BitReader<bool>;
#[doc = "Field `LSEEN` writer - LSEEN"]
pub type LSEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CMPCLR` reader - CMPCLR"]
pub type CMPCLR_R = crate::BitReader<bool>;
#[doc = "Field `CMPCLR` writer - CMPCLR"]
pub type CMPCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `LSESM` reader - LSESM"]
pub type LSESM_R = crate::BitReader<bool>;
#[doc = "Field `LSESM` writer - LSESM"]
pub type LSESM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RPRE` reader - RPRE"]
pub type RPRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RPRE` writer - RPRE"]
pub type RPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 4, O>;
#[doc = "Field `ROEN` reader - ROEN"]
pub type ROEN_R = crate::BitReader<bool>;
#[doc = "Field `ROEN` writer - ROEN"]
pub type ROEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ROES` reader - ROES"]
pub type ROES_R = crate::BitReader<bool>;
#[doc = "Field `ROES` writer - ROES"]
pub type ROES_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ROWM` reader - ROWM"]
pub type ROWM_R = crate::BitReader<bool>;
#[doc = "Field `ROWM` writer - ROWM"]
pub type ROWM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ROAP` reader - ROAP"]
pub type ROAP_R = crate::BitReader<bool>;
#[doc = "Field `ROAP` writer - ROAP"]
pub type ROAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ROLF` reader - ROLF"]
pub type ROLF_R = crate::BitReader<bool>;
#[doc = "Field `ROLF` writer - ROLF"]
pub type ROLF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RTCEN"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTCSRC"]
    #[inline(always)]
    pub fn rtcsrc(&self) -> RTCSRC_R {
        RTCSRC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LSIEN"]
    #[inline(always)]
    pub fn lsien(&self) -> LSIEN_R {
        LSIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LSEEN"]
    #[inline(always)]
    pub fn lseen(&self) -> LSEEN_R {
        LSEEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CMPCLR"]
    #[inline(always)]
    pub fn cmpclr(&self) -> CMPCLR_R {
        CMPCLR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LSESM"]
    #[inline(always)]
    pub fn lsesm(&self) -> LSESM_R {
        LSESM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - RPRE"]
    #[inline(always)]
    pub fn rpre(&self) -> RPRE_R {
        RPRE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - ROEN"]
    #[inline(always)]
    pub fn roen(&self) -> ROEN_R {
        ROEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ROES"]
    #[inline(always)]
    pub fn roes(&self) -> ROES_R {
        ROES_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ROWM"]
    #[inline(always)]
    pub fn rowm(&self) -> ROWM_R {
        ROWM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ROAP"]
    #[inline(always)]
    pub fn roap(&self) -> ROAP_R {
        ROAP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ROLF"]
    #[inline(always)]
    pub fn rolf(&self) -> ROLF_R {
        ROLF_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTCEN"]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RTCEN_W<0> {
        RTCEN_W::new(self)
    }
    #[doc = "Bit 1 - RTCSRC"]
    #[inline(always)]
    #[must_use]
    pub fn rtcsrc(&mut self) -> RTCSRC_W<1> {
        RTCSRC_W::new(self)
    }
    #[doc = "Bit 2 - LSIEN"]
    #[inline(always)]
    #[must_use]
    pub fn lsien(&mut self) -> LSIEN_W<2> {
        LSIEN_W::new(self)
    }
    #[doc = "Bit 3 - LSEEN"]
    #[inline(always)]
    #[must_use]
    pub fn lseen(&mut self) -> LSEEN_W<3> {
        LSEEN_W::new(self)
    }
    #[doc = "Bit 4 - CMPCLR"]
    #[inline(always)]
    #[must_use]
    pub fn cmpclr(&mut self) -> CMPCLR_W<4> {
        CMPCLR_W::new(self)
    }
    #[doc = "Bit 5 - LSESM"]
    #[inline(always)]
    #[must_use]
    pub fn lsesm(&mut self) -> LSESM_W<5> {
        LSESM_W::new(self)
    }
    #[doc = "Bits 8:11 - RPRE"]
    #[inline(always)]
    #[must_use]
    pub fn rpre(&mut self) -> RPRE_W<8> {
        RPRE_W::new(self)
    }
    #[doc = "Bit 16 - ROEN"]
    #[inline(always)]
    #[must_use]
    pub fn roen(&mut self) -> ROEN_W<16> {
        ROEN_W::new(self)
    }
    #[doc = "Bit 17 - ROES"]
    #[inline(always)]
    #[must_use]
    pub fn roes(&mut self) -> ROES_W<17> {
        ROES_W::new(self)
    }
    #[doc = "Bit 18 - ROWM"]
    #[inline(always)]
    #[must_use]
    pub fn rowm(&mut self) -> ROWM_W<18> {
        ROWM_W::new(self)
    }
    #[doc = "Bit 19 - ROAP"]
    #[inline(always)]
    #[must_use]
    pub fn roap(&mut self) -> ROAP_W<19> {
        ROAP_W::new(self)
    }
    #[doc = "Bit 20 - ROLF"]
    #[inline(always)]
    #[must_use]
    pub fn rolf(&mut self) -> ROLF_W<20> {
        ROLF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
