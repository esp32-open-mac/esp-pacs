#[doc = "Register `SAR_TOUCH_THRES1` reader"]
pub struct R(crate::R<SAR_TOUCH_THRES1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_THRES1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_THRES1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_THRES1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_TOUCH_THRES1` writer"]
pub struct W(crate::W<SAR_TOUCH_THRES1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_TOUCH_THRES1_SPEC>;
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
impl From<crate::W<SAR_TOUCH_THRES1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_TOUCH_THRES1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_TOUCH_OUT_TH1` reader - Finger threshold for touch pad 1"]
pub type SAR_TOUCH_OUT_TH1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SAR_TOUCH_OUT_TH1` writer - Finger threshold for touch pad 1"]
pub type SAR_TOUCH_OUT_TH1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_TOUCH_THRES1_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 1"]
    #[inline(always)]
    pub fn sar_touch_out_th1(&self) -> SAR_TOUCH_OUT_TH1_R {
        SAR_TOUCH_OUT_TH1_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 1"]
    #[inline(always)]
    pub fn sar_touch_out_th1(&mut self) -> SAR_TOUCH_OUT_TH1_W<0> {
        SAR_TOUCH_OUT_TH1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure touch thres of touch pad\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_thres1](index.html) module"]
pub struct SAR_TOUCH_THRES1_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_THRES1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_thres1::R](R) reader structure"]
impl crate::Readable for SAR_TOUCH_THRES1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_touch_thres1::W](W) writer structure"]
impl crate::Writable for SAR_TOUCH_THRES1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_TOUCH_THRES1 to value 0"]
impl crate::Resettable for SAR_TOUCH_THRES1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}