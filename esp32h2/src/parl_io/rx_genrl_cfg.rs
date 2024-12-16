#[doc = "Register `RX_GENRL_CFG` reader"]
pub type R = crate::R<RX_GENRL_CFG_SPEC>;
#[doc = "Register `RX_GENRL_CFG` writer"]
pub type W = crate::W<RX_GENRL_CFG_SPEC>;
#[doc = "Field `RX_GATING_EN` reader - Set this bit to enable the clock gating of output rx clock."]
pub type RX_GATING_EN_R = crate::BitReader;
#[doc = "Field `RX_GATING_EN` writer - Set this bit to enable the clock gating of output rx clock."]
pub type RX_GATING_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_TIMEOUT_THRES` reader - Configures threshold of timeout counter."]
pub type RX_TIMEOUT_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `RX_TIMEOUT_THRES` writer - Configures threshold of timeout counter."]
pub type RX_TIMEOUT_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RX_TIMEOUT_EN` reader - Set this bit to enable timeout function to generate error eof."]
pub type RX_TIMEOUT_EN_R = crate::BitReader;
#[doc = "Field `RX_TIMEOUT_EN` writer - Set this bit to enable timeout function to generate error eof."]
pub type RX_TIMEOUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_EOF_GEN_SEL` reader - Configures the DMA eof generated mechanism. 1'b0: eof generated by data byte length. 1'b1: eof generated by external enable signal."]
pub type RX_EOF_GEN_SEL_R = crate::BitReader;
#[doc = "Field `RX_EOF_GEN_SEL` writer - Configures the DMA eof generated mechanism. 1'b0: eof generated by data byte length. 1'b1: eof generated by external enable signal."]
pub type RX_EOF_GEN_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12 - Set this bit to enable the clock gating of output rx clock."]
    #[inline(always)]
    pub fn rx_gating_en(&self) -> RX_GATING_EN_R {
        RX_GATING_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:28 - Configures threshold of timeout counter."]
    #[inline(always)]
    pub fn rx_timeout_thres(&self) -> RX_TIMEOUT_THRES_R {
        RX_TIMEOUT_THRES_R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    #[doc = "Bit 29 - Set this bit to enable timeout function to generate error eof."]
    #[inline(always)]
    pub fn rx_timeout_en(&self) -> RX_TIMEOUT_EN_R {
        RX_TIMEOUT_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Configures the DMA eof generated mechanism. 1'b0: eof generated by data byte length. 1'b1: eof generated by external enable signal."]
    #[inline(always)]
    pub fn rx_eof_gen_sel(&self) -> RX_EOF_GEN_SEL_R {
        RX_EOF_GEN_SEL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_GENRL_CFG")
            .field("rx_gating_en", &self.rx_gating_en())
            .field("rx_timeout_thres", &self.rx_timeout_thres())
            .field("rx_timeout_en", &self.rx_timeout_en())
            .field("rx_eof_gen_sel", &self.rx_eof_gen_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 12 - Set this bit to enable the clock gating of output rx clock."]
    #[inline(always)]
    pub fn rx_gating_en(&mut self) -> RX_GATING_EN_W<RX_GENRL_CFG_SPEC> {
        RX_GATING_EN_W::new(self, 12)
    }
    #[doc = "Bits 13:28 - Configures threshold of timeout counter."]
    #[inline(always)]
    pub fn rx_timeout_thres(&mut self) -> RX_TIMEOUT_THRES_W<RX_GENRL_CFG_SPEC> {
        RX_TIMEOUT_THRES_W::new(self, 13)
    }
    #[doc = "Bit 29 - Set this bit to enable timeout function to generate error eof."]
    #[inline(always)]
    pub fn rx_timeout_en(&mut self) -> RX_TIMEOUT_EN_W<RX_GENRL_CFG_SPEC> {
        RX_TIMEOUT_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - Configures the DMA eof generated mechanism. 1'b0: eof generated by data byte length. 1'b1: eof generated by external enable signal."]
    #[inline(always)]
    pub fn rx_eof_gen_sel(&mut self) -> RX_EOF_GEN_SEL_W<RX_GENRL_CFG_SPEC> {
        RX_EOF_GEN_SEL_W::new(self, 30)
    }
}
#[doc = "Parallel RX general configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_genrl_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_genrl_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_GENRL_CFG_SPEC;
impl crate::RegisterSpec for RX_GENRL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_genrl_cfg::R`](R) reader structure"]
impl crate::Readable for RX_GENRL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_genrl_cfg::W`](W) writer structure"]
impl crate::Writable for RX_GENRL_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_GENRL_CFG to value 0x21ff_e000"]
impl crate::Resettable for RX_GENRL_CFG_SPEC {
    const RESET_VALUE: u32 = 0x21ff_e000;
}
