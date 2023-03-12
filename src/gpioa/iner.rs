#[doc = "Register `INER` reader"]
pub struct R(crate::R<INER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INER` writer"]
pub struct W(crate::W<INER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INER_SPEC>;
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
impl From<crate::W<INER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INEN0` reader - INEN0"]
pub type INEN0_R = crate::BitReader<bool>;
#[doc = "Field `INEN0` writer - INEN0"]
pub type INEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INER_SPEC, bool, O>;
#[doc = "Field `INEN1` reader - INEN1"]
pub type INEN1_R = crate::BitReader<bool>;
#[doc = "Field `INEN1` writer - INEN1"]
pub type INEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INER_SPEC, bool, O>;
#[doc = "Field `INEN2` reader - INEN2"]
pub type INEN2_R = crate::BitReader<bool>;
#[doc = "Field `INEN2` writer - INEN2"]
pub type INEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INER_SPEC, bool, O>;
#[doc = "Field `INEN3` reader - INEN3"]
pub type INEN3_R = crate::BitReader<bool>;
#[doc = "Field `INEN3` writer - INEN3"]
pub type INEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, INER_SPEC, bool, O>;
#[doc = "Field `INEN4` reader - INEN4"]
pub type INEN4_R = crate::BitReader<bool>;
#[doc = "Field `INEN4` writer - INEN4"]
pub type INEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, INER_SPEC, bool, O>;
#[doc = "Field `INEN5` reader - INEN5"]
pub type INEN5_R = crate::BitReader<bool>;
#[doc = "Field `INEN5` writer - INEN5"]
pub type INEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, INER_SPEC, bool, O>;
#[doc = "Field `INEN6` reader - INEN6"]
pub type INEN6_R = crate::BitReader<bool>;
#[doc = "Field `INEN6` writer - INEN6"]
pub type INEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, INER_SPEC, bool, O>;
#[doc = "Field `INEN7` reader - INEN7"]
pub type INEN7_R = crate::BitReader<bool>;
#[doc = "Field `INEN7` writer - INEN7"]
pub type INEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, INER_SPEC, bool, O>;
#[doc = "Field `INEN8` reader - INEN8"]
pub type INEN8_R = crate::BitReader<bool>;
#[doc = "Field `INEN8` writer - INEN8"]
pub type INEN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, INER_SPEC, bool, O>;
#[doc = "Field `INEN9` reader - INEN9"]
pub type INEN9_R = crate::BitReader<bool>;
#[doc = "Field `INEN9` writer - INEN9"]
pub type INEN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, INER_SPEC, bool, O>;
#[doc = "Field `INEN10` reader - INEN10"]
pub type INEN10_R = crate::BitReader<bool>;
#[doc = "Field `INEN10` writer - INEN10"]
pub type INEN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, INER_SPEC, bool, O>;
#[doc = "Field `INEN11` reader - INEN11"]
pub type INEN11_R = crate::BitReader<bool>;
#[doc = "Field `INEN11` writer - INEN11"]
pub type INEN11_W<'a, const O: u8> = crate::BitWriter<'a, u32, INER_SPEC, bool, O>;
#[doc = "Field `INEN12` reader - INEN12"]
pub type INEN12_R = crate::BitReader<bool>;
#[doc = "Field `INEN12` writer - INEN12"]
pub type INEN12_W<'a, const O: u8> = crate::BitWriter<'a, u32, INER_SPEC, bool, O>;
#[doc = "Field `INEN13` reader - INEN13"]
pub type INEN13_R = crate::BitReader<bool>;
#[doc = "Field `INEN13` writer - INEN13"]
pub type INEN13_W<'a, const O: u8> = crate::BitWriter<'a, u32, INER_SPEC, bool, O>;
#[doc = "Field `INEN14` reader - INEN14"]
pub type INEN14_R = crate::BitReader<bool>;
#[doc = "Field `INEN14` writer - INEN14"]
pub type INEN14_W<'a, const O: u8> = crate::BitWriter<'a, u32, INER_SPEC, bool, O>;
#[doc = "Field `INEN15` reader - INEN15"]
pub type INEN15_R = crate::BitReader<bool>;
#[doc = "Field `INEN15` writer - INEN15"]
pub type INEN15_W<'a, const O: u8> = crate::BitWriter<'a, u32, INER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - INEN0"]
    #[inline(always)]
    pub fn inen0(&self) -> INEN0_R {
        INEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - INEN1"]
    #[inline(always)]
    pub fn inen1(&self) -> INEN1_R {
        INEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - INEN2"]
    #[inline(always)]
    pub fn inen2(&self) -> INEN2_R {
        INEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - INEN3"]
    #[inline(always)]
    pub fn inen3(&self) -> INEN3_R {
        INEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - INEN4"]
    #[inline(always)]
    pub fn inen4(&self) -> INEN4_R {
        INEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - INEN5"]
    #[inline(always)]
    pub fn inen5(&self) -> INEN5_R {
        INEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - INEN6"]
    #[inline(always)]
    pub fn inen6(&self) -> INEN6_R {
        INEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - INEN7"]
    #[inline(always)]
    pub fn inen7(&self) -> INEN7_R {
        INEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - INEN8"]
    #[inline(always)]
    pub fn inen8(&self) -> INEN8_R {
        INEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - INEN9"]
    #[inline(always)]
    pub fn inen9(&self) -> INEN9_R {
        INEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - INEN10"]
    #[inline(always)]
    pub fn inen10(&self) -> INEN10_R {
        INEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - INEN11"]
    #[inline(always)]
    pub fn inen11(&self) -> INEN11_R {
        INEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - INEN12"]
    #[inline(always)]
    pub fn inen12(&self) -> INEN12_R {
        INEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - INEN13"]
    #[inline(always)]
    pub fn inen13(&self) -> INEN13_R {
        INEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - INEN14"]
    #[inline(always)]
    pub fn inen14(&self) -> INEN14_R {
        INEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - INEN15"]
    #[inline(always)]
    pub fn inen15(&self) -> INEN15_R {
        INEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INEN0"]
    #[inline(always)]
    #[must_use]
    pub fn inen0(&mut self) -> INEN0_W<0> {
        INEN0_W::new(self)
    }
    #[doc = "Bit 1 - INEN1"]
    #[inline(always)]
    #[must_use]
    pub fn inen1(&mut self) -> INEN1_W<1> {
        INEN1_W::new(self)
    }
    #[doc = "Bit 2 - INEN2"]
    #[inline(always)]
    #[must_use]
    pub fn inen2(&mut self) -> INEN2_W<2> {
        INEN2_W::new(self)
    }
    #[doc = "Bit 3 - INEN3"]
    #[inline(always)]
    #[must_use]
    pub fn inen3(&mut self) -> INEN3_W<3> {
        INEN3_W::new(self)
    }
    #[doc = "Bit 4 - INEN4"]
    #[inline(always)]
    #[must_use]
    pub fn inen4(&mut self) -> INEN4_W<4> {
        INEN4_W::new(self)
    }
    #[doc = "Bit 5 - INEN5"]
    #[inline(always)]
    #[must_use]
    pub fn inen5(&mut self) -> INEN5_W<5> {
        INEN5_W::new(self)
    }
    #[doc = "Bit 6 - INEN6"]
    #[inline(always)]
    #[must_use]
    pub fn inen6(&mut self) -> INEN6_W<6> {
        INEN6_W::new(self)
    }
    #[doc = "Bit 7 - INEN7"]
    #[inline(always)]
    #[must_use]
    pub fn inen7(&mut self) -> INEN7_W<7> {
        INEN7_W::new(self)
    }
    #[doc = "Bit 8 - INEN8"]
    #[inline(always)]
    #[must_use]
    pub fn inen8(&mut self) -> INEN8_W<8> {
        INEN8_W::new(self)
    }
    #[doc = "Bit 9 - INEN9"]
    #[inline(always)]
    #[must_use]
    pub fn inen9(&mut self) -> INEN9_W<9> {
        INEN9_W::new(self)
    }
    #[doc = "Bit 10 - INEN10"]
    #[inline(always)]
    #[must_use]
    pub fn inen10(&mut self) -> INEN10_W<10> {
        INEN10_W::new(self)
    }
    #[doc = "Bit 11 - INEN11"]
    #[inline(always)]
    #[must_use]
    pub fn inen11(&mut self) -> INEN11_W<11> {
        INEN11_W::new(self)
    }
    #[doc = "Bit 12 - INEN12"]
    #[inline(always)]
    #[must_use]
    pub fn inen12(&mut self) -> INEN12_W<12> {
        INEN12_W::new(self)
    }
    #[doc = "Bit 13 - INEN13"]
    #[inline(always)]
    #[must_use]
    pub fn inen13(&mut self) -> INEN13_W<13> {
        INEN13_W::new(self)
    }
    #[doc = "Bit 14 - INEN14"]
    #[inline(always)]
    #[must_use]
    pub fn inen14(&mut self) -> INEN14_W<14> {
        INEN14_W::new(self)
    }
    #[doc = "Bit 15 - INEN15"]
    #[inline(always)]
    #[must_use]
    pub fn inen15(&mut self) -> INEN15_W<15> {
        INEN15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iner](index.html) module"]
pub struct INER_SPEC;
impl crate::RegisterSpec for INER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iner::R](R) reader structure"]
impl crate::Readable for INER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iner::W](W) writer structure"]
impl crate::Writable for INER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INER to value 0"]
impl crate::Resettable for INER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
