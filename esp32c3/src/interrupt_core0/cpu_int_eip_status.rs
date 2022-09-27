#[doc = "Register `CPU_INT_EIP_STATUS` reader"]
pub struct R(crate::R<CPU_INT_EIP_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_INT_EIP_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_INT_EIP_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_INT_EIP_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CPU_INT_EIP_STATUS` reader - reg_core0_cpu_int_eip_status"]
pub type CPU_INT_EIP_STATUS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - reg_core0_cpu_int_eip_status"]
    #[inline(always)]
    pub fn cpu_int_eip_status(&self) -> CPU_INT_EIP_STATUS_R {
        CPU_INT_EIP_STATUS_R::new(self.bits)
    }
}
#[doc = "mac intr map register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_int_eip_status](index.html) module"]
pub struct CPU_INT_EIP_STATUS_SPEC;
impl crate::RegisterSpec for CPU_INT_EIP_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_int_eip_status::R](R) reader structure"]
impl crate::Readable for CPU_INT_EIP_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CPU_INT_EIP_STATUS to value 0"]
impl crate::Resettable for CPU_INT_EIP_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}