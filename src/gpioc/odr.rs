#[doc = "Register `ODR` reader"]
pub struct R(crate::R<ODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ODR` writer"]
pub struct W(crate::W<ODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ODR_SPEC>;
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
impl From<crate::W<ODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OD0` reader - OD0"]
pub type OD0_R = crate::BitReader<bool>;
#[doc = "Field `OD0` writer - OD0"]
pub type OD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `OD1` reader - OD1"]
pub type OD1_R = crate::BitReader<bool>;
#[doc = "Field `OD1` writer - OD1"]
pub type OD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `OD2` reader - OD2"]
pub type OD2_R = crate::BitReader<bool>;
#[doc = "Field `OD2` writer - OD2"]
pub type OD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `OD3` reader - OD3"]
pub type OD3_R = crate::BitReader<bool>;
#[doc = "Field `OD3` writer - OD3"]
pub type OD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `OD4` reader - OD4"]
pub type OD4_R = crate::BitReader<bool>;
#[doc = "Field `OD4` writer - OD4"]
pub type OD4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `OD5` reader - OD5"]
pub type OD5_R = crate::BitReader<bool>;
#[doc = "Field `OD5` writer - OD5"]
pub type OD5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `OD6` reader - OD6"]
pub type OD6_R = crate::BitReader<bool>;
#[doc = "Field `OD6` writer - OD6"]
pub type OD6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `OD7` reader - OD7"]
pub type OD7_R = crate::BitReader<bool>;
#[doc = "Field `OD7` writer - OD7"]
pub type OD7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `OD8` reader - OD8"]
pub type OD8_R = crate::BitReader<bool>;
#[doc = "Field `OD8` writer - OD8"]
pub type OD8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `OD9` reader - OD9"]
pub type OD9_R = crate::BitReader<bool>;
#[doc = "Field `OD9` writer - OD9"]
pub type OD9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `OD10` reader - OD10"]
pub type OD10_R = crate::BitReader<bool>;
#[doc = "Field `OD10` writer - OD10"]
pub type OD10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `OD11` reader - OD11"]
pub type OD11_R = crate::BitReader<bool>;
#[doc = "Field `OD11` writer - OD11"]
pub type OD11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `OD12` reader - OD12"]
pub type OD12_R = crate::BitReader<bool>;
#[doc = "Field `OD12` writer - OD12"]
pub type OD12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `OD13` reader - OD13"]
pub type OD13_R = crate::BitReader<bool>;
#[doc = "Field `OD13` writer - OD13"]
pub type OD13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `OD14` reader - OD14"]
pub type OD14_R = crate::BitReader<bool>;
#[doc = "Field `OD14` writer - OD14"]
pub type OD14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
#[doc = "Field `OD15` reader - OD15"]
pub type OD15_R = crate::BitReader<bool>;
#[doc = "Field `OD15` writer - OD15"]
pub type OD15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - OD0"]
    #[inline(always)]
    pub fn od0(&self) -> OD0_R {
        OD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OD1"]
    #[inline(always)]
    pub fn od1(&self) -> OD1_R {
        OD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OD2"]
    #[inline(always)]
    pub fn od2(&self) -> OD2_R {
        OD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OD3"]
    #[inline(always)]
    pub fn od3(&self) -> OD3_R {
        OD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OD4"]
    #[inline(always)]
    pub fn od4(&self) -> OD4_R {
        OD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OD5"]
    #[inline(always)]
    pub fn od5(&self) -> OD5_R {
        OD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OD6"]
    #[inline(always)]
    pub fn od6(&self) -> OD6_R {
        OD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - OD7"]
    #[inline(always)]
    pub fn od7(&self) -> OD7_R {
        OD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OD8"]
    #[inline(always)]
    pub fn od8(&self) -> OD8_R {
        OD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OD9"]
    #[inline(always)]
    pub fn od9(&self) -> OD9_R {
        OD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OD10"]
    #[inline(always)]
    pub fn od10(&self) -> OD10_R {
        OD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OD11"]
    #[inline(always)]
    pub fn od11(&self) -> OD11_R {
        OD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OD12"]
    #[inline(always)]
    pub fn od12(&self) -> OD12_R {
        OD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - OD13"]
    #[inline(always)]
    pub fn od13(&self) -> OD13_R {
        OD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - OD14"]
    #[inline(always)]
    pub fn od14(&self) -> OD14_R {
        OD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - OD15"]
    #[inline(always)]
    pub fn od15(&self) -> OD15_R {
        OD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OD0"]
    #[inline(always)]
    #[must_use]
    pub fn od0(&mut self) -> OD0_W<0> {
        OD0_W::new(self)
    }
    #[doc = "Bit 1 - OD1"]
    #[inline(always)]
    #[must_use]
    pub fn od1(&mut self) -> OD1_W<1> {
        OD1_W::new(self)
    }
    #[doc = "Bit 2 - OD2"]
    #[inline(always)]
    #[must_use]
    pub fn od2(&mut self) -> OD2_W<2> {
        OD2_W::new(self)
    }
    #[doc = "Bit 3 - OD3"]
    #[inline(always)]
    #[must_use]
    pub fn od3(&mut self) -> OD3_W<3> {
        OD3_W::new(self)
    }
    #[doc = "Bit 4 - OD4"]
    #[inline(always)]
    #[must_use]
    pub fn od4(&mut self) -> OD4_W<4> {
        OD4_W::new(self)
    }
    #[doc = "Bit 5 - OD5"]
    #[inline(always)]
    #[must_use]
    pub fn od5(&mut self) -> OD5_W<5> {
        OD5_W::new(self)
    }
    #[doc = "Bit 6 - OD6"]
    #[inline(always)]
    #[must_use]
    pub fn od6(&mut self) -> OD6_W<6> {
        OD6_W::new(self)
    }
    #[doc = "Bit 7 - OD7"]
    #[inline(always)]
    #[must_use]
    pub fn od7(&mut self) -> OD7_W<7> {
        OD7_W::new(self)
    }
    #[doc = "Bit 8 - OD8"]
    #[inline(always)]
    #[must_use]
    pub fn od8(&mut self) -> OD8_W<8> {
        OD8_W::new(self)
    }
    #[doc = "Bit 9 - OD9"]
    #[inline(always)]
    #[must_use]
    pub fn od9(&mut self) -> OD9_W<9> {
        OD9_W::new(self)
    }
    #[doc = "Bit 10 - OD10"]
    #[inline(always)]
    #[must_use]
    pub fn od10(&mut self) -> OD10_W<10> {
        OD10_W::new(self)
    }
    #[doc = "Bit 11 - OD11"]
    #[inline(always)]
    #[must_use]
    pub fn od11(&mut self) -> OD11_W<11> {
        OD11_W::new(self)
    }
    #[doc = "Bit 12 - OD12"]
    #[inline(always)]
    #[must_use]
    pub fn od12(&mut self) -> OD12_W<12> {
        OD12_W::new(self)
    }
    #[doc = "Bit 13 - OD13"]
    #[inline(always)]
    #[must_use]
    pub fn od13(&mut self) -> OD13_W<13> {
        OD13_W::new(self)
    }
    #[doc = "Bit 14 - OD14"]
    #[inline(always)]
    #[must_use]
    pub fn od14(&mut self) -> OD14_W<14> {
        OD14_W::new(self)
    }
    #[doc = "Bit 15 - OD15"]
    #[inline(always)]
    #[must_use]
    pub fn od15(&mut self) -> OD15_W<15> {
        OD15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ODR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odr](index.html) module"]
pub struct ODR_SPEC;
impl crate::RegisterSpec for ODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [odr::R](R) reader structure"]
impl crate::Readable for ODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [odr::W](W) writer structure"]
impl crate::Writable for ODR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ODR to value 0"]
impl crate::Resettable for ODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
