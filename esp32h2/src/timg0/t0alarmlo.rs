#[doc = "Register `T0ALARMLO` reader"]
pub struct R(crate::R<T0ALARMLO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0ALARMLO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0ALARMLO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0ALARMLO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T0ALARMLO` writer"]
pub struct W(crate::W<T0ALARMLO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T0ALARMLO_SPEC>;
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
impl From<crate::W<T0ALARMLO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T0ALARMLO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_ALARM_LO` reader - Timer %s alarm trigger time-base counter value, low 32 bits."]
pub type T_ALARM_LO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `T_ALARM_LO` writer - Timer %s alarm trigger time-base counter value, low 32 bits."]
pub type T_ALARM_LO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, T0ALARMLO_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Timer %s alarm trigger time-base counter value, low 32 bits."]
    #[inline(always)]
    pub fn t_alarm_lo(&self) -> T_ALARM_LO_R {
        T_ALARM_LO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer %s alarm trigger time-base counter value, low 32 bits."]
    #[inline(always)]
    #[must_use]
    pub fn t_alarm_lo(&mut self) -> T_ALARM_LO_W<0> {
        T_ALARM_LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer %s alarm value, low 32 bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0alarmlo](index.html) module"]
pub struct T0ALARMLO_SPEC;
impl crate::RegisterSpec for T0ALARMLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0alarmlo::R](R) reader structure"]
impl crate::Readable for T0ALARMLO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t0alarmlo::W](W) writer structure"]
impl crate::Writable for T0ALARMLO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T0ALARMLO to value 0"]
impl crate::Resettable for T0ALARMLO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}