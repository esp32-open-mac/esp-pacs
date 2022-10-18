#[doc = "Register `SAR_MEAS2_MUX` reader"]
pub struct R(crate::R<SAR_MEAS2_MUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_MEAS2_MUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_MEAS2_MUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_MEAS2_MUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_MEAS2_MUX` writer"]
pub struct W(crate::W<SAR_MEAS2_MUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_MEAS2_MUX_SPEC>;
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
impl From<crate::W<SAR_MEAS2_MUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_MEAS2_MUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_SAR2_PWDET_CCT` reader - SAR2_PWDET_CCT"]
pub type SAR_SAR2_PWDET_CCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR_SAR2_PWDET_CCT` writer - SAR2_PWDET_CCT"]
pub type SAR_SAR2_PWDET_CCT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_MEAS2_MUX_SPEC, u8, u8, 3, O>;
#[doc = "Field `SAR_SAR2_RTC_FORCE` reader - in sleep, force to use rtc to control ADC"]
pub type SAR_SAR2_RTC_FORCE_R = crate::BitReader<bool>;
#[doc = "Field `SAR_SAR2_RTC_FORCE` writer - in sleep, force to use rtc to control ADC"]
pub type SAR_SAR2_RTC_FORCE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_MEAS2_MUX_SPEC, bool, O>;
impl R {
    #[doc = "Bits 28:30 - SAR2_PWDET_CCT"]
    #[inline(always)]
    pub fn sar_sar2_pwdet_cct(&self) -> SAR_SAR2_PWDET_CCT_R {
        SAR_SAR2_PWDET_CCT_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - in sleep, force to use rtc to control ADC"]
    #[inline(always)]
    pub fn sar_sar2_rtc_force(&self) -> SAR_SAR2_RTC_FORCE_R {
        SAR_SAR2_RTC_FORCE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 28:30 - SAR2_PWDET_CCT"]
    #[inline(always)]
    pub fn sar_sar2_pwdet_cct(&mut self) -> SAR_SAR2_PWDET_CCT_W<28> {
        SAR_SAR2_PWDET_CCT_W::new(self)
    }
    #[doc = "Bit 31 - in sleep, force to use rtc to control ADC"]
    #[inline(always)]
    pub fn sar_sar2_rtc_force(&mut self) -> SAR_SAR2_RTC_FORCE_W<31> {
        SAR_SAR2_RTC_FORCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure saradc2 controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_meas2_mux](index.html) module"]
pub struct SAR_MEAS2_MUX_SPEC;
impl crate::RegisterSpec for SAR_MEAS2_MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_meas2_mux::R](R) reader structure"]
impl crate::Readable for SAR_MEAS2_MUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_meas2_mux::W](W) writer structure"]
impl crate::Writable for SAR_MEAS2_MUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_MEAS2_MUX to value 0"]
impl crate::Resettable for SAR_MEAS2_MUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}