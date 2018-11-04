#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CKCU_APBPCSR0 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct I2C0PCLKR {
    bits: u8,
}
impl I2C0PCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct I2C1PCLKR {
    bits: u8,
}
impl I2C1PCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SPI0PCLKR {
    bits: u8,
}
impl SPI0PCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SPI1PCLKR {
    bits: u8,
}
impl SPI1PCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BFTM0PCLKR {
    bits: u8,
}
impl BFTM0PCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BFTM1PCLKR {
    bits: u8,
}
impl BFTM1PCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MCTM0PCLKR {
    bits: u8,
}
impl MCTM0PCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPTM0PCLKR {
    bits: u8,
}
impl GPTM0PCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GPTM1PCLKR {
    bits: u8,
}
impl GPTM1PCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USR0PCLKR {
    bits: u8,
}
impl USR0PCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USR1PCLKR {
    bits: u8,
}
impl USR1PCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UR0PCLKR {
    bits: u8,
}
impl UR0PCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UR1PCLKR {
    bits: u8,
}
impl UR1PCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _I2C0PCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C0PCLKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _I2C1PCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C1PCLKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SPI0PCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI0PCLKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SPI1PCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI1PCLKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BFTM0PCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _BFTM0PCLKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BFTM1PCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _BFTM1PCLKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MCTM0PCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _MCTM0PCLKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPTM0PCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _GPTM0PCLKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPTM1PCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _GPTM1PCLKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USR0PCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _USR0PCLKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USR1PCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _USR1PCLKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UR0PCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _UR0PCLKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UR1PCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _UR1PCLKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - I2C0PCLK"]
    #[inline]
    pub fn i2c0pclk(&self) -> I2C0PCLKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        I2C0PCLKR { bits }
    }
    #[doc = "Bits 2:3 - I2C1PCLK"]
    #[inline]
    pub fn i2c1pclk(&self) -> I2C1PCLKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        I2C1PCLKR { bits }
    }
    #[doc = "Bits 4:5 - SPI0PCLK"]
    #[inline]
    pub fn spi0pclk(&self) -> SPI0PCLKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPI0PCLKR { bits }
    }
    #[doc = "Bits 6:7 - SPI1PCLK"]
    #[inline]
    pub fn spi1pclk(&self) -> SPI1PCLKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPI1PCLKR { bits }
    }
    #[doc = "Bits 12:13 - BFTM0PCLK"]
    #[inline]
    pub fn bftm0pclk(&self) -> BFTM0PCLKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BFTM0PCLKR { bits }
    }
    #[doc = "Bits 14:15 - BFTM1PCLK"]
    #[inline]
    pub fn bftm1pclk(&self) -> BFTM1PCLKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BFTM1PCLKR { bits }
    }
    #[doc = "Bits 16:17 - MCTM0PCLK"]
    #[inline]
    pub fn mctm0pclk(&self) -> MCTM0PCLKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MCTM0PCLKR { bits }
    }
    #[doc = "Bits 20:21 - GPTM0PCLK"]
    #[inline]
    pub fn gptm0pclk(&self) -> GPTM0PCLKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPTM0PCLKR { bits }
    }
    #[doc = "Bits 22:23 - GPTM1PCLK"]
    #[inline]
    pub fn gptm1pclk(&self) -> GPTM1PCLKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GPTM1PCLKR { bits }
    }
    #[doc = "Bits 24:25 - USR0PCLK"]
    #[inline]
    pub fn usr0pclk(&self) -> USR0PCLKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USR0PCLKR { bits }
    }
    #[doc = "Bits 26:27 - USR1PCLK"]
    #[inline]
    pub fn usr1pclk(&self) -> USR1PCLKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USR1PCLKR { bits }
    }
    #[doc = "Bits 28:29 - UR0PCLK"]
    #[inline]
    pub fn ur0pclk(&self) -> UR0PCLKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UR0PCLKR { bits }
    }
    #[doc = "Bits 30:31 - UR1PCLK"]
    #[inline]
    pub fn ur1pclk(&self) -> UR1PCLKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UR1PCLKR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - I2C0PCLK"]
    #[inline]
    pub fn i2c0pclk(&mut self) -> _I2C0PCLKW {
        _I2C0PCLKW { w: self }
    }
    #[doc = "Bits 2:3 - I2C1PCLK"]
    #[inline]
    pub fn i2c1pclk(&mut self) -> _I2C1PCLKW {
        _I2C1PCLKW { w: self }
    }
    #[doc = "Bits 4:5 - SPI0PCLK"]
    #[inline]
    pub fn spi0pclk(&mut self) -> _SPI0PCLKW {
        _SPI0PCLKW { w: self }
    }
    #[doc = "Bits 6:7 - SPI1PCLK"]
    #[inline]
    pub fn spi1pclk(&mut self) -> _SPI1PCLKW {
        _SPI1PCLKW { w: self }
    }
    #[doc = "Bits 12:13 - BFTM0PCLK"]
    #[inline]
    pub fn bftm0pclk(&mut self) -> _BFTM0PCLKW {
        _BFTM0PCLKW { w: self }
    }
    #[doc = "Bits 14:15 - BFTM1PCLK"]
    #[inline]
    pub fn bftm1pclk(&mut self) -> _BFTM1PCLKW {
        _BFTM1PCLKW { w: self }
    }
    #[doc = "Bits 16:17 - MCTM0PCLK"]
    #[inline]
    pub fn mctm0pclk(&mut self) -> _MCTM0PCLKW {
        _MCTM0PCLKW { w: self }
    }
    #[doc = "Bits 20:21 - GPTM0PCLK"]
    #[inline]
    pub fn gptm0pclk(&mut self) -> _GPTM0PCLKW {
        _GPTM0PCLKW { w: self }
    }
    #[doc = "Bits 22:23 - GPTM1PCLK"]
    #[inline]
    pub fn gptm1pclk(&mut self) -> _GPTM1PCLKW {
        _GPTM1PCLKW { w: self }
    }
    #[doc = "Bits 24:25 - USR0PCLK"]
    #[inline]
    pub fn usr0pclk(&mut self) -> _USR0PCLKW {
        _USR0PCLKW { w: self }
    }
    #[doc = "Bits 26:27 - USR1PCLK"]
    #[inline]
    pub fn usr1pclk(&mut self) -> _USR1PCLKW {
        _USR1PCLKW { w: self }
    }
    #[doc = "Bits 28:29 - UR0PCLK"]
    #[inline]
    pub fn ur0pclk(&mut self) -> _UR0PCLKW {
        _UR0PCLKW { w: self }
    }
    #[doc = "Bits 30:31 - UR1PCLK"]
    #[inline]
    pub fn ur1pclk(&mut self) -> _UR1PCLKW {
        _UR1PCLKW { w: self }
    }
}
