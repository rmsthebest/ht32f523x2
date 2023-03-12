#[doc = "Register `LOCKR` reader"]
pub struct R(crate::R<LOCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCKR` writer"]
pub struct W(crate::W<LOCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCKR_SPEC>;
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
impl From<crate::W<LOCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK0` reader - LOCK0"]
pub type LOCK0_R = crate::BitReader<bool>;
#[doc = "Field `LOCK0` writer - LOCK0"]
pub type LOCK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOCKR_SPEC, bool, O>;
#[doc = "Field `LOCK1` reader - LOCK1"]
pub type LOCK1_R = crate::BitReader<bool>;
#[doc = "Field `LOCK1` writer - LOCK1"]
pub type LOCK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOCKR_SPEC, bool, O>;
#[doc = "Field `LOCK2` reader - LOCK2"]
pub type LOCK2_R = crate::BitReader<bool>;
#[doc = "Field `LOCK2` writer - LOCK2"]
pub type LOCK2_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOCKR_SPEC, bool, O>;
#[doc = "Field `LOCK3` reader - LOCK3"]
pub type LOCK3_R = crate::BitReader<bool>;
#[doc = "Field `LOCK3` writer - LOCK3"]
pub type LOCK3_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOCKR_SPEC, bool, O>;
#[doc = "Field `LOCK4` reader - LOCK4"]
pub type LOCK4_R = crate::BitReader<bool>;
#[doc = "Field `LOCK4` writer - LOCK4"]
pub type LOCK4_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOCKR_SPEC, bool, O>;
#[doc = "Field `LOCK5` reader - LOCK5"]
pub type LOCK5_R = crate::BitReader<bool>;
#[doc = "Field `LOCK5` writer - LOCK5"]
pub type LOCK5_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOCKR_SPEC, bool, O>;
#[doc = "Field `LOCK6` reader - LOCK6"]
pub type LOCK6_R = crate::BitReader<bool>;
#[doc = "Field `LOCK6` writer - LOCK6"]
pub type LOCK6_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOCKR_SPEC, bool, O>;
#[doc = "Field `LOCK7` reader - LOCK7"]
pub type LOCK7_R = crate::BitReader<bool>;
#[doc = "Field `LOCK7` writer - LOCK7"]
pub type LOCK7_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOCKR_SPEC, bool, O>;
#[doc = "Field `LOCK8` reader - LOCK8"]
pub type LOCK8_R = crate::BitReader<bool>;
#[doc = "Field `LOCK8` writer - LOCK8"]
pub type LOCK8_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOCKR_SPEC, bool, O>;
#[doc = "Field `LOCK9` reader - LOCK9"]
pub type LOCK9_R = crate::BitReader<bool>;
#[doc = "Field `LOCK9` writer - LOCK9"]
pub type LOCK9_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOCKR_SPEC, bool, O>;
#[doc = "Field `LOCK10` reader - LOCK10"]
pub type LOCK10_R = crate::BitReader<bool>;
#[doc = "Field `LOCK10` writer - LOCK10"]
pub type LOCK10_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOCKR_SPEC, bool, O>;
#[doc = "Field `LOCK11` reader - LOCK11"]
pub type LOCK11_R = crate::BitReader<bool>;
#[doc = "Field `LOCK11` writer - LOCK11"]
pub type LOCK11_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOCKR_SPEC, bool, O>;
#[doc = "Field `LOCK12` reader - LOCK12"]
pub type LOCK12_R = crate::BitReader<bool>;
#[doc = "Field `LOCK12` writer - LOCK12"]
pub type LOCK12_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOCKR_SPEC, bool, O>;
#[doc = "Field `LOCK13` reader - LOCK13"]
pub type LOCK13_R = crate::BitReader<bool>;
#[doc = "Field `LOCK13` writer - LOCK13"]
pub type LOCK13_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOCKR_SPEC, bool, O>;
#[doc = "Field `LOCK14` reader - LOCK14"]
pub type LOCK14_R = crate::BitReader<bool>;
#[doc = "Field `LOCK14` writer - LOCK14"]
pub type LOCK14_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOCKR_SPEC, bool, O>;
#[doc = "Field `LOCK15` reader - LOCK15"]
pub type LOCK15_R = crate::BitReader<bool>;
#[doc = "Field `LOCK15` writer - LOCK15"]
pub type LOCK15_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOCKR_SPEC, bool, O>;
#[doc = "Field `LKEY` reader - LKEY"]
pub type LKEY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LKEY` writer - LKEY"]
pub type LKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LOCKR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - LOCK0"]
    #[inline(always)]
    pub fn lock0(&self) -> LOCK0_R {
        LOCK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LOCK1"]
    #[inline(always)]
    pub fn lock1(&self) -> LOCK1_R {
        LOCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LOCK2"]
    #[inline(always)]
    pub fn lock2(&self) -> LOCK2_R {
        LOCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LOCK3"]
    #[inline(always)]
    pub fn lock3(&self) -> LOCK3_R {
        LOCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LOCK4"]
    #[inline(always)]
    pub fn lock4(&self) -> LOCK4_R {
        LOCK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LOCK5"]
    #[inline(always)]
    pub fn lock5(&self) -> LOCK5_R {
        LOCK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LOCK6"]
    #[inline(always)]
    pub fn lock6(&self) -> LOCK6_R {
        LOCK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LOCK7"]
    #[inline(always)]
    pub fn lock7(&self) -> LOCK7_R {
        LOCK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LOCK8"]
    #[inline(always)]
    pub fn lock8(&self) -> LOCK8_R {
        LOCK8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LOCK9"]
    #[inline(always)]
    pub fn lock9(&self) -> LOCK9_R {
        LOCK9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LOCK10"]
    #[inline(always)]
    pub fn lock10(&self) -> LOCK10_R {
        LOCK10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LOCK11"]
    #[inline(always)]
    pub fn lock11(&self) -> LOCK11_R {
        LOCK11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LOCK12"]
    #[inline(always)]
    pub fn lock12(&self) -> LOCK12_R {
        LOCK12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LOCK13"]
    #[inline(always)]
    pub fn lock13(&self) -> LOCK13_R {
        LOCK13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LOCK14"]
    #[inline(always)]
    pub fn lock14(&self) -> LOCK14_R {
        LOCK14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LOCK15"]
    #[inline(always)]
    pub fn lock15(&self) -> LOCK15_R {
        LOCK15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - LKEY"]
    #[inline(always)]
    pub fn lkey(&self) -> LKEY_R {
        LKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - LOCK0"]
    #[inline(always)]
    #[must_use]
    pub fn lock0(&mut self) -> LOCK0_W<0> {
        LOCK0_W::new(self)
    }
    #[doc = "Bit 1 - LOCK1"]
    #[inline(always)]
    #[must_use]
    pub fn lock1(&mut self) -> LOCK1_W<1> {
        LOCK1_W::new(self)
    }
    #[doc = "Bit 2 - LOCK2"]
    #[inline(always)]
    #[must_use]
    pub fn lock2(&mut self) -> LOCK2_W<2> {
        LOCK2_W::new(self)
    }
    #[doc = "Bit 3 - LOCK3"]
    #[inline(always)]
    #[must_use]
    pub fn lock3(&mut self) -> LOCK3_W<3> {
        LOCK3_W::new(self)
    }
    #[doc = "Bit 4 - LOCK4"]
    #[inline(always)]
    #[must_use]
    pub fn lock4(&mut self) -> LOCK4_W<4> {
        LOCK4_W::new(self)
    }
    #[doc = "Bit 5 - LOCK5"]
    #[inline(always)]
    #[must_use]
    pub fn lock5(&mut self) -> LOCK5_W<5> {
        LOCK5_W::new(self)
    }
    #[doc = "Bit 6 - LOCK6"]
    #[inline(always)]
    #[must_use]
    pub fn lock6(&mut self) -> LOCK6_W<6> {
        LOCK6_W::new(self)
    }
    #[doc = "Bit 7 - LOCK7"]
    #[inline(always)]
    #[must_use]
    pub fn lock7(&mut self) -> LOCK7_W<7> {
        LOCK7_W::new(self)
    }
    #[doc = "Bit 8 - LOCK8"]
    #[inline(always)]
    #[must_use]
    pub fn lock8(&mut self) -> LOCK8_W<8> {
        LOCK8_W::new(self)
    }
    #[doc = "Bit 9 - LOCK9"]
    #[inline(always)]
    #[must_use]
    pub fn lock9(&mut self) -> LOCK9_W<9> {
        LOCK9_W::new(self)
    }
    #[doc = "Bit 10 - LOCK10"]
    #[inline(always)]
    #[must_use]
    pub fn lock10(&mut self) -> LOCK10_W<10> {
        LOCK10_W::new(self)
    }
    #[doc = "Bit 11 - LOCK11"]
    #[inline(always)]
    #[must_use]
    pub fn lock11(&mut self) -> LOCK11_W<11> {
        LOCK11_W::new(self)
    }
    #[doc = "Bit 12 - LOCK12"]
    #[inline(always)]
    #[must_use]
    pub fn lock12(&mut self) -> LOCK12_W<12> {
        LOCK12_W::new(self)
    }
    #[doc = "Bit 13 - LOCK13"]
    #[inline(always)]
    #[must_use]
    pub fn lock13(&mut self) -> LOCK13_W<13> {
        LOCK13_W::new(self)
    }
    #[doc = "Bit 14 - LOCK14"]
    #[inline(always)]
    #[must_use]
    pub fn lock14(&mut self) -> LOCK14_W<14> {
        LOCK14_W::new(self)
    }
    #[doc = "Bit 15 - LOCK15"]
    #[inline(always)]
    #[must_use]
    pub fn lock15(&mut self) -> LOCK15_W<15> {
        LOCK15_W::new(self)
    }
    #[doc = "Bits 16:31 - LKEY"]
    #[inline(always)]
    #[must_use]
    pub fn lkey(&mut self) -> LKEY_W<16> {
        LKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LOCKR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockr](index.html) module"]
pub struct LOCKR_SPEC;
impl crate::RegisterSpec for LOCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lockr::R](R) reader structure"]
impl crate::Readable for LOCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lockr::W](W) writer structure"]
impl crate::Writable for LOCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOCKR to value 0"]
impl crate::Resettable for LOCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
