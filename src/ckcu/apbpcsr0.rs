#[doc = "Register `APBPCSR0` reader"]
pub struct R(crate::R<APBPCSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBPCSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBPCSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBPCSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBPCSR0` writer"]
pub struct W(crate::W<APBPCSR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBPCSR0_SPEC>;
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
impl From<crate::W<APBPCSR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBPCSR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C0PCLK` reader - I2C0PCLK"]
pub type I2C0PCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2C0PCLK` writer - I2C0PCLK"]
pub type I2C0PCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, APBPCSR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `I2C1PCLK` reader - I2C1PCLK"]
pub type I2C1PCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2C1PCLK` writer - I2C1PCLK"]
pub type I2C1PCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, APBPCSR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `SPI0PCLK` reader - SPI0PCLK"]
pub type SPI0PCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI0PCLK` writer - SPI0PCLK"]
pub type SPI0PCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, APBPCSR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `SPI1PCLK` reader - SPI1PCLK"]
pub type SPI1PCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI1PCLK` writer - SPI1PCLK"]
pub type SPI1PCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, APBPCSR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `BFTM0PCLK` reader - BFTM0PCLK"]
pub type BFTM0PCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BFTM0PCLK` writer - BFTM0PCLK"]
pub type BFTM0PCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, APBPCSR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `BFTM1PCLK` reader - BFTM1PCLK"]
pub type BFTM1PCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BFTM1PCLK` writer - BFTM1PCLK"]
pub type BFTM1PCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, APBPCSR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `MCTM0PCLK` reader - MCTM0PCLK"]
pub type MCTM0PCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCTM0PCLK` writer - MCTM0PCLK"]
pub type MCTM0PCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, APBPCSR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `GPTM0PCLK` reader - GPTM0PCLK"]
pub type GPTM0PCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPTM0PCLK` writer - GPTM0PCLK"]
pub type GPTM0PCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, APBPCSR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `GPTM1PCLK` reader - GPTM1PCLK"]
pub type GPTM1PCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPTM1PCLK` writer - GPTM1PCLK"]
pub type GPTM1PCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, APBPCSR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `USR0PCLK` reader - USR0PCLK"]
pub type USR0PCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USR0PCLK` writer - USR0PCLK"]
pub type USR0PCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, APBPCSR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `USR1PCLK` reader - USR1PCLK"]
pub type USR1PCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USR1PCLK` writer - USR1PCLK"]
pub type USR1PCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, APBPCSR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `UR0PCLK` reader - UR0PCLK"]
pub type UR0PCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UR0PCLK` writer - UR0PCLK"]
pub type UR0PCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, APBPCSR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `UR1PCLK` reader - UR1PCLK"]
pub type UR1PCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UR1PCLK` writer - UR1PCLK"]
pub type UR1PCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, APBPCSR0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - I2C0PCLK"]
    #[inline(always)]
    pub fn i2c0pclk(&self) -> I2C0PCLK_R {
        I2C0PCLK_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - I2C1PCLK"]
    #[inline(always)]
    pub fn i2c1pclk(&self) -> I2C1PCLK_R {
        I2C1PCLK_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - SPI0PCLK"]
    #[inline(always)]
    pub fn spi0pclk(&self) -> SPI0PCLK_R {
        SPI0PCLK_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - SPI1PCLK"]
    #[inline(always)]
    pub fn spi1pclk(&self) -> SPI1PCLK_R {
        SPI1PCLK_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 12:13 - BFTM0PCLK"]
    #[inline(always)]
    pub fn bftm0pclk(&self) -> BFTM0PCLK_R {
        BFTM0PCLK_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - BFTM1PCLK"]
    #[inline(always)]
    pub fn bftm1pclk(&self) -> BFTM1PCLK_R {
        BFTM1PCLK_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - MCTM0PCLK"]
    #[inline(always)]
    pub fn mctm0pclk(&self) -> MCTM0PCLK_R {
        MCTM0PCLK_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - GPTM0PCLK"]
    #[inline(always)]
    pub fn gptm0pclk(&self) -> GPTM0PCLK_R {
        GPTM0PCLK_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - GPTM1PCLK"]
    #[inline(always)]
    pub fn gptm1pclk(&self) -> GPTM1PCLK_R {
        GPTM1PCLK_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - USR0PCLK"]
    #[inline(always)]
    pub fn usr0pclk(&self) -> USR0PCLK_R {
        USR0PCLK_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - USR1PCLK"]
    #[inline(always)]
    pub fn usr1pclk(&self) -> USR1PCLK_R {
        USR1PCLK_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - UR0PCLK"]
    #[inline(always)]
    pub fn ur0pclk(&self) -> UR0PCLK_R {
        UR0PCLK_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - UR1PCLK"]
    #[inline(always)]
    pub fn ur1pclk(&self) -> UR1PCLK_R {
        UR1PCLK_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - I2C0PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0pclk(&mut self) -> I2C0PCLK_W<0> {
        I2C0PCLK_W::new(self)
    }
    #[doc = "Bits 2:3 - I2C1PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1pclk(&mut self) -> I2C1PCLK_W<2> {
        I2C1PCLK_W::new(self)
    }
    #[doc = "Bits 4:5 - SPI0PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn spi0pclk(&mut self) -> SPI0PCLK_W<4> {
        SPI0PCLK_W::new(self)
    }
    #[doc = "Bits 6:7 - SPI1PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn spi1pclk(&mut self) -> SPI1PCLK_W<6> {
        SPI1PCLK_W::new(self)
    }
    #[doc = "Bits 12:13 - BFTM0PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn bftm0pclk(&mut self) -> BFTM0PCLK_W<12> {
        BFTM0PCLK_W::new(self)
    }
    #[doc = "Bits 14:15 - BFTM1PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn bftm1pclk(&mut self) -> BFTM1PCLK_W<14> {
        BFTM1PCLK_W::new(self)
    }
    #[doc = "Bits 16:17 - MCTM0PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn mctm0pclk(&mut self) -> MCTM0PCLK_W<16> {
        MCTM0PCLK_W::new(self)
    }
    #[doc = "Bits 20:21 - GPTM0PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn gptm0pclk(&mut self) -> GPTM0PCLK_W<20> {
        GPTM0PCLK_W::new(self)
    }
    #[doc = "Bits 22:23 - GPTM1PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn gptm1pclk(&mut self) -> GPTM1PCLK_W<22> {
        GPTM1PCLK_W::new(self)
    }
    #[doc = "Bits 24:25 - USR0PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn usr0pclk(&mut self) -> USR0PCLK_W<24> {
        USR0PCLK_W::new(self)
    }
    #[doc = "Bits 26:27 - USR1PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn usr1pclk(&mut self) -> USR1PCLK_W<26> {
        USR1PCLK_W::new(self)
    }
    #[doc = "Bits 28:29 - UR0PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn ur0pclk(&mut self) -> UR0PCLK_W<28> {
        UR0PCLK_W::new(self)
    }
    #[doc = "Bits 30:31 - UR1PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn ur1pclk(&mut self) -> UR1PCLK_W<30> {
        UR1PCLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APBPCSR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbpcsr0](index.html) module"]
pub struct APBPCSR0_SPEC;
impl crate::RegisterSpec for APBPCSR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbpcsr0::R](R) reader structure"]
impl crate::Readable for APBPCSR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbpcsr0::W](W) writer structure"]
impl crate::Writable for APBPCSR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBPCSR0 to value 0"]
impl crate::Resettable for APBPCSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
