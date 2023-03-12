#[doc = "Register `DOUTR` reader"]
pub struct R(crate::R<DOUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOUTR` writer"]
pub struct W(crate::W<DOUTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUTR_SPEC>;
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
impl From<crate::W<DOUTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUT0` reader - DOUT0"]
pub type DOUT0_R = crate::BitReader<bool>;
#[doc = "Field `DOUT0` writer - DOUT0"]
pub type DOUT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTR_SPEC, bool, O>;
#[doc = "Field `DOUT1` reader - DOUT1"]
pub type DOUT1_R = crate::BitReader<bool>;
#[doc = "Field `DOUT1` writer - DOUT1"]
pub type DOUT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTR_SPEC, bool, O>;
#[doc = "Field `DOUT2` reader - DOUT2"]
pub type DOUT2_R = crate::BitReader<bool>;
#[doc = "Field `DOUT2` writer - DOUT2"]
pub type DOUT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTR_SPEC, bool, O>;
#[doc = "Field `DOUT3` reader - DOUT3"]
pub type DOUT3_R = crate::BitReader<bool>;
#[doc = "Field `DOUT3` writer - DOUT3"]
pub type DOUT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTR_SPEC, bool, O>;
#[doc = "Field `DOUT4` reader - DOUT4"]
pub type DOUT4_R = crate::BitReader<bool>;
#[doc = "Field `DOUT4` writer - DOUT4"]
pub type DOUT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTR_SPEC, bool, O>;
#[doc = "Field `DOUT5` reader - DOUT5"]
pub type DOUT5_R = crate::BitReader<bool>;
#[doc = "Field `DOUT5` writer - DOUT5"]
pub type DOUT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTR_SPEC, bool, O>;
#[doc = "Field `DOUT6` reader - DOUT6"]
pub type DOUT6_R = crate::BitReader<bool>;
#[doc = "Field `DOUT6` writer - DOUT6"]
pub type DOUT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTR_SPEC, bool, O>;
#[doc = "Field `DOUT7` reader - DOUT7"]
pub type DOUT7_R = crate::BitReader<bool>;
#[doc = "Field `DOUT7` writer - DOUT7"]
pub type DOUT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTR_SPEC, bool, O>;
#[doc = "Field `DOUT8` reader - DOUT8"]
pub type DOUT8_R = crate::BitReader<bool>;
#[doc = "Field `DOUT8` writer - DOUT8"]
pub type DOUT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTR_SPEC, bool, O>;
#[doc = "Field `DOUT9` reader - DOUT9"]
pub type DOUT9_R = crate::BitReader<bool>;
#[doc = "Field `DOUT9` writer - DOUT9"]
pub type DOUT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTR_SPEC, bool, O>;
#[doc = "Field `DOUT10` reader - DOUT10"]
pub type DOUT10_R = crate::BitReader<bool>;
#[doc = "Field `DOUT10` writer - DOUT10"]
pub type DOUT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTR_SPEC, bool, O>;
#[doc = "Field `DOUT11` reader - DOUT11"]
pub type DOUT11_R = crate::BitReader<bool>;
#[doc = "Field `DOUT11` writer - DOUT11"]
pub type DOUT11_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTR_SPEC, bool, O>;
#[doc = "Field `DOUT12` reader - DOUT12"]
pub type DOUT12_R = crate::BitReader<bool>;
#[doc = "Field `DOUT12` writer - DOUT12"]
pub type DOUT12_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTR_SPEC, bool, O>;
#[doc = "Field `DOUT13` reader - DOUT13"]
pub type DOUT13_R = crate::BitReader<bool>;
#[doc = "Field `DOUT13` writer - DOUT13"]
pub type DOUT13_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTR_SPEC, bool, O>;
#[doc = "Field `DOUT14` reader - DOUT14"]
pub type DOUT14_R = crate::BitReader<bool>;
#[doc = "Field `DOUT14` writer - DOUT14"]
pub type DOUT14_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTR_SPEC, bool, O>;
#[doc = "Field `DOUT15` reader - DOUT15"]
pub type DOUT15_R = crate::BitReader<bool>;
#[doc = "Field `DOUT15` writer - DOUT15"]
pub type DOUT15_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DOUT0"]
    #[inline(always)]
    pub fn dout0(&self) -> DOUT0_R {
        DOUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DOUT1"]
    #[inline(always)]
    pub fn dout1(&self) -> DOUT1_R {
        DOUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DOUT2"]
    #[inline(always)]
    pub fn dout2(&self) -> DOUT2_R {
        DOUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DOUT3"]
    #[inline(always)]
    pub fn dout3(&self) -> DOUT3_R {
        DOUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DOUT4"]
    #[inline(always)]
    pub fn dout4(&self) -> DOUT4_R {
        DOUT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DOUT5"]
    #[inline(always)]
    pub fn dout5(&self) -> DOUT5_R {
        DOUT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DOUT6"]
    #[inline(always)]
    pub fn dout6(&self) -> DOUT6_R {
        DOUT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DOUT7"]
    #[inline(always)]
    pub fn dout7(&self) -> DOUT7_R {
        DOUT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DOUT8"]
    #[inline(always)]
    pub fn dout8(&self) -> DOUT8_R {
        DOUT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DOUT9"]
    #[inline(always)]
    pub fn dout9(&self) -> DOUT9_R {
        DOUT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DOUT10"]
    #[inline(always)]
    pub fn dout10(&self) -> DOUT10_R {
        DOUT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DOUT11"]
    #[inline(always)]
    pub fn dout11(&self) -> DOUT11_R {
        DOUT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DOUT12"]
    #[inline(always)]
    pub fn dout12(&self) -> DOUT12_R {
        DOUT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DOUT13"]
    #[inline(always)]
    pub fn dout13(&self) -> DOUT13_R {
        DOUT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DOUT14"]
    #[inline(always)]
    pub fn dout14(&self) -> DOUT14_R {
        DOUT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DOUT15"]
    #[inline(always)]
    pub fn dout15(&self) -> DOUT15_R {
        DOUT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DOUT0"]
    #[inline(always)]
    #[must_use]
    pub fn dout0(&mut self) -> DOUT0_W<0> {
        DOUT0_W::new(self)
    }
    #[doc = "Bit 1 - DOUT1"]
    #[inline(always)]
    #[must_use]
    pub fn dout1(&mut self) -> DOUT1_W<1> {
        DOUT1_W::new(self)
    }
    #[doc = "Bit 2 - DOUT2"]
    #[inline(always)]
    #[must_use]
    pub fn dout2(&mut self) -> DOUT2_W<2> {
        DOUT2_W::new(self)
    }
    #[doc = "Bit 3 - DOUT3"]
    #[inline(always)]
    #[must_use]
    pub fn dout3(&mut self) -> DOUT3_W<3> {
        DOUT3_W::new(self)
    }
    #[doc = "Bit 4 - DOUT4"]
    #[inline(always)]
    #[must_use]
    pub fn dout4(&mut self) -> DOUT4_W<4> {
        DOUT4_W::new(self)
    }
    #[doc = "Bit 5 - DOUT5"]
    #[inline(always)]
    #[must_use]
    pub fn dout5(&mut self) -> DOUT5_W<5> {
        DOUT5_W::new(self)
    }
    #[doc = "Bit 6 - DOUT6"]
    #[inline(always)]
    #[must_use]
    pub fn dout6(&mut self) -> DOUT6_W<6> {
        DOUT6_W::new(self)
    }
    #[doc = "Bit 7 - DOUT7"]
    #[inline(always)]
    #[must_use]
    pub fn dout7(&mut self) -> DOUT7_W<7> {
        DOUT7_W::new(self)
    }
    #[doc = "Bit 8 - DOUT8"]
    #[inline(always)]
    #[must_use]
    pub fn dout8(&mut self) -> DOUT8_W<8> {
        DOUT8_W::new(self)
    }
    #[doc = "Bit 9 - DOUT9"]
    #[inline(always)]
    #[must_use]
    pub fn dout9(&mut self) -> DOUT9_W<9> {
        DOUT9_W::new(self)
    }
    #[doc = "Bit 10 - DOUT10"]
    #[inline(always)]
    #[must_use]
    pub fn dout10(&mut self) -> DOUT10_W<10> {
        DOUT10_W::new(self)
    }
    #[doc = "Bit 11 - DOUT11"]
    #[inline(always)]
    #[must_use]
    pub fn dout11(&mut self) -> DOUT11_W<11> {
        DOUT11_W::new(self)
    }
    #[doc = "Bit 12 - DOUT12"]
    #[inline(always)]
    #[must_use]
    pub fn dout12(&mut self) -> DOUT12_W<12> {
        DOUT12_W::new(self)
    }
    #[doc = "Bit 13 - DOUT13"]
    #[inline(always)]
    #[must_use]
    pub fn dout13(&mut self) -> DOUT13_W<13> {
        DOUT13_W::new(self)
    }
    #[doc = "Bit 14 - DOUT14"]
    #[inline(always)]
    #[must_use]
    pub fn dout14(&mut self) -> DOUT14_W<14> {
        DOUT14_W::new(self)
    }
    #[doc = "Bit 15 - DOUT15"]
    #[inline(always)]
    #[must_use]
    pub fn dout15(&mut self) -> DOUT15_W<15> {
        DOUT15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DOUTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr](index.html) module"]
pub struct DOUTR_SPEC;
impl crate::RegisterSpec for DOUTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doutr::R](R) reader structure"]
impl crate::Readable for DOUTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doutr::W](W) writer structure"]
impl crate::Writable for DOUTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOUTR to value 0"]
impl crate::Resettable for DOUTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
