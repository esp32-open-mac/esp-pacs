#[doc = "Register `HP_SLEEP_LP_DCDC_RESERVE` writer"]
pub type W = crate::W<HP_SLEEP_LP_DCDC_RESERVE_SPEC>;
#[doc = "Field `PMU_HP_SLEEP_LP_DCDC_RESERVE` writer - need_des"]
pub type PMU_HP_SLEEP_LP_DCDC_RESERVE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_SLEEP_LP_DCDC_RESERVE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn pmu_hp_sleep_lp_dcdc_reserve(
        &mut self,
    ) -> PMU_HP_SLEEP_LP_DCDC_RESERVE_W<HP_SLEEP_LP_DCDC_RESERVE_SPEC> {
        PMU_HP_SLEEP_LP_DCDC_RESERVE_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_lp_dcdc_reserve::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_SLEEP_LP_DCDC_RESERVE_SPEC;
impl crate::RegisterSpec for HP_SLEEP_LP_DCDC_RESERVE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_sleep_lp_dcdc_reserve::W`](W) writer structure"]
impl crate::Writable for HP_SLEEP_LP_DCDC_RESERVE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HP_SLEEP_LP_DCDC_RESERVE to value 0"]
impl crate::Resettable for HP_SLEEP_LP_DCDC_RESERVE_SPEC {
    const RESET_VALUE: u32 = 0;
}
