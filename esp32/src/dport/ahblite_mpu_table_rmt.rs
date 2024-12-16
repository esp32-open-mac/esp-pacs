#[doc = "Register `AHBLITE_MPU_TABLE_RMT` reader"]
pub type R = crate::R<AHBLITE_MPU_TABLE_RMT_SPEC>;
#[doc = "Register `AHBLITE_MPU_TABLE_RMT` writer"]
pub type W = crate::W<AHBLITE_MPU_TABLE_RMT_SPEC>;
#[doc = "Field `RMT_ACCESS_GRANT_CONFIG` reader - "]
pub type RMT_ACCESS_GRANT_CONFIG_R = crate::FieldReader;
#[doc = "Field `RMT_ACCESS_GRANT_CONFIG` writer - "]
pub type RMT_ACCESS_GRANT_CONFIG_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rmt_access_grant_config(&self) -> RMT_ACCESS_GRANT_CONFIG_R {
        RMT_ACCESS_GRANT_CONFIG_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBLITE_MPU_TABLE_RMT")
            .field("rmt_access_grant_config", &self.rmt_access_grant_config())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rmt_access_grant_config(
        &mut self,
    ) -> RMT_ACCESS_GRANT_CONFIG_W<AHBLITE_MPU_TABLE_RMT_SPEC> {
        RMT_ACCESS_GRANT_CONFIG_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ahblite_mpu_table_rmt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahblite_mpu_table_rmt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBLITE_MPU_TABLE_RMT_SPEC;
impl crate::RegisterSpec for AHBLITE_MPU_TABLE_RMT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahblite_mpu_table_rmt::R`](R) reader structure"]
impl crate::Readable for AHBLITE_MPU_TABLE_RMT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahblite_mpu_table_rmt::W`](W) writer structure"]
impl crate::Writable for AHBLITE_MPU_TABLE_RMT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBLITE_MPU_TABLE_RMT to value 0"]
impl crate::Resettable for AHBLITE_MPU_TABLE_RMT_SPEC {
    const RESET_VALUE: u32 = 0;
}
