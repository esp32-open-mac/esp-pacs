#[doc = "Register `CORE_0_SP_PC` reader"]
pub type R = crate::R<CORE_0_SP_PC_SPEC>;
#[doc = "Field `CORE_0_SP_PC` reader - This regsiter stores the PC when trigger stack monitor."]
pub type CORE_0_SP_PC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This regsiter stores the PC when trigger stack monitor."]
    #[inline(always)]
    pub fn core_0_sp_pc(&self) -> CORE_0_SP_PC_R {
        CORE_0_SP_PC_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_SP_PC")
            .field(
                "core_0_sp_pc",
                &format_args!("{}", self.core_0_sp_pc().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_SP_PC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "stack monitor pc status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_sp_pc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_SP_PC_SPEC;
impl crate::RegisterSpec for CORE_0_SP_PC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_sp_pc::R`](R) reader structure"]
impl crate::Readable for CORE_0_SP_PC_SPEC {}
#[doc = "`reset()` method sets CORE_0_SP_PC to value 0"]
impl crate::Resettable for CORE_0_SP_PC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}