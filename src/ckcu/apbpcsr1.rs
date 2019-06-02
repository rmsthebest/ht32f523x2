#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::APBPCSR1 {
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
pub struct AFIOPCLKR {
    bits: u8,
}
impl AFIOPCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTIPCLKR {
    bits: u8,
}
impl EXTIPCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADCCPCLKR {
    bits: u8,
}
impl ADCCPCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CMPPCLKR {
    bits: u8,
}
impl CMPPCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WDTRPCLKR {
    bits: u8,
}
impl WDTRPCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BKPRPCLKR {
    bits: u8,
}
impl BKPRPCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SCI0PCLKR {
    bits: u8,
}
impl SCI0PCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SCI1PCLKR {
    bits: u8,
}
impl SCI1PCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct I2SPCLKR {
    bits: u8,
}
impl I2SPCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SCTM0PCLKR {
    bits: u8,
}
impl SCTM0PCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SCTM1PCLKR {
    bits: u8,
}
impl SCTM1PCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _AFIOPCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _AFIOPCLKW<'a> {
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
pub struct _EXTIPCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPCLKW<'a> {
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
pub struct _ADCCPCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCCPCLKW<'a> {
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
pub struct _CMPPCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPPCLKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WDTRPCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTRPCLKW<'a> {
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
pub struct _BKPRPCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _BKPRPCLKW<'a> {
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
pub struct _SCI0PCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _SCI0PCLKW<'a> {
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
pub struct _SCI1PCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _SCI1PCLKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _I2SPCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _I2SPCLKW<'a> {
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
pub struct _SCTM0PCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _SCTM0PCLKW<'a> {
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
pub struct _SCTM1PCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _SCTM1PCLKW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - AFIOPCLK"]
    #[inline]
    pub fn afiopclk(&self) -> AFIOPCLKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AFIOPCLKR { bits }
    }
    #[doc = "Bits 2:3 - EXTIPCLK"]
    #[inline]
    pub fn extipclk(&self) -> EXTIPCLKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTIPCLKR { bits }
    }
    #[doc = "Bits 4:5 - ADCCPCLK"]
    #[inline]
    pub fn adccpclk(&self) -> ADCCPCLKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADCCPCLKR { bits }
    }
    #[doc = "Bits 8:9 - CMPPCLK"]
    #[inline]
    pub fn cmppclk(&self) -> CMPPCLKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMPPCLKR { bits }
    }
    #[doc = "Bits 12:13 - WDTRPCLK"]
    #[inline]
    pub fn wdtrpclk(&self) -> WDTRPCLKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WDTRPCLKR { bits }
    }
    #[doc = "Bits 14:15 - BKPRPCLK"]
    #[inline]
    pub fn bkprpclk(&self) -> BKPRPCLKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BKPRPCLKR { bits }
    }
    #[doc = "Bits 16:17 - SCI0PCLK"]
    #[inline]
    pub fn sci0pclk(&self) -> SCI0PCLKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SCI0PCLKR { bits }
    }
    #[doc = "Bits 18:19 - SCI1PCLK"]
    #[inline]
    pub fn sci1pclk(&self) -> SCI1PCLKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SCI1PCLKR { bits }
    }
    #[doc = "Bits 20:21 - I2SPCLK"]
    #[inline]
    pub fn i2spclk(&self) -> I2SPCLKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        I2SPCLKR { bits }
    }
    #[doc = "Bits 24:25 - SCTM0PCLK"]
    #[inline]
    pub fn sctm0pclk(&self) -> SCTM0PCLKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SCTM0PCLKR { bits }
    }
    #[doc = "Bits 26:27 - SCTM1PCLK"]
    #[inline]
    pub fn sctm1pclk(&self) -> SCTM1PCLKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SCTM1PCLKR { bits }
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
    #[doc = "Bits 0:1 - AFIOPCLK"]
    #[inline]
    pub fn afiopclk(&mut self) -> _AFIOPCLKW {
        _AFIOPCLKW { w: self }
    }
    #[doc = "Bits 2:3 - EXTIPCLK"]
    #[inline]
    pub fn extipclk(&mut self) -> _EXTIPCLKW {
        _EXTIPCLKW { w: self }
    }
    #[doc = "Bits 4:5 - ADCCPCLK"]
    #[inline]
    pub fn adccpclk(&mut self) -> _ADCCPCLKW {
        _ADCCPCLKW { w: self }
    }
    #[doc = "Bits 8:9 - CMPPCLK"]
    #[inline]
    pub fn cmppclk(&mut self) -> _CMPPCLKW {
        _CMPPCLKW { w: self }
    }
    #[doc = "Bits 12:13 - WDTRPCLK"]
    #[inline]
    pub fn wdtrpclk(&mut self) -> _WDTRPCLKW {
        _WDTRPCLKW { w: self }
    }
    #[doc = "Bits 14:15 - BKPRPCLK"]
    #[inline]
    pub fn bkprpclk(&mut self) -> _BKPRPCLKW {
        _BKPRPCLKW { w: self }
    }
    #[doc = "Bits 16:17 - SCI0PCLK"]
    #[inline]
    pub fn sci0pclk(&mut self) -> _SCI0PCLKW {
        _SCI0PCLKW { w: self }
    }
    #[doc = "Bits 18:19 - SCI1PCLK"]
    #[inline]
    pub fn sci1pclk(&mut self) -> _SCI1PCLKW {
        _SCI1PCLKW { w: self }
    }
    #[doc = "Bits 20:21 - I2SPCLK"]
    #[inline]
    pub fn i2spclk(&mut self) -> _I2SPCLKW {
        _I2SPCLKW { w: self }
    }
    #[doc = "Bits 24:25 - SCTM0PCLK"]
    #[inline]
    pub fn sctm0pclk(&mut self) -> _SCTM0PCLKW {
        _SCTM0PCLKW { w: self }
    }
    #[doc = "Bits 26:27 - SCTM1PCLK"]
    #[inline]
    pub fn sctm1pclk(&mut self) -> _SCTM1PCLKW {
        _SCTM1PCLKW { w: self }
    }
}
