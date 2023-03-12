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
#[doc = "Field `I2SEN` reader - I2SEN"]
pub type I2SEN_R = crate::BitReader<bool>;
#[doc = "Field `I2SEN` writer - I2SEN"]
pub type I2SEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TXEN` reader - TXEN"]
pub type TXEN_R = crate::BitReader<bool>;
#[doc = "Field `TXEN` writer - TXEN"]
pub type TXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RXEN` reader - RXEN"]
pub type RXEN_R = crate::BitReader<bool>;
#[doc = "Field `RXEN` writer - RXEN"]
pub type RXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SMPSIZE` reader - SMPSIZE"]
pub type SMPSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMPSIZE` writer - SMPSIZE"]
pub type SMPSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `FORMAT` reader - FORMAT"]
pub type FORMAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FORMAT` writer - FORMAT"]
pub type FORMAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `BITEXT` reader - BITEXT"]
pub type BITEXT_R = crate::BitReader<bool>;
#[doc = "Field `BITEXT` writer - BITEXT"]
pub type BITEXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `MCLKEN` reader - MCLKEN"]
pub type MCLKEN_R = crate::BitReader<bool>;
#[doc = "Field `MCLKEN` writer - MCLKEN"]
pub type MCLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `REPEAT` reader - REPEAT"]
pub type REPEAT_R = crate::BitReader<bool>;
#[doc = "Field `REPEAT` writer - REPEAT"]
pub type REPEAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CHANNEL` reader - CHANNEL"]
pub type CHANNEL_R = crate::BitReader<bool>;
#[doc = "Field `CHANNEL` writer - CHANNEL"]
pub type CHANNEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TXMUTE` reader - TXMUTE"]
pub type TXMUTE_R = crate::BitReader<bool>;
#[doc = "Field `TXMUTE` writer - TXMUTE"]
pub type TXMUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TXDMAEN` reader - TXDMAEN"]
pub type TXDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `TXDMAEN` writer - TXDMAEN"]
pub type TXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RXDMAEN` reader - RXDMAEN"]
pub type RXDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `RXDMAEN` writer - RXDMAEN"]
pub type RXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CLKDEN` reader - CLKDEN"]
pub type CLKDEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKDEN` writer - CLKDEN"]
pub type CLKDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RCEN` reader - RCEN"]
pub type RCEN_R = crate::BitReader<bool>;
#[doc = "Field `RCEN` writer - RCEN"]
pub type RCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RCSEL` reader - RCSEL"]
pub type RCSEL_R = crate::BitReader<bool>;
#[doc = "Field `RCSEL` writer - RCSEL"]
pub type RCSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `BCKINV` reader - BCKINV"]
pub type BCKINV_R = crate::BitReader<bool>;
#[doc = "Field `BCKINV` writer - BCKINV"]
pub type BCKINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `MCKINV` reader - MCKINV"]
pub type MCKINV_R = crate::BitReader<bool>;
#[doc = "Field `MCKINV` writer - MCKINV"]
pub type MCKINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - I2SEN"]
    #[inline(always)]
    pub fn i2sen(&self) -> I2SEN_R {
        I2SEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXEN"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXEN"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - SMPSIZE"]
    #[inline(always)]
    pub fn smpsize(&self) -> SMPSIZE_R {
        SMPSIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - FORMAT"]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - BITEXT"]
    #[inline(always)]
    pub fn bitext(&self) -> BITEXT_R {
        BITEXT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MCLKEN"]
    #[inline(always)]
    pub fn mclken(&self) -> MCLKEN_R {
        MCLKEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - REPEAT"]
    #[inline(always)]
    pub fn repeat(&self) -> REPEAT_R {
        REPEAT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CHANNEL"]
    #[inline(always)]
    pub fn channel(&self) -> CHANNEL_R {
        CHANNEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TXMUTE"]
    #[inline(always)]
    pub fn txmute(&self) -> TXMUTE_R {
        TXMUTE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TXDMAEN"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RXDMAEN"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CLKDEN"]
    #[inline(always)]
    pub fn clkden(&self) -> CLKDEN_R {
        CLKDEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RCEN"]
    #[inline(always)]
    pub fn rcen(&self) -> RCEN_R {
        RCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RCSEL"]
    #[inline(always)]
    pub fn rcsel(&self) -> RCSEL_R {
        RCSEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - BCKINV"]
    #[inline(always)]
    pub fn bckinv(&self) -> BCKINV_R {
        BCKINV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MCKINV"]
    #[inline(always)]
    pub fn mckinv(&self) -> MCKINV_R {
        MCKINV_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2SEN"]
    #[inline(always)]
    #[must_use]
    pub fn i2sen(&mut self) -> I2SEN_W<0> {
        I2SEN_W::new(self)
    }
    #[doc = "Bit 1 - TXEN"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<1> {
        TXEN_W::new(self)
    }
    #[doc = "Bit 2 - RXEN"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<2> {
        RXEN_W::new(self)
    }
    #[doc = "Bits 4:5 - SMPSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn smpsize(&mut self) -> SMPSIZE_W<4> {
        SMPSIZE_W::new(self)
    }
    #[doc = "Bits 6:7 - FORMAT"]
    #[inline(always)]
    #[must_use]
    pub fn format(&mut self) -> FORMAT_W<6> {
        FORMAT_W::new(self)
    }
    #[doc = "Bit 8 - BITEXT"]
    #[inline(always)]
    #[must_use]
    pub fn bitext(&mut self) -> BITEXT_W<8> {
        BITEXT_W::new(self)
    }
    #[doc = "Bit 9 - MCLKEN"]
    #[inline(always)]
    #[must_use]
    pub fn mclken(&mut self) -> MCLKEN_W<9> {
        MCLKEN_W::new(self)
    }
    #[doc = "Bit 10 - REPEAT"]
    #[inline(always)]
    #[must_use]
    pub fn repeat(&mut self) -> REPEAT_W<10> {
        REPEAT_W::new(self)
    }
    #[doc = "Bit 11 - CHANNEL"]
    #[inline(always)]
    #[must_use]
    pub fn channel(&mut self) -> CHANNEL_W<11> {
        CHANNEL_W::new(self)
    }
    #[doc = "Bit 12 - TXMUTE"]
    #[inline(always)]
    #[must_use]
    pub fn txmute(&mut self) -> TXMUTE_W<12> {
        TXMUTE_W::new(self)
    }
    #[doc = "Bit 13 - TXDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<13> {
        TXDMAEN_W::new(self)
    }
    #[doc = "Bit 14 - RXDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<14> {
        RXDMAEN_W::new(self)
    }
    #[doc = "Bit 15 - CLKDEN"]
    #[inline(always)]
    #[must_use]
    pub fn clkden(&mut self) -> CLKDEN_W<15> {
        CLKDEN_W::new(self)
    }
    #[doc = "Bit 16 - RCEN"]
    #[inline(always)]
    #[must_use]
    pub fn rcen(&mut self) -> RCEN_W<16> {
        RCEN_W::new(self)
    }
    #[doc = "Bit 17 - RCSEL"]
    #[inline(always)]
    #[must_use]
    pub fn rcsel(&mut self) -> RCSEL_W<17> {
        RCSEL_W::new(self)
    }
    #[doc = "Bit 18 - BCKINV"]
    #[inline(always)]
    #[must_use]
    pub fn bckinv(&mut self) -> BCKINV_W<18> {
        BCKINV_W::new(self)
    }
    #[doc = "Bit 19 - MCKINV"]
    #[inline(always)]
    #[must_use]
    pub fn mckinv(&mut self) -> MCKINV_W<19> {
        MCKINV_W::new(self)
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
