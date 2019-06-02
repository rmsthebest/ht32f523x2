#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ESSR0 {
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
pub struct EXTI0PINR {
    bits: u8,
}
impl EXTI0PINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTI1PINR {
    bits: u8,
}
impl EXTI1PINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTI2PINR {
    bits: u8,
}
impl EXTI2PINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTI3PINR {
    bits: u8,
}
impl EXTI3PINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTI4PINR {
    bits: u8,
}
impl EXTI4PINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTI5PINR {
    bits: u8,
}
impl EXTI5PINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTI6PINR {
    bits: u8,
}
impl EXTI6PINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTI7PINR {
    bits: u8,
}
impl EXTI7PINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _EXTI0PINW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI0PINW<'a> {
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
pub struct _EXTI1PINW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI1PINW<'a> {
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
pub struct _EXTI2PINW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI2PINW<'a> {
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
pub struct _EXTI3PINW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI3PINW<'a> {
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
pub struct _EXTI4PINW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI4PINW<'a> {
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
pub struct _EXTI5PINW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI5PINW<'a> {
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
pub struct _EXTI6PINW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI6PINW<'a> {
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
pub struct _EXTI7PINW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTI7PINW<'a> {
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
    #[doc = "Bits 0:3 - EXTI0PIN"]
    #[inline]
    pub fn exti0pin(&self) -> EXTI0PINR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI0PINR { bits }
    }
    #[doc = "Bits 4:7 - EXTI1PIN"]
    #[inline]
    pub fn exti1pin(&self) -> EXTI1PINR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI1PINR { bits }
    }
    #[doc = "Bits 8:11 - EXTI2PIN"]
    #[inline]
    pub fn exti2pin(&self) -> EXTI2PINR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI2PINR { bits }
    }
    #[doc = "Bits 12:15 - EXTI3PIN"]
    #[inline]
    pub fn exti3pin(&self) -> EXTI3PINR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI3PINR { bits }
    }
    #[doc = "Bits 16:19 - EXTI4PIN"]
    #[inline]
    pub fn exti4pin(&self) -> EXTI4PINR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI4PINR { bits }
    }
    #[doc = "Bits 20:23 - EXTI5PIN"]
    #[inline]
    pub fn exti5pin(&self) -> EXTI5PINR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI5PINR { bits }
    }
    #[doc = "Bits 24:27 - EXTI6PIN"]
    #[inline]
    pub fn exti6pin(&self) -> EXTI6PINR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI6PINR { bits }
    }
    #[doc = "Bits 28:31 - EXTI7PIN"]
    #[inline]
    pub fn exti7pin(&self) -> EXTI7PINR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTI7PINR { bits }
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
    #[doc = "Bits 0:3 - EXTI0PIN"]
    #[inline]
    pub fn exti0pin(&mut self) -> _EXTI0PINW {
        _EXTI0PINW { w: self }
    }
    #[doc = "Bits 4:7 - EXTI1PIN"]
    #[inline]
    pub fn exti1pin(&mut self) -> _EXTI1PINW {
        _EXTI1PINW { w: self }
    }
    #[doc = "Bits 8:11 - EXTI2PIN"]
    #[inline]
    pub fn exti2pin(&mut self) -> _EXTI2PINW {
        _EXTI2PINW { w: self }
    }
    #[doc = "Bits 12:15 - EXTI3PIN"]
    #[inline]
    pub fn exti3pin(&mut self) -> _EXTI3PINW {
        _EXTI3PINW { w: self }
    }
    #[doc = "Bits 16:19 - EXTI4PIN"]
    #[inline]
    pub fn exti4pin(&mut self) -> _EXTI4PINW {
        _EXTI4PINW { w: self }
    }
    #[doc = "Bits 20:23 - EXTI5PIN"]
    #[inline]
    pub fn exti5pin(&mut self) -> _EXTI5PINW {
        _EXTI5PINW { w: self }
    }
    #[doc = "Bits 24:27 - EXTI6PIN"]
    #[inline]
    pub fn exti6pin(&mut self) -> _EXTI6PINW {
        _EXTI6PINW { w: self }
    }
    #[doc = "Bits 28:31 - EXTI7PIN"]
    #[inline]
    pub fn exti7pin(&mut self) -> _EXTI7PINW {
        _EXTI7PINW { w: self }
    }
}
