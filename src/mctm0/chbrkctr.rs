#[doc = "Register `CHBRKCTR` reader"]
pub struct R(crate::R<CHBRKCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHBRKCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHBRKCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHBRKCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHBRKCTR` writer"]
pub struct W(crate::W<CHBRKCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHBRKCTR_SPEC>;
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
impl From<crate::W<CHBRKCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHBRKCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKE0` reader - BKE0"]
pub type BKE0_R = crate::BitReader<bool>;
#[doc = "Field `BKE0` writer - BKE0"]
pub type BKE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHBRKCTR_SPEC, bool, O>;
#[doc = "Field `BKP0` reader - BKP0"]
pub type BKP0_R = crate::BitReader<bool>;
#[doc = "Field `BKP0` writer - BKP0"]
pub type BKP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHBRKCTR_SPEC, bool, O>;
#[doc = "Field `CHMOE` reader - CHMOE"]
pub type CHMOE_R = crate::BitReader<bool>;
#[doc = "Field `CHMOE` writer - CHMOE"]
pub type CHMOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHBRKCTR_SPEC, bool, O>;
#[doc = "Field `CHAOE` reader - CHAOE"]
pub type CHAOE_R = crate::BitReader<bool>;
#[doc = "Field `CHAOE` writer - CHAOE"]
pub type CHAOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHBRKCTR_SPEC, bool, O>;
#[doc = "Field `BKF0` reader - BKF0"]
pub type BKF0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BKF0` writer - BKF0"]
pub type BKF0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHBRKCTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `LOCKLV` reader - LOCKLV"]
pub type LOCKLV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOCKLV` writer - LOCKLV"]
pub type LOCKLV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHBRKCTR_SPEC, u8, u8, 2, O>;
#[doc = "Field `GFSEL0` reader - GFSEL0"]
pub type GFSEL0_R = crate::BitReader<bool>;
#[doc = "Field `GFSEL0` writer - GFSEL0"]
pub type GFSEL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHBRKCTR_SPEC, bool, O>;
#[doc = "Field `CHOSSI` reader - CHOSSI"]
pub type CHOSSI_R = crate::BitReader<bool>;
#[doc = "Field `CHOSSI` writer - CHOSSI"]
pub type CHOSSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHBRKCTR_SPEC, bool, O>;
#[doc = "Field `CHOSSR` reader - CHOSSR"]
pub type CHOSSR_R = crate::BitReader<bool>;
#[doc = "Field `CHOSSR` writer - CHOSSR"]
pub type CHOSSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHBRKCTR_SPEC, bool, O>;
#[doc = "Field `CHDTG` reader - CHDTG"]
pub type CHDTG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHDTG` writer - CHDTG"]
pub type CHDTG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHBRKCTR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - BKE0"]
    #[inline(always)]
    pub fn bke0(&self) -> BKE0_R {
        BKE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BKP0"]
    #[inline(always)]
    pub fn bkp0(&self) -> BKP0_R {
        BKP0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - CHMOE"]
    #[inline(always)]
    pub fn chmoe(&self) -> CHMOE_R {
        CHMOE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CHAOE"]
    #[inline(always)]
    pub fn chaoe(&self) -> CHAOE_R {
        CHAOE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - BKF0"]
    #[inline(always)]
    pub fn bkf0(&self) -> BKF0_R {
        BKF0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - LOCKLV"]
    #[inline(always)]
    pub fn locklv(&self) -> LOCKLV_R {
        LOCKLV_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - GFSEL0"]
    #[inline(always)]
    pub fn gfsel0(&self) -> GFSEL0_R {
        GFSEL0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - CHOSSI"]
    #[inline(always)]
    pub fn chossi(&self) -> CHOSSI_R {
        CHOSSI_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CHOSSR"]
    #[inline(always)]
    pub fn chossr(&self) -> CHOSSR_R {
        CHOSSR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:31 - CHDTG"]
    #[inline(always)]
    pub fn chdtg(&self) -> CHDTG_R {
        CHDTG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - BKE0"]
    #[inline(always)]
    #[must_use]
    pub fn bke0(&mut self) -> BKE0_W<0> {
        BKE0_W::new(self)
    }
    #[doc = "Bit 1 - BKP0"]
    #[inline(always)]
    #[must_use]
    pub fn bkp0(&mut self) -> BKP0_W<1> {
        BKP0_W::new(self)
    }
    #[doc = "Bit 4 - CHMOE"]
    #[inline(always)]
    #[must_use]
    pub fn chmoe(&mut self) -> CHMOE_W<4> {
        CHMOE_W::new(self)
    }
    #[doc = "Bit 5 - CHAOE"]
    #[inline(always)]
    #[must_use]
    pub fn chaoe(&mut self) -> CHAOE_W<5> {
        CHAOE_W::new(self)
    }
    #[doc = "Bits 8:11 - BKF0"]
    #[inline(always)]
    #[must_use]
    pub fn bkf0(&mut self) -> BKF0_W<8> {
        BKF0_W::new(self)
    }
    #[doc = "Bits 16:17 - LOCKLV"]
    #[inline(always)]
    #[must_use]
    pub fn locklv(&mut self) -> LOCKLV_W<16> {
        LOCKLV_W::new(self)
    }
    #[doc = "Bit 18 - GFSEL0"]
    #[inline(always)]
    #[must_use]
    pub fn gfsel0(&mut self) -> GFSEL0_W<18> {
        GFSEL0_W::new(self)
    }
    #[doc = "Bit 20 - CHOSSI"]
    #[inline(always)]
    #[must_use]
    pub fn chossi(&mut self) -> CHOSSI_W<20> {
        CHOSSI_W::new(self)
    }
    #[doc = "Bit 21 - CHOSSR"]
    #[inline(always)]
    #[must_use]
    pub fn chossr(&mut self) -> CHOSSR_W<21> {
        CHOSSR_W::new(self)
    }
    #[doc = "Bits 24:31 - CHDTG"]
    #[inline(always)]
    #[must_use]
    pub fn chdtg(&mut self) -> CHDTG_W<24> {
        CHDTG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CHBRKCTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chbrkctr](index.html) module"]
pub struct CHBRKCTR_SPEC;
impl crate::RegisterSpec for CHBRKCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chbrkctr::R](R) reader structure"]
impl crate::Readable for CHBRKCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chbrkctr::W](W) writer structure"]
impl crate::Writable for CHBRKCTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHBRKCTR to value 0"]
impl crate::Resettable for CHBRKCTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
