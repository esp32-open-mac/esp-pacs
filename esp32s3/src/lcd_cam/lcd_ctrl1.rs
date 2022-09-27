#[doc = "Register `LCD_CTRL1` reader"]
pub struct R(crate::R<LCD_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCD_CTRL1` writer"]
pub struct W(crate::W<LCD_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_CTRL1_SPEC>;
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
impl From<crate::W<LCD_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCD_VB_FRONT` reader - It is the vertical blank front porch of a frame."]
pub type LCD_VB_FRONT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LCD_VB_FRONT` writer - It is the vertical blank front porch of a frame."]
pub type LCD_VB_FRONT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_CTRL1_SPEC, u8, u8, 8, O>;
#[doc = "Field `LCD_HA_WIDTH` reader - It is the horizontal active width of a frame."]
pub type LCD_HA_WIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LCD_HA_WIDTH` writer - It is the horizontal active width of a frame."]
pub type LCD_HA_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_CTRL1_SPEC, u16, u16, 12, O>;
#[doc = "Field `LCD_HT_WIDTH` reader - It is the horizontal total width of a frame."]
pub type LCD_HT_WIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LCD_HT_WIDTH` writer - It is the horizontal total width of a frame."]
pub type LCD_HT_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_CTRL1_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:7 - It is the vertical blank front porch of a frame."]
    #[inline(always)]
    pub fn lcd_vb_front(&self) -> LCD_VB_FRONT_R {
        LCD_VB_FRONT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:19 - It is the horizontal active width of a frame."]
    #[inline(always)]
    pub fn lcd_ha_width(&self) -> LCD_HA_WIDTH_R {
        LCD_HA_WIDTH_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - It is the horizontal total width of a frame."]
    #[inline(always)]
    pub fn lcd_ht_width(&self) -> LCD_HT_WIDTH_R {
        LCD_HT_WIDTH_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - It is the vertical blank front porch of a frame."]
    #[inline(always)]
    pub fn lcd_vb_front(&mut self) -> LCD_VB_FRONT_W<0> {
        LCD_VB_FRONT_W::new(self)
    }
    #[doc = "Bits 8:19 - It is the horizontal active width of a frame."]
    #[inline(always)]
    pub fn lcd_ha_width(&mut self) -> LCD_HA_WIDTH_W<8> {
        LCD_HA_WIDTH_W::new(self)
    }
    #[doc = "Bits 20:31 - It is the horizontal total width of a frame."]
    #[inline(always)]
    pub fn lcd_ht_width(&mut self) -> LCD_HT_WIDTH_W<20> {
        LCD_HT_WIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_ctrl1](index.html) module"]
pub struct LCD_CTRL1_SPEC;
impl crate::RegisterSpec for LCD_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_ctrl1::R](R) reader structure"]
impl crate::Readable for LCD_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_ctrl1::W](W) writer structure"]
impl crate::Writable for LCD_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCD_CTRL1 to value 0"]
impl crate::Resettable for LCD_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}