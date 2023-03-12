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
#[doc = "Field `AA` reader - AA"]
pub type AA_R = crate::BitReader<bool>;
#[doc = "Field `AA` writer - AA"]
pub type AA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `STOP` reader - STOP"]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - STOP"]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `GCEN` reader - GCEN"]
pub type GCEN_R = crate::BitReader<bool>;
#[doc = "Field `GCEN` writer - GCEN"]
pub type GCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `I2CEN` reader - I2CEN"]
pub type I2CEN_R = crate::BitReader<bool>;
#[doc = "Field `I2CEN` writer - I2CEN"]
pub type I2CEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ADRM` reader - ADRM"]
pub type ADRM_R = crate::BitReader<bool>;
#[doc = "Field `ADRM` writer - ADRM"]
pub type ADRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TXDMAE` reader - TXDMAE"]
pub type TXDMAE_R = crate::BitReader<bool>;
#[doc = "Field `TXDMAE` writer - TXDMAE"]
pub type TXDMAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RXDMAE` reader - RXDMAE"]
pub type RXDMAE_R = crate::BitReader<bool>;
#[doc = "Field `RXDMAE` writer - RXDMAE"]
pub type RXDMAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DMANACK` reader - DMANACK"]
pub type DMANACK_R = crate::BitReader<bool>;
#[doc = "Field `DMANACK` writer - DMANACK"]
pub type DMANACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ENTOUT` reader - ENTOUT"]
pub type ENTOUT_R = crate::BitReader<bool>;
#[doc = "Field `ENTOUT` writer - ENTOUT"]
pub type ENTOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `COMB_FILTER_EN` reader - COMB_FILTER_EN"]
pub type COMB_FILTER_EN_R = crate::BitReader<bool>;
#[doc = "Field `COMB_FILTER_EN` writer - COMB_FILTER_EN"]
pub type COMB_FILTER_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SEQ_FILTER` reader - SEQ_FILTER"]
pub type SEQ_FILTER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEQ_FILTER` writer - SEQ_FILTER"]
pub type SEQ_FILTER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - AA"]
    #[inline(always)]
    pub fn aa(&self) -> AA_R {
        AA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STOP"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GCEN"]
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2CEN"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2CEN_R {
        I2CEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - ADRM"]
    #[inline(always)]
    pub fn adrm(&self) -> ADRM_R {
        ADRM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TXDMAE"]
    #[inline(always)]
    pub fn txdmae(&self) -> TXDMAE_R {
        TXDMAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXDMAE"]
    #[inline(always)]
    pub fn rxdmae(&self) -> RXDMAE_R {
        RXDMAE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMANACK"]
    #[inline(always)]
    pub fn dmanack(&self) -> DMANACK_R {
        DMANACK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - ENTOUT"]
    #[inline(always)]
    pub fn entout(&self) -> ENTOUT_R {
        ENTOUT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - COMB_FILTER_EN"]
    #[inline(always)]
    pub fn comb_filter_en(&self) -> COMB_FILTER_EN_R {
        COMB_FILTER_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - SEQ_FILTER"]
    #[inline(always)]
    pub fn seq_filter(&self) -> SEQ_FILTER_R {
        SEQ_FILTER_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AA"]
    #[inline(always)]
    #[must_use]
    pub fn aa(&mut self) -> AA_W<0> {
        AA_W::new(self)
    }
    #[doc = "Bit 1 - STOP"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<1> {
        STOP_W::new(self)
    }
    #[doc = "Bit 2 - GCEN"]
    #[inline(always)]
    #[must_use]
    pub fn gcen(&mut self) -> GCEN_W<2> {
        GCEN_W::new(self)
    }
    #[doc = "Bit 3 - I2CEN"]
    #[inline(always)]
    #[must_use]
    pub fn i2cen(&mut self) -> I2CEN_W<3> {
        I2CEN_W::new(self)
    }
    #[doc = "Bit 7 - ADRM"]
    #[inline(always)]
    #[must_use]
    pub fn adrm(&mut self) -> ADRM_W<7> {
        ADRM_W::new(self)
    }
    #[doc = "Bit 8 - TXDMAE"]
    #[inline(always)]
    #[must_use]
    pub fn txdmae(&mut self) -> TXDMAE_W<8> {
        TXDMAE_W::new(self)
    }
    #[doc = "Bit 9 - RXDMAE"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmae(&mut self) -> RXDMAE_W<9> {
        RXDMAE_W::new(self)
    }
    #[doc = "Bit 10 - DMANACK"]
    #[inline(always)]
    #[must_use]
    pub fn dmanack(&mut self) -> DMANACK_W<10> {
        DMANACK_W::new(self)
    }
    #[doc = "Bit 12 - ENTOUT"]
    #[inline(always)]
    #[must_use]
    pub fn entout(&mut self) -> ENTOUT_W<12> {
        ENTOUT_W::new(self)
    }
    #[doc = "Bit 13 - COMB_FILTER_EN"]
    #[inline(always)]
    #[must_use]
    pub fn comb_filter_en(&mut self) -> COMB_FILTER_EN_W<13> {
        COMB_FILTER_EN_W::new(self)
    }
    #[doc = "Bits 14:15 - SEQ_FILTER"]
    #[inline(always)]
    #[must_use]
    pub fn seq_filter(&mut self) -> SEQ_FILTER_W<14> {
        SEQ_FILTER_W::new(self)
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
