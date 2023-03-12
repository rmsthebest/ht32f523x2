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
#[doc = "Field `MODE0` reader - MODE0"]
pub type MODE0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE0` writer - MODE0"]
pub type MODE0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE1` reader - MODE1"]
pub type MODE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE1` writer - MODE1"]
pub type MODE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE2` reader - MODE2"]
pub type MODE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE2` writer - MODE2"]
pub type MODE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE3` reader - MODE3"]
pub type MODE3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE3` writer - MODE3"]
pub type MODE3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `BANKEN0` reader - BANKEN0"]
pub type BANKEN0_R = crate::BitReader<bool>;
#[doc = "Field `BANKEN0` writer - BANKEN0"]
pub type BANKEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `BANKEN1` reader - BANKEN1"]
pub type BANKEN1_R = crate::BitReader<bool>;
#[doc = "Field `BANKEN1` writer - BANKEN1"]
pub type BANKEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `BANKEN2` reader - BANKEN2"]
pub type BANKEN2_R = crate::BitReader<bool>;
#[doc = "Field `BANKEN2` writer - BANKEN2"]
pub type BANKEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `BANKEN3` reader - BANKEN3"]
pub type BANKEN3_R = crate::BitReader<bool>;
#[doc = "Field `BANKEN3` writer - BANKEN3"]
pub type BANKEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `NOIDLE0` reader - NOIDLE0"]
pub type NOIDLE0_R = crate::BitReader<bool>;
#[doc = "Field `NOIDLE0` writer - NOIDLE0"]
pub type NOIDLE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `NOIDLE1` reader - NOIDLE1"]
pub type NOIDLE1_R = crate::BitReader<bool>;
#[doc = "Field `NOIDLE1` writer - NOIDLE1"]
pub type NOIDLE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `NOIDLE2` reader - NOIDLE2"]
pub type NOIDLE2_R = crate::BitReader<bool>;
#[doc = "Field `NOIDLE2` writer - NOIDLE2"]
pub type NOIDLE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `NOIDLE3` reader - NOIDLE3"]
pub type NOIDLE3_R = crate::BitReader<bool>;
#[doc = "Field `NOIDLE3` writer - NOIDLE3"]
pub type NOIDLE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `IDLET` reader - IDLET"]
pub type IDLET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDLET` writer - IDLET"]
pub type IDLET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1 - MODE0"]
    #[inline(always)]
    pub fn mode0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - MODE1"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - MODE2"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - MODE3"]
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - BANKEN0"]
    #[inline(always)]
    pub fn banken0(&self) -> BANKEN0_R {
        BANKEN0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BANKEN1"]
    #[inline(always)]
    pub fn banken1(&self) -> BANKEN1_R {
        BANKEN1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BANKEN2"]
    #[inline(always)]
    pub fn banken2(&self) -> BANKEN2_R {
        BANKEN2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BANKEN3"]
    #[inline(always)]
    pub fn banken3(&self) -> BANKEN3_R {
        BANKEN3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NOIDLE0"]
    #[inline(always)]
    pub fn noidle0(&self) -> NOIDLE0_R {
        NOIDLE0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NOIDLE1"]
    #[inline(always)]
    pub fn noidle1(&self) -> NOIDLE1_R {
        NOIDLE1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NOIDLE2"]
    #[inline(always)]
    pub fn noidle2(&self) -> NOIDLE2_R {
        NOIDLE2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NOIDLE3"]
    #[inline(always)]
    pub fn noidle3(&self) -> NOIDLE3_R {
        NOIDLE3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 28:31 - IDLET"]
    #[inline(always)]
    pub fn idlet(&self) -> IDLET_R {
        IDLET_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - MODE0"]
    #[inline(always)]
    #[must_use]
    pub fn mode0(&mut self) -> MODE0_W<0> {
        MODE0_W::new(self)
    }
    #[doc = "Bits 2:3 - MODE1"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<2> {
        MODE1_W::new(self)
    }
    #[doc = "Bits 4:5 - MODE2"]
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> MODE2_W<4> {
        MODE2_W::new(self)
    }
    #[doc = "Bits 6:7 - MODE3"]
    #[inline(always)]
    #[must_use]
    pub fn mode3(&mut self) -> MODE3_W<6> {
        MODE3_W::new(self)
    }
    #[doc = "Bit 8 - BANKEN0"]
    #[inline(always)]
    #[must_use]
    pub fn banken0(&mut self) -> BANKEN0_W<8> {
        BANKEN0_W::new(self)
    }
    #[doc = "Bit 9 - BANKEN1"]
    #[inline(always)]
    #[must_use]
    pub fn banken1(&mut self) -> BANKEN1_W<9> {
        BANKEN1_W::new(self)
    }
    #[doc = "Bit 10 - BANKEN2"]
    #[inline(always)]
    #[must_use]
    pub fn banken2(&mut self) -> BANKEN2_W<10> {
        BANKEN2_W::new(self)
    }
    #[doc = "Bit 11 - BANKEN3"]
    #[inline(always)]
    #[must_use]
    pub fn banken3(&mut self) -> BANKEN3_W<11> {
        BANKEN3_W::new(self)
    }
    #[doc = "Bit 12 - NOIDLE0"]
    #[inline(always)]
    #[must_use]
    pub fn noidle0(&mut self) -> NOIDLE0_W<12> {
        NOIDLE0_W::new(self)
    }
    #[doc = "Bit 13 - NOIDLE1"]
    #[inline(always)]
    #[must_use]
    pub fn noidle1(&mut self) -> NOIDLE1_W<13> {
        NOIDLE1_W::new(self)
    }
    #[doc = "Bit 14 - NOIDLE2"]
    #[inline(always)]
    #[must_use]
    pub fn noidle2(&mut self) -> NOIDLE2_W<14> {
        NOIDLE2_W::new(self)
    }
    #[doc = "Bit 15 - NOIDLE3"]
    #[inline(always)]
    #[must_use]
    pub fn noidle3(&mut self) -> NOIDLE3_W<15> {
        NOIDLE3_W::new(self)
    }
    #[doc = "Bits 28:31 - IDLET"]
    #[inline(always)]
    #[must_use]
    pub fn idlet(&mut self) -> IDLET_W<28> {
        IDLET_W::new(self)
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
