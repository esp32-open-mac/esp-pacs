#[doc = "Register `OUT_DSCR_CH%s` reader"]
pub type R = crate::R<OUT_DSCR_CH_SPEC>;
#[doc = "Field `OUTLINK_DSCR_CH` reader - The address of the current outlink descriptor y."]
pub type OUTLINK_DSCR_CH_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The address of the current outlink descriptor y."]
    #[inline(always)]
    pub fn outlink_dscr_ch(&self) -> OUTLINK_DSCR_CH_R {
        OUTLINK_DSCR_CH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_DSCR_CH")
            .field(
                "outlink_dscr_ch",
                &format_args!("{}", self.outlink_dscr_ch().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_DSCR_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Current inlink descriptor address of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_ch::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_DSCR_CH_SPEC;
impl crate::RegisterSpec for OUT_DSCR_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_dscr_ch::R`](R) reader structure"]
impl crate::Readable for OUT_DSCR_CH_SPEC {}
#[doc = "`reset()` method sets OUT_DSCR_CH%s to value 0"]
impl crate::Resettable for OUT_DSCR_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}