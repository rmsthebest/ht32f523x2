#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GEIE0` reader - GEIE0"]
pub type GEIE0_R = crate::BitReader<bool>;
#[doc = "Field `GEIE0` writer - GEIE0"]
pub type GEIE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `BEIE0` reader - BEIE0"]
pub type BEIE0_R = crate::BitReader<bool>;
#[doc = "Field `BEIE0` writer - BEIE0"]
pub type BEIE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `HTIE0` reader - HTIE0"]
pub type HTIE0_R = crate::BitReader<bool>;
#[doc = "Field `HTIE0` writer - HTIE0"]
pub type HTIE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TCIE0` reader - TCIE0"]
pub type TCIE0_R = crate::BitReader<bool>;
#[doc = "Field `TCIE0` writer - TCIE0"]
pub type TCIE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TEIE0` reader - TEIE0"]
pub type TEIE0_R = crate::BitReader<bool>;
#[doc = "Field `TEIE0` writer - TEIE0"]
pub type TEIE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `GEIE1` reader - GEIE1"]
pub type GEIE1_R = crate::BitReader<bool>;
#[doc = "Field `GEIE1` writer - GEIE1"]
pub type GEIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `BEIE1` reader - BEIE1"]
pub type BEIE1_R = crate::BitReader<bool>;
#[doc = "Field `BEIE1` writer - BEIE1"]
pub type BEIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `HTIE1` reader - HTIE1"]
pub type HTIE1_R = crate::BitReader<bool>;
#[doc = "Field `HTIE1` writer - HTIE1"]
pub type HTIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TCIE1` reader - TCIE1"]
pub type TCIE1_R = crate::BitReader<bool>;
#[doc = "Field `TCIE1` writer - TCIE1"]
pub type TCIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TEIE1` reader - TEIE1"]
pub type TEIE1_R = crate::BitReader<bool>;
#[doc = "Field `TEIE1` writer - TEIE1"]
pub type TEIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `GEIE2` reader - GEIE2"]
pub type GEIE2_R = crate::BitReader<bool>;
#[doc = "Field `GEIE2` writer - GEIE2"]
pub type GEIE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `BEIE2` reader - BEIE2"]
pub type BEIE2_R = crate::BitReader<bool>;
#[doc = "Field `BEIE2` writer - BEIE2"]
pub type BEIE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `HTIE2` reader - HTIE2"]
pub type HTIE2_R = crate::BitReader<bool>;
#[doc = "Field `HTIE2` writer - HTIE2"]
pub type HTIE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TCIE2` reader - TCIE2"]
pub type TCIE2_R = crate::BitReader<bool>;
#[doc = "Field `TCIE2` writer - TCIE2"]
pub type TCIE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TEIE2` reader - TEIE2"]
pub type TEIE2_R = crate::BitReader<bool>;
#[doc = "Field `TEIE2` writer - TEIE2"]
pub type TEIE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `GEIE3` reader - GEIE3"]
pub type GEIE3_R = crate::BitReader<bool>;
#[doc = "Field `GEIE3` writer - GEIE3"]
pub type GEIE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `BEIE3` reader - BEIE3"]
pub type BEIE3_R = crate::BitReader<bool>;
#[doc = "Field `BEIE3` writer - BEIE3"]
pub type BEIE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `HTIE3` reader - HTIE3"]
pub type HTIE3_R = crate::BitReader<bool>;
#[doc = "Field `HTIE3` writer - HTIE3"]
pub type HTIE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TCIE3` reader - TCIE3"]
pub type TCIE3_R = crate::BitReader<bool>;
#[doc = "Field `TCIE3` writer - TCIE3"]
pub type TCIE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TEIE3` reader - TEIE3"]
pub type TEIE3_R = crate::BitReader<bool>;
#[doc = "Field `TEIE3` writer - TEIE3"]
pub type TEIE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `GEIE4` reader - GEIE4"]
pub type GEIE4_R = crate::BitReader<bool>;
#[doc = "Field `GEIE4` writer - GEIE4"]
pub type GEIE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `BEIE4` reader - BEIE4"]
pub type BEIE4_R = crate::BitReader<bool>;
#[doc = "Field `BEIE4` writer - BEIE4"]
pub type BEIE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `HTIE4` reader - HTIE4"]
pub type HTIE4_R = crate::BitReader<bool>;
#[doc = "Field `HTIE4` writer - HTIE4"]
pub type HTIE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TCIE4` reader - TCIE4"]
pub type TCIE4_R = crate::BitReader<bool>;
#[doc = "Field `TCIE4` writer - TCIE4"]
pub type TCIE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TEIE4` reader - TEIE4"]
pub type TEIE4_R = crate::BitReader<bool>;
#[doc = "Field `TEIE4` writer - TEIE4"]
pub type TEIE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `GEIE5` reader - GEIE5"]
pub type GEIE5_R = crate::BitReader<bool>;
#[doc = "Field `GEIE5` writer - GEIE5"]
pub type GEIE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `BEIE5` reader - BEIE5"]
pub type BEIE5_R = crate::BitReader<bool>;
#[doc = "Field `BEIE5` writer - BEIE5"]
pub type BEIE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `HTIE5` reader - HTIE5"]
pub type HTIE5_R = crate::BitReader<bool>;
#[doc = "Field `HTIE5` writer - HTIE5"]
pub type HTIE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TCIE5` reader - TCIE5"]
pub type TCIE5_R = crate::BitReader<bool>;
#[doc = "Field `TCIE5` writer - TCIE5"]
pub type TCIE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TEIE5` reader - TEIE5"]
pub type TEIE5_R = crate::BitReader<bool>;
#[doc = "Field `TEIE5` writer - TEIE5"]
pub type TEIE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GEIE0"]
    #[inline(always)]
    pub fn geie0(&self) -> GEIE0_R {
        GEIE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BEIE0"]
    #[inline(always)]
    pub fn beie0(&self) -> BEIE0_R {
        BEIE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HTIE0"]
    #[inline(always)]
    pub fn htie0(&self) -> HTIE0_R {
        HTIE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TCIE0"]
    #[inline(always)]
    pub fn tcie0(&self) -> TCIE0_R {
        TCIE0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TEIE0"]
    #[inline(always)]
    pub fn teie0(&self) -> TEIE0_R {
        TEIE0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GEIE1"]
    #[inline(always)]
    pub fn geie1(&self) -> GEIE1_R {
        GEIE1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BEIE1"]
    #[inline(always)]
    pub fn beie1(&self) -> BEIE1_R {
        BEIE1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HTIE1"]
    #[inline(always)]
    pub fn htie1(&self) -> HTIE1_R {
        HTIE1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TCIE1"]
    #[inline(always)]
    pub fn tcie1(&self) -> TCIE1_R {
        TCIE1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TEIE1"]
    #[inline(always)]
    pub fn teie1(&self) -> TEIE1_R {
        TEIE1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GEIE2"]
    #[inline(always)]
    pub fn geie2(&self) -> GEIE2_R {
        GEIE2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BEIE2"]
    #[inline(always)]
    pub fn beie2(&self) -> BEIE2_R {
        BEIE2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - HTIE2"]
    #[inline(always)]
    pub fn htie2(&self) -> HTIE2_R {
        HTIE2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TCIE2"]
    #[inline(always)]
    pub fn tcie2(&self) -> TCIE2_R {
        TCIE2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TEIE2"]
    #[inline(always)]
    pub fn teie2(&self) -> TEIE2_R {
        TEIE2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GEIE3"]
    #[inline(always)]
    pub fn geie3(&self) -> GEIE3_R {
        GEIE3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - BEIE3"]
    #[inline(always)]
    pub fn beie3(&self) -> BEIE3_R {
        BEIE3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HTIE3"]
    #[inline(always)]
    pub fn htie3(&self) -> HTIE3_R {
        HTIE3_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TCIE3"]
    #[inline(always)]
    pub fn tcie3(&self) -> TCIE3_R {
        TCIE3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TEIE3"]
    #[inline(always)]
    pub fn teie3(&self) -> TEIE3_R {
        TEIE3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GEIE4"]
    #[inline(always)]
    pub fn geie4(&self) -> GEIE4_R {
        GEIE4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - BEIE4"]
    #[inline(always)]
    pub fn beie4(&self) -> BEIE4_R {
        BEIE4_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - HTIE4"]
    #[inline(always)]
    pub fn htie4(&self) -> HTIE4_R {
        HTIE4_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TCIE4"]
    #[inline(always)]
    pub fn tcie4(&self) -> TCIE4_R {
        TCIE4_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TEIE4"]
    #[inline(always)]
    pub fn teie4(&self) -> TEIE4_R {
        TEIE4_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GEIE5"]
    #[inline(always)]
    pub fn geie5(&self) -> GEIE5_R {
        GEIE5_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - BEIE5"]
    #[inline(always)]
    pub fn beie5(&self) -> BEIE5_R {
        BEIE5_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - HTIE5"]
    #[inline(always)]
    pub fn htie5(&self) -> HTIE5_R {
        HTIE5_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - TCIE5"]
    #[inline(always)]
    pub fn tcie5(&self) -> TCIE5_R {
        TCIE5_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TEIE5"]
    #[inline(always)]
    pub fn teie5(&self) -> TEIE5_R {
        TEIE5_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GEIE0"]
    #[inline(always)]
    #[must_use]
    pub fn geie0(&mut self) -> GEIE0_W<0> {
        GEIE0_W::new(self)
    }
    #[doc = "Bit 1 - BEIE0"]
    #[inline(always)]
    #[must_use]
    pub fn beie0(&mut self) -> BEIE0_W<1> {
        BEIE0_W::new(self)
    }
    #[doc = "Bit 2 - HTIE0"]
    #[inline(always)]
    #[must_use]
    pub fn htie0(&mut self) -> HTIE0_W<2> {
        HTIE0_W::new(self)
    }
    #[doc = "Bit 3 - TCIE0"]
    #[inline(always)]
    #[must_use]
    pub fn tcie0(&mut self) -> TCIE0_W<3> {
        TCIE0_W::new(self)
    }
    #[doc = "Bit 4 - TEIE0"]
    #[inline(always)]
    #[must_use]
    pub fn teie0(&mut self) -> TEIE0_W<4> {
        TEIE0_W::new(self)
    }
    #[doc = "Bit 5 - GEIE1"]
    #[inline(always)]
    #[must_use]
    pub fn geie1(&mut self) -> GEIE1_W<5> {
        GEIE1_W::new(self)
    }
    #[doc = "Bit 6 - BEIE1"]
    #[inline(always)]
    #[must_use]
    pub fn beie1(&mut self) -> BEIE1_W<6> {
        BEIE1_W::new(self)
    }
    #[doc = "Bit 7 - HTIE1"]
    #[inline(always)]
    #[must_use]
    pub fn htie1(&mut self) -> HTIE1_W<7> {
        HTIE1_W::new(self)
    }
    #[doc = "Bit 8 - TCIE1"]
    #[inline(always)]
    #[must_use]
    pub fn tcie1(&mut self) -> TCIE1_W<8> {
        TCIE1_W::new(self)
    }
    #[doc = "Bit 9 - TEIE1"]
    #[inline(always)]
    #[must_use]
    pub fn teie1(&mut self) -> TEIE1_W<9> {
        TEIE1_W::new(self)
    }
    #[doc = "Bit 10 - GEIE2"]
    #[inline(always)]
    #[must_use]
    pub fn geie2(&mut self) -> GEIE2_W<10> {
        GEIE2_W::new(self)
    }
    #[doc = "Bit 11 - BEIE2"]
    #[inline(always)]
    #[must_use]
    pub fn beie2(&mut self) -> BEIE2_W<11> {
        BEIE2_W::new(self)
    }
    #[doc = "Bit 12 - HTIE2"]
    #[inline(always)]
    #[must_use]
    pub fn htie2(&mut self) -> HTIE2_W<12> {
        HTIE2_W::new(self)
    }
    #[doc = "Bit 13 - TCIE2"]
    #[inline(always)]
    #[must_use]
    pub fn tcie2(&mut self) -> TCIE2_W<13> {
        TCIE2_W::new(self)
    }
    #[doc = "Bit 14 - TEIE2"]
    #[inline(always)]
    #[must_use]
    pub fn teie2(&mut self) -> TEIE2_W<14> {
        TEIE2_W::new(self)
    }
    #[doc = "Bit 15 - GEIE3"]
    #[inline(always)]
    #[must_use]
    pub fn geie3(&mut self) -> GEIE3_W<15> {
        GEIE3_W::new(self)
    }
    #[doc = "Bit 16 - BEIE3"]
    #[inline(always)]
    #[must_use]
    pub fn beie3(&mut self) -> BEIE3_W<16> {
        BEIE3_W::new(self)
    }
    #[doc = "Bit 17 - HTIE3"]
    #[inline(always)]
    #[must_use]
    pub fn htie3(&mut self) -> HTIE3_W<17> {
        HTIE3_W::new(self)
    }
    #[doc = "Bit 18 - TCIE3"]
    #[inline(always)]
    #[must_use]
    pub fn tcie3(&mut self) -> TCIE3_W<18> {
        TCIE3_W::new(self)
    }
    #[doc = "Bit 19 - TEIE3"]
    #[inline(always)]
    #[must_use]
    pub fn teie3(&mut self) -> TEIE3_W<19> {
        TEIE3_W::new(self)
    }
    #[doc = "Bit 20 - GEIE4"]
    #[inline(always)]
    #[must_use]
    pub fn geie4(&mut self) -> GEIE4_W<20> {
        GEIE4_W::new(self)
    }
    #[doc = "Bit 21 - BEIE4"]
    #[inline(always)]
    #[must_use]
    pub fn beie4(&mut self) -> BEIE4_W<21> {
        BEIE4_W::new(self)
    }
    #[doc = "Bit 22 - HTIE4"]
    #[inline(always)]
    #[must_use]
    pub fn htie4(&mut self) -> HTIE4_W<22> {
        HTIE4_W::new(self)
    }
    #[doc = "Bit 23 - TCIE4"]
    #[inline(always)]
    #[must_use]
    pub fn tcie4(&mut self) -> TCIE4_W<23> {
        TCIE4_W::new(self)
    }
    #[doc = "Bit 24 - TEIE4"]
    #[inline(always)]
    #[must_use]
    pub fn teie4(&mut self) -> TEIE4_W<24> {
        TEIE4_W::new(self)
    }
    #[doc = "Bit 25 - GEIE5"]
    #[inline(always)]
    #[must_use]
    pub fn geie5(&mut self) -> GEIE5_W<25> {
        GEIE5_W::new(self)
    }
    #[doc = "Bit 26 - BEIE5"]
    #[inline(always)]
    #[must_use]
    pub fn beie5(&mut self) -> BEIE5_W<26> {
        BEIE5_W::new(self)
    }
    #[doc = "Bit 27 - HTIE5"]
    #[inline(always)]
    #[must_use]
    pub fn htie5(&mut self) -> HTIE5_W<27> {
        HTIE5_W::new(self)
    }
    #[doc = "Bit 28 - TCIE5"]
    #[inline(always)]
    #[must_use]
    pub fn tcie5(&mut self) -> TCIE5_W<28> {
        TCIE5_W::new(self)
    }
    #[doc = "Bit 29 - TEIE5"]
    #[inline(always)]
    #[must_use]
    pub fn teie5(&mut self) -> TEIE5_W<29> {
        TEIE5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
