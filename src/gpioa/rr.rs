#[doc = "Register `RR` reader"]
pub struct R(crate::R<RR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RR` writer"]
pub struct W(crate::W<RR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RR_SPEC>;
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
impl From<crate::W<RR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RST0` reader - RST0"]
pub type RST0_R = crate::BitReader<bool>;
#[doc = "Field `RST0` writer - RST0"]
pub type RST0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RR_SPEC, bool, O>;
#[doc = "Field `RST1` reader - RST1"]
pub type RST1_R = crate::BitReader<bool>;
#[doc = "Field `RST1` writer - RST1"]
pub type RST1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RR_SPEC, bool, O>;
#[doc = "Field `RST2` reader - RST2"]
pub type RST2_R = crate::BitReader<bool>;
#[doc = "Field `RST2` writer - RST2"]
pub type RST2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RR_SPEC, bool, O>;
#[doc = "Field `RST3` reader - RST3"]
pub type RST3_R = crate::BitReader<bool>;
#[doc = "Field `RST3` writer - RST3"]
pub type RST3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RR_SPEC, bool, O>;
#[doc = "Field `RST4` reader - RST4"]
pub type RST4_R = crate::BitReader<bool>;
#[doc = "Field `RST4` writer - RST4"]
pub type RST4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RR_SPEC, bool, O>;
#[doc = "Field `RST5` reader - RST5"]
pub type RST5_R = crate::BitReader<bool>;
#[doc = "Field `RST5` writer - RST5"]
pub type RST5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RR_SPEC, bool, O>;
#[doc = "Field `RST6` reader - RST6"]
pub type RST6_R = crate::BitReader<bool>;
#[doc = "Field `RST6` writer - RST6"]
pub type RST6_W<'a, const O: u8> = crate::BitWriter<'a, u32, RR_SPEC, bool, O>;
#[doc = "Field `RST7` reader - RST7"]
pub type RST7_R = crate::BitReader<bool>;
#[doc = "Field `RST7` writer - RST7"]
pub type RST7_W<'a, const O: u8> = crate::BitWriter<'a, u32, RR_SPEC, bool, O>;
#[doc = "Field `RST8` reader - RST8"]
pub type RST8_R = crate::BitReader<bool>;
#[doc = "Field `RST8` writer - RST8"]
pub type RST8_W<'a, const O: u8> = crate::BitWriter<'a, u32, RR_SPEC, bool, O>;
#[doc = "Field `RST9` reader - RST9"]
pub type RST9_R = crate::BitReader<bool>;
#[doc = "Field `RST9` writer - RST9"]
pub type RST9_W<'a, const O: u8> = crate::BitWriter<'a, u32, RR_SPEC, bool, O>;
#[doc = "Field `RST10` reader - RST10"]
pub type RST10_R = crate::BitReader<bool>;
#[doc = "Field `RST10` writer - RST10"]
pub type RST10_W<'a, const O: u8> = crate::BitWriter<'a, u32, RR_SPEC, bool, O>;
#[doc = "Field `RST11` reader - RST11"]
pub type RST11_R = crate::BitReader<bool>;
#[doc = "Field `RST11` writer - RST11"]
pub type RST11_W<'a, const O: u8> = crate::BitWriter<'a, u32, RR_SPEC, bool, O>;
#[doc = "Field `RST12` reader - RST12"]
pub type RST12_R = crate::BitReader<bool>;
#[doc = "Field `RST12` writer - RST12"]
pub type RST12_W<'a, const O: u8> = crate::BitWriter<'a, u32, RR_SPEC, bool, O>;
#[doc = "Field `RST13` reader - RST13"]
pub type RST13_R = crate::BitReader<bool>;
#[doc = "Field `RST13` writer - RST13"]
pub type RST13_W<'a, const O: u8> = crate::BitWriter<'a, u32, RR_SPEC, bool, O>;
#[doc = "Field `RST14` reader - RST14"]
pub type RST14_R = crate::BitReader<bool>;
#[doc = "Field `RST14` writer - RST14"]
pub type RST14_W<'a, const O: u8> = crate::BitWriter<'a, u32, RR_SPEC, bool, O>;
#[doc = "Field `RST15` reader - RST15"]
pub type RST15_R = crate::BitReader<bool>;
#[doc = "Field `RST15` writer - RST15"]
pub type RST15_W<'a, const O: u8> = crate::BitWriter<'a, u32, RR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RST0"]
    #[inline(always)]
    pub fn rst0(&self) -> RST0_R {
        RST0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RST1"]
    #[inline(always)]
    pub fn rst1(&self) -> RST1_R {
        RST1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RST2"]
    #[inline(always)]
    pub fn rst2(&self) -> RST2_R {
        RST2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RST3"]
    #[inline(always)]
    pub fn rst3(&self) -> RST3_R {
        RST3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RST4"]
    #[inline(always)]
    pub fn rst4(&self) -> RST4_R {
        RST4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RST5"]
    #[inline(always)]
    pub fn rst5(&self) -> RST5_R {
        RST5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RST6"]
    #[inline(always)]
    pub fn rst6(&self) -> RST6_R {
        RST6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RST7"]
    #[inline(always)]
    pub fn rst7(&self) -> RST7_R {
        RST7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RST8"]
    #[inline(always)]
    pub fn rst8(&self) -> RST8_R {
        RST8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RST9"]
    #[inline(always)]
    pub fn rst9(&self) -> RST9_R {
        RST9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RST10"]
    #[inline(always)]
    pub fn rst10(&self) -> RST10_R {
        RST10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RST11"]
    #[inline(always)]
    pub fn rst11(&self) -> RST11_R {
        RST11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RST12"]
    #[inline(always)]
    pub fn rst12(&self) -> RST12_R {
        RST12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RST13"]
    #[inline(always)]
    pub fn rst13(&self) -> RST13_R {
        RST13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RST14"]
    #[inline(always)]
    pub fn rst14(&self) -> RST14_R {
        RST14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RST15"]
    #[inline(always)]
    pub fn rst15(&self) -> RST15_R {
        RST15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RST0"]
    #[inline(always)]
    #[must_use]
    pub fn rst0(&mut self) -> RST0_W<0> {
        RST0_W::new(self)
    }
    #[doc = "Bit 1 - RST1"]
    #[inline(always)]
    #[must_use]
    pub fn rst1(&mut self) -> RST1_W<1> {
        RST1_W::new(self)
    }
    #[doc = "Bit 2 - RST2"]
    #[inline(always)]
    #[must_use]
    pub fn rst2(&mut self) -> RST2_W<2> {
        RST2_W::new(self)
    }
    #[doc = "Bit 3 - RST3"]
    #[inline(always)]
    #[must_use]
    pub fn rst3(&mut self) -> RST3_W<3> {
        RST3_W::new(self)
    }
    #[doc = "Bit 4 - RST4"]
    #[inline(always)]
    #[must_use]
    pub fn rst4(&mut self) -> RST4_W<4> {
        RST4_W::new(self)
    }
    #[doc = "Bit 5 - RST5"]
    #[inline(always)]
    #[must_use]
    pub fn rst5(&mut self) -> RST5_W<5> {
        RST5_W::new(self)
    }
    #[doc = "Bit 6 - RST6"]
    #[inline(always)]
    #[must_use]
    pub fn rst6(&mut self) -> RST6_W<6> {
        RST6_W::new(self)
    }
    #[doc = "Bit 7 - RST7"]
    #[inline(always)]
    #[must_use]
    pub fn rst7(&mut self) -> RST7_W<7> {
        RST7_W::new(self)
    }
    #[doc = "Bit 8 - RST8"]
    #[inline(always)]
    #[must_use]
    pub fn rst8(&mut self) -> RST8_W<8> {
        RST8_W::new(self)
    }
    #[doc = "Bit 9 - RST9"]
    #[inline(always)]
    #[must_use]
    pub fn rst9(&mut self) -> RST9_W<9> {
        RST9_W::new(self)
    }
    #[doc = "Bit 10 - RST10"]
    #[inline(always)]
    #[must_use]
    pub fn rst10(&mut self) -> RST10_W<10> {
        RST10_W::new(self)
    }
    #[doc = "Bit 11 - RST11"]
    #[inline(always)]
    #[must_use]
    pub fn rst11(&mut self) -> RST11_W<11> {
        RST11_W::new(self)
    }
    #[doc = "Bit 12 - RST12"]
    #[inline(always)]
    #[must_use]
    pub fn rst12(&mut self) -> RST12_W<12> {
        RST12_W::new(self)
    }
    #[doc = "Bit 13 - RST13"]
    #[inline(always)]
    #[must_use]
    pub fn rst13(&mut self) -> RST13_W<13> {
        RST13_W::new(self)
    }
    #[doc = "Bit 14 - RST14"]
    #[inline(always)]
    #[must_use]
    pub fn rst14(&mut self) -> RST14_W<14> {
        RST14_W::new(self)
    }
    #[doc = "Bit 15 - RST15"]
    #[inline(always)]
    #[must_use]
    pub fn rst15(&mut self) -> RST15_W<15> {
        RST15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rr](index.html) module"]
pub struct RR_SPEC;
impl crate::RegisterSpec for RR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rr::R](R) reader structure"]
impl crate::Readable for RR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rr::W](W) writer structure"]
impl crate::Writable for RR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RR to value 0"]
impl crate::Resettable for RR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
