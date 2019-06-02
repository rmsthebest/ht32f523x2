#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ESSR1 {
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
pub struct EXTI8PINR {
    bits: u8,
}
impl EXTI8PINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTI9PINR {
    bits: u8,
}
impl EXTI9PINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTI10PINR {
    bits: u8,
}
impl EXTI10PINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTI11PINR {
    bits: u8,
}
impl EXTI11PINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTI12PINR {
    bits: u8,
}
impl EXTI12PINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTI13PINR {
    bits: u8,
}
impl EXTI13PINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTI14PINR {
    bits: u8,
}
impl EXTI14PINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTI15PINR {
    bits: u8,
}
impl EXTI15PINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _EXTI8PINW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI8PINW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EXTI9PINW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI9PINW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EXTI10PINW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI10PINW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EXTI11PINW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI11PINW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EXTI12PINW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI12PINW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EXTI13PINW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI13PINW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EXTI14PINW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI14PINW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EXTI15PINW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI15PINW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:3 - EXTI8PIN"]
    #[inline]
    pub fn exti8pin(&self) -> EXTI8PINR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI8PINR { bits }
    }
    #[doc = "Bits 4:7 - EXTI9PIN"]
    #[inline]
    pub fn exti9pin(&self) -> EXTI9PINR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI9PINR { bits }
    }
    #[doc = "Bits 8:11 - EXTI10PIN"]
    #[inline]
    pub fn exti10pin(&self) -> EXTI10PINR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI10PINR { bits }
    }
    #[doc = "Bits 12:15 - EXTI11PIN"]
    #[inline]
    pub fn exti11pin(&self) -> EXTI11PINR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI11PINR { bits }
    }
    #[doc = "Bits 16:19 - EXTI12PIN"]
    #[inline]
    pub fn exti12pin(&self) -> EXTI12PINR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI12PINR { bits }
    }
    #[doc = "Bits 20:23 - EXTI13PIN"]
    #[inline]
    pub fn exti13pin(&self) -> EXTI13PINR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI13PINR { bits }
    }
    #[doc = "Bits 24:27 - EXTI14PIN"]
    #[inline]
    pub fn exti14pin(&self) -> EXTI14PINR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI14PINR { bits }
    }
    #[doc = "Bits 28:31 - EXTI15PIN"]
    #[inline]
    pub fn exti15pin(&self) -> EXTI15PINR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI15PINR { bits }
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
    #[doc = "Bits 0:3 - EXTI8PIN"]
    #[inline]
    pub fn exti8pin(&mut self) -> _EXTI8PINW {
        _EXTI8PINW { w: self }
    }
    #[doc = "Bits 4:7 - EXTI9PIN"]
    #[inline]
    pub fn exti9pin(&mut self) -> _EXTI9PINW {
        _EXTI9PINW { w: self }
    }
    #[doc = "Bits 8:11 - EXTI10PIN"]
    #[inline]
    pub fn exti10pin(&mut self) -> _EXTI10PINW {
        _EXTI10PINW { w: self }
    }
    #[doc = "Bits 12:15 - EXTI11PIN"]
    #[inline]
    pub fn exti11pin(&mut self) -> _EXTI11PINW {
        _EXTI11PINW { w: self }
    }
    #[doc = "Bits 16:19 - EXTI12PIN"]
    #[inline]
    pub fn exti12pin(&mut self) -> _EXTI12PINW {
        _EXTI12PINW { w: self }
    }
    #[doc = "Bits 20:23 - EXTI13PIN"]
    #[inline]
    pub fn exti13pin(&mut self) -> _EXTI13PINW {
        _EXTI13PINW { w: self }
    }
    #[doc = "Bits 24:27 - EXTI14PIN"]
    #[inline]
    pub fn exti14pin(&mut self) -> _EXTI14PINW {
        _EXTI14PINW { w: self }
    }
    #[doc = "Bits 28:31 - EXTI15PIN"]
    #[inline]
    pub fn exti15pin(&mut self) -> _EXTI15PINW {
        _EXTI15PINW { w: self }
    }
}
