#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISR` writer"]
pub struct W(crate::W<ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR_SPEC>;
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
impl From<crate::W<ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GEISTA0` reader - GEISTA0"]
pub type GEISTA0_R = crate::BitReader<bool>;
#[doc = "Field `GEISTA0` writer - GEISTA0"]
pub type GEISTA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `BEISTA0` reader - BEISTA0"]
pub type BEISTA0_R = crate::BitReader<bool>;
#[doc = "Field `BEISTA0` writer - BEISTA0"]
pub type BEISTA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `HTISTA0` reader - HTISTA0"]
pub type HTISTA0_R = crate::BitReader<bool>;
#[doc = "Field `HTISTA0` writer - HTISTA0"]
pub type HTISTA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `TCISTA0` reader - TCISTA0"]
pub type TCISTA0_R = crate::BitReader<bool>;
#[doc = "Field `TCISTA0` writer - TCISTA0"]
pub type TCISTA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `TEISTA0` reader - TEISTA0"]
pub type TEISTA0_R = crate::BitReader<bool>;
#[doc = "Field `TEISTA0` writer - TEISTA0"]
pub type TEISTA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `GEISTA1` reader - GEISTA1"]
pub type GEISTA1_R = crate::BitReader<bool>;
#[doc = "Field `GEISTA1` writer - GEISTA1"]
pub type GEISTA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `BEISTA1` reader - BEISTA1"]
pub type BEISTA1_R = crate::BitReader<bool>;
#[doc = "Field `BEISTA1` writer - BEISTA1"]
pub type BEISTA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `HTISTA1` reader - HTISTA1"]
pub type HTISTA1_R = crate::BitReader<bool>;
#[doc = "Field `HTISTA1` writer - HTISTA1"]
pub type HTISTA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `TCISTA1` reader - TCISTA1"]
pub type TCISTA1_R = crate::BitReader<bool>;
#[doc = "Field `TCISTA1` writer - TCISTA1"]
pub type TCISTA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `TEISTA1` reader - TEISTA1"]
pub type TEISTA1_R = crate::BitReader<bool>;
#[doc = "Field `TEISTA1` writer - TEISTA1"]
pub type TEISTA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `GEISTA2` reader - GEISTA2"]
pub type GEISTA2_R = crate::BitReader<bool>;
#[doc = "Field `GEISTA2` writer - GEISTA2"]
pub type GEISTA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `BEISTA2` reader - BEISTA2"]
pub type BEISTA2_R = crate::BitReader<bool>;
#[doc = "Field `BEISTA2` writer - BEISTA2"]
pub type BEISTA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `HTISTA2` reader - HTISTA2"]
pub type HTISTA2_R = crate::BitReader<bool>;
#[doc = "Field `HTISTA2` writer - HTISTA2"]
pub type HTISTA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `TCISTA2` reader - TCISTA2"]
pub type TCISTA2_R = crate::BitReader<bool>;
#[doc = "Field `TCISTA2` writer - TCISTA2"]
pub type TCISTA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `TEISTA2` reader - TEISTA2"]
pub type TEISTA2_R = crate::BitReader<bool>;
#[doc = "Field `TEISTA2` writer - TEISTA2"]
pub type TEISTA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `GEISTA3` reader - GEISTA3"]
pub type GEISTA3_R = crate::BitReader<bool>;
#[doc = "Field `GEISTA3` writer - GEISTA3"]
pub type GEISTA3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `BEISTA3` reader - BEISTA3"]
pub type BEISTA3_R = crate::BitReader<bool>;
#[doc = "Field `BEISTA3` writer - BEISTA3"]
pub type BEISTA3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `HTISTA3` reader - HTISTA3"]
pub type HTISTA3_R = crate::BitReader<bool>;
#[doc = "Field `HTISTA3` writer - HTISTA3"]
pub type HTISTA3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `TCISTA3` reader - TCISTA3"]
pub type TCISTA3_R = crate::BitReader<bool>;
#[doc = "Field `TCISTA3` writer - TCISTA3"]
pub type TCISTA3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `TEISTA3` reader - TEISTA3"]
pub type TEISTA3_R = crate::BitReader<bool>;
#[doc = "Field `TEISTA3` writer - TEISTA3"]
pub type TEISTA3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `GEISTA4` reader - GEISTA4"]
pub type GEISTA4_R = crate::BitReader<bool>;
#[doc = "Field `GEISTA4` writer - GEISTA4"]
pub type GEISTA4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `BEISTA4` reader - BEISTA4"]
pub type BEISTA4_R = crate::BitReader<bool>;
#[doc = "Field `BEISTA4` writer - BEISTA4"]
pub type BEISTA4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `HTISTA4` reader - HTISTA4"]
pub type HTISTA4_R = crate::BitReader<bool>;
#[doc = "Field `HTISTA4` writer - HTISTA4"]
pub type HTISTA4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `TCISTA4` reader - TCISTA4"]
pub type TCISTA4_R = crate::BitReader<bool>;
#[doc = "Field `TCISTA4` writer - TCISTA4"]
pub type TCISTA4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `TEISTA4` reader - TEISTA4"]
pub type TEISTA4_R = crate::BitReader<bool>;
#[doc = "Field `TEISTA4` writer - TEISTA4"]
pub type TEISTA4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `GEISTA5` reader - GEISTA5"]
pub type GEISTA5_R = crate::BitReader<bool>;
#[doc = "Field `GEISTA5` writer - GEISTA5"]
pub type GEISTA5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `BEISTA5` reader - BEISTA5"]
pub type BEISTA5_R = crate::BitReader<bool>;
#[doc = "Field `BEISTA5` writer - BEISTA5"]
pub type BEISTA5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `HTISTA5` reader - HTISTA5"]
pub type HTISTA5_R = crate::BitReader<bool>;
#[doc = "Field `HTISTA5` writer - HTISTA5"]
pub type HTISTA5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `TCISTA5` reader - TCISTA5"]
pub type TCISTA5_R = crate::BitReader<bool>;
#[doc = "Field `TCISTA5` writer - TCISTA5"]
pub type TCISTA5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `TEISTA5` reader - TEISTA5"]
pub type TEISTA5_R = crate::BitReader<bool>;
#[doc = "Field `TEISTA5` writer - TEISTA5"]
pub type TEISTA5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GEISTA0"]
    #[inline(always)]
    pub fn geista0(&self) -> GEISTA0_R {
        GEISTA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BEISTA0"]
    #[inline(always)]
    pub fn beista0(&self) -> BEISTA0_R {
        BEISTA0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HTISTA0"]
    #[inline(always)]
    pub fn htista0(&self) -> HTISTA0_R {
        HTISTA0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TCISTA0"]
    #[inline(always)]
    pub fn tcista0(&self) -> TCISTA0_R {
        TCISTA0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TEISTA0"]
    #[inline(always)]
    pub fn teista0(&self) -> TEISTA0_R {
        TEISTA0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GEISTA1"]
    #[inline(always)]
    pub fn geista1(&self) -> GEISTA1_R {
        GEISTA1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BEISTA1"]
    #[inline(always)]
    pub fn beista1(&self) -> BEISTA1_R {
        BEISTA1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HTISTA1"]
    #[inline(always)]
    pub fn htista1(&self) -> HTISTA1_R {
        HTISTA1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TCISTA1"]
    #[inline(always)]
    pub fn tcista1(&self) -> TCISTA1_R {
        TCISTA1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TEISTA1"]
    #[inline(always)]
    pub fn teista1(&self) -> TEISTA1_R {
        TEISTA1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GEISTA2"]
    #[inline(always)]
    pub fn geista2(&self) -> GEISTA2_R {
        GEISTA2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BEISTA2"]
    #[inline(always)]
    pub fn beista2(&self) -> BEISTA2_R {
        BEISTA2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - HTISTA2"]
    #[inline(always)]
    pub fn htista2(&self) -> HTISTA2_R {
        HTISTA2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TCISTA2"]
    #[inline(always)]
    pub fn tcista2(&self) -> TCISTA2_R {
        TCISTA2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TEISTA2"]
    #[inline(always)]
    pub fn teista2(&self) -> TEISTA2_R {
        TEISTA2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GEISTA3"]
    #[inline(always)]
    pub fn geista3(&self) -> GEISTA3_R {
        GEISTA3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - BEISTA3"]
    #[inline(always)]
    pub fn beista3(&self) -> BEISTA3_R {
        BEISTA3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HTISTA3"]
    #[inline(always)]
    pub fn htista3(&self) -> HTISTA3_R {
        HTISTA3_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TCISTA3"]
    #[inline(always)]
    pub fn tcista3(&self) -> TCISTA3_R {
        TCISTA3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TEISTA3"]
    #[inline(always)]
    pub fn teista3(&self) -> TEISTA3_R {
        TEISTA3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GEISTA4"]
    #[inline(always)]
    pub fn geista4(&self) -> GEISTA4_R {
        GEISTA4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - BEISTA4"]
    #[inline(always)]
    pub fn beista4(&self) -> BEISTA4_R {
        BEISTA4_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - HTISTA4"]
    #[inline(always)]
    pub fn htista4(&self) -> HTISTA4_R {
        HTISTA4_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TCISTA4"]
    #[inline(always)]
    pub fn tcista4(&self) -> TCISTA4_R {
        TCISTA4_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TEISTA4"]
    #[inline(always)]
    pub fn teista4(&self) -> TEISTA4_R {
        TEISTA4_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GEISTA5"]
    #[inline(always)]
    pub fn geista5(&self) -> GEISTA5_R {
        GEISTA5_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - BEISTA5"]
    #[inline(always)]
    pub fn beista5(&self) -> BEISTA5_R {
        BEISTA5_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - HTISTA5"]
    #[inline(always)]
    pub fn htista5(&self) -> HTISTA5_R {
        HTISTA5_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - TCISTA5"]
    #[inline(always)]
    pub fn tcista5(&self) -> TCISTA5_R {
        TCISTA5_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TEISTA5"]
    #[inline(always)]
    pub fn teista5(&self) -> TEISTA5_R {
        TEISTA5_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GEISTA0"]
    #[inline(always)]
    #[must_use]
    pub fn geista0(&mut self) -> GEISTA0_W<0> {
        GEISTA0_W::new(self)
    }
    #[doc = "Bit 1 - BEISTA0"]
    #[inline(always)]
    #[must_use]
    pub fn beista0(&mut self) -> BEISTA0_W<1> {
        BEISTA0_W::new(self)
    }
    #[doc = "Bit 2 - HTISTA0"]
    #[inline(always)]
    #[must_use]
    pub fn htista0(&mut self) -> HTISTA0_W<2> {
        HTISTA0_W::new(self)
    }
    #[doc = "Bit 3 - TCISTA0"]
    #[inline(always)]
    #[must_use]
    pub fn tcista0(&mut self) -> TCISTA0_W<3> {
        TCISTA0_W::new(self)
    }
    #[doc = "Bit 4 - TEISTA0"]
    #[inline(always)]
    #[must_use]
    pub fn teista0(&mut self) -> TEISTA0_W<4> {
        TEISTA0_W::new(self)
    }
    #[doc = "Bit 5 - GEISTA1"]
    #[inline(always)]
    #[must_use]
    pub fn geista1(&mut self) -> GEISTA1_W<5> {
        GEISTA1_W::new(self)
    }
    #[doc = "Bit 6 - BEISTA1"]
    #[inline(always)]
    #[must_use]
    pub fn beista1(&mut self) -> BEISTA1_W<6> {
        BEISTA1_W::new(self)
    }
    #[doc = "Bit 7 - HTISTA1"]
    #[inline(always)]
    #[must_use]
    pub fn htista1(&mut self) -> HTISTA1_W<7> {
        HTISTA1_W::new(self)
    }
    #[doc = "Bit 8 - TCISTA1"]
    #[inline(always)]
    #[must_use]
    pub fn tcista1(&mut self) -> TCISTA1_W<8> {
        TCISTA1_W::new(self)
    }
    #[doc = "Bit 9 - TEISTA1"]
    #[inline(always)]
    #[must_use]
    pub fn teista1(&mut self) -> TEISTA1_W<9> {
        TEISTA1_W::new(self)
    }
    #[doc = "Bit 10 - GEISTA2"]
    #[inline(always)]
    #[must_use]
    pub fn geista2(&mut self) -> GEISTA2_W<10> {
        GEISTA2_W::new(self)
    }
    #[doc = "Bit 11 - BEISTA2"]
    #[inline(always)]
    #[must_use]
    pub fn beista2(&mut self) -> BEISTA2_W<11> {
        BEISTA2_W::new(self)
    }
    #[doc = "Bit 12 - HTISTA2"]
    #[inline(always)]
    #[must_use]
    pub fn htista2(&mut self) -> HTISTA2_W<12> {
        HTISTA2_W::new(self)
    }
    #[doc = "Bit 13 - TCISTA2"]
    #[inline(always)]
    #[must_use]
    pub fn tcista2(&mut self) -> TCISTA2_W<13> {
        TCISTA2_W::new(self)
    }
    #[doc = "Bit 14 - TEISTA2"]
    #[inline(always)]
    #[must_use]
    pub fn teista2(&mut self) -> TEISTA2_W<14> {
        TEISTA2_W::new(self)
    }
    #[doc = "Bit 15 - GEISTA3"]
    #[inline(always)]
    #[must_use]
    pub fn geista3(&mut self) -> GEISTA3_W<15> {
        GEISTA3_W::new(self)
    }
    #[doc = "Bit 16 - BEISTA3"]
    #[inline(always)]
    #[must_use]
    pub fn beista3(&mut self) -> BEISTA3_W<16> {
        BEISTA3_W::new(self)
    }
    #[doc = "Bit 17 - HTISTA3"]
    #[inline(always)]
    #[must_use]
    pub fn htista3(&mut self) -> HTISTA3_W<17> {
        HTISTA3_W::new(self)
    }
    #[doc = "Bit 18 - TCISTA3"]
    #[inline(always)]
    #[must_use]
    pub fn tcista3(&mut self) -> TCISTA3_W<18> {
        TCISTA3_W::new(self)
    }
    #[doc = "Bit 19 - TEISTA3"]
    #[inline(always)]
    #[must_use]
    pub fn teista3(&mut self) -> TEISTA3_W<19> {
        TEISTA3_W::new(self)
    }
    #[doc = "Bit 20 - GEISTA4"]
    #[inline(always)]
    #[must_use]
    pub fn geista4(&mut self) -> GEISTA4_W<20> {
        GEISTA4_W::new(self)
    }
    #[doc = "Bit 21 - BEISTA4"]
    #[inline(always)]
    #[must_use]
    pub fn beista4(&mut self) -> BEISTA4_W<21> {
        BEISTA4_W::new(self)
    }
    #[doc = "Bit 22 - HTISTA4"]
    #[inline(always)]
    #[must_use]
    pub fn htista4(&mut self) -> HTISTA4_W<22> {
        HTISTA4_W::new(self)
    }
    #[doc = "Bit 23 - TCISTA4"]
    #[inline(always)]
    #[must_use]
    pub fn tcista4(&mut self) -> TCISTA4_W<23> {
        TCISTA4_W::new(self)
    }
    #[doc = "Bit 24 - TEISTA4"]
    #[inline(always)]
    #[must_use]
    pub fn teista4(&mut self) -> TEISTA4_W<24> {
        TEISTA4_W::new(self)
    }
    #[doc = "Bit 25 - GEISTA5"]
    #[inline(always)]
    #[must_use]
    pub fn geista5(&mut self) -> GEISTA5_W<25> {
        GEISTA5_W::new(self)
    }
    #[doc = "Bit 26 - BEISTA5"]
    #[inline(always)]
    #[must_use]
    pub fn beista5(&mut self) -> BEISTA5_W<26> {
        BEISTA5_W::new(self)
    }
    #[doc = "Bit 27 - HTISTA5"]
    #[inline(always)]
    #[must_use]
    pub fn htista5(&mut self) -> HTISTA5_W<27> {
        HTISTA5_W::new(self)
    }
    #[doc = "Bit 28 - TCISTA5"]
    #[inline(always)]
    #[must_use]
    pub fn tcista5(&mut self) -> TCISTA5_W<28> {
        TCISTA5_W::new(self)
    }
    #[doc = "Bit 29 - TEISTA5"]
    #[inline(always)]
    #[must_use]
    pub fn teista5(&mut self) -> TEISTA5_W<29> {
        TEISTA5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isr::W](W) writer structure"]
impl crate::Writable for ISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
