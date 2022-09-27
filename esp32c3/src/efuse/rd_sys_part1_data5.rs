#[doc = "Register `RD_SYS_PART1_DATA5` reader"]
pub struct R(crate::R<RD_SYS_PART1_DATA5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_SYS_PART1_DATA5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_SYS_PART1_DATA5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_SYS_PART1_DATA5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYS_DATA_PART1_5` reader - Stores the fifth 32 bits of the first part of system data."]
pub type SYS_DATA_PART1_5_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the fifth 32 bits of the first part of system data."]
    #[inline(always)]
    pub fn sys_data_part1_5(&self) -> SYS_DATA_PART1_5_R {
        SYS_DATA_PART1_5_R::new(self.bits)
    }
}
#[doc = "Register 5 of BLOCK2 (system).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_sys_part1_data5](index.html) module"]
pub struct RD_SYS_PART1_DATA5_SPEC;
impl crate::RegisterSpec for RD_SYS_PART1_DATA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_sys_part1_data5::R](R) reader structure"]
impl crate::Readable for RD_SYS_PART1_DATA5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_SYS_PART1_DATA5 to value 0"]
impl crate::Resettable for RD_SYS_PART1_DATA5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}