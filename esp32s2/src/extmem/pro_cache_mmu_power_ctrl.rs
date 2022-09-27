#[doc = "Register `PRO_CACHE_MMU_POWER_CTRL` reader"]
pub struct R(crate::R<PRO_CACHE_MMU_POWER_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_CACHE_MMU_POWER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_CACHE_MMU_POWER_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_CACHE_MMU_POWER_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_CACHE_MMU_POWER_CTRL` writer"]
pub struct W(crate::W<PRO_CACHE_MMU_POWER_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_CACHE_MMU_POWER_CTRL_SPEC>;
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
impl From<crate::W<PRO_CACHE_MMU_POWER_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_CACHE_MMU_POWER_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_CACHE_MMU_MEM_FORCE_ON` reader - The bit is used to enable clock gating to save power when access mmu memory, 0: enable, 1: disable"]
pub type PRO_CACHE_MMU_MEM_FORCE_ON_R = crate::BitReader<bool>;
#[doc = "Field `PRO_CACHE_MMU_MEM_FORCE_ON` writer - The bit is used to enable clock gating to save power when access mmu memory, 0: enable, 1: disable"]
pub type PRO_CACHE_MMU_MEM_FORCE_ON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRO_CACHE_MMU_POWER_CTRL_SPEC, bool, O>;
#[doc = "Field `PRO_CACHE_MMU_MEM_FORCE_PD` reader - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power down"]
pub type PRO_CACHE_MMU_MEM_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `PRO_CACHE_MMU_MEM_FORCE_PD` writer - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power down"]
pub type PRO_CACHE_MMU_MEM_FORCE_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRO_CACHE_MMU_POWER_CTRL_SPEC, bool, O>;
#[doc = "Field `PRO_CACHE_MMU_MEM_FORCE_PU` reader - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power up"]
pub type PRO_CACHE_MMU_MEM_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `PRO_CACHE_MMU_MEM_FORCE_PU` writer - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power up"]
pub type PRO_CACHE_MMU_MEM_FORCE_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRO_CACHE_MMU_POWER_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable clock gating to save power when access mmu memory, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn pro_cache_mmu_mem_force_on(&self) -> PRO_CACHE_MMU_MEM_FORCE_ON_R {
        PRO_CACHE_MMU_MEM_FORCE_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power down"]
    #[inline(always)]
    pub fn pro_cache_mmu_mem_force_pd(&self) -> PRO_CACHE_MMU_MEM_FORCE_PD_R {
        PRO_CACHE_MMU_MEM_FORCE_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power up"]
    #[inline(always)]
    pub fn pro_cache_mmu_mem_force_pu(&self) -> PRO_CACHE_MMU_MEM_FORCE_PU_R {
        PRO_CACHE_MMU_MEM_FORCE_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable clock gating to save power when access mmu memory, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn pro_cache_mmu_mem_force_on(&mut self) -> PRO_CACHE_MMU_MEM_FORCE_ON_W<0> {
        PRO_CACHE_MMU_MEM_FORCE_ON_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power down"]
    #[inline(always)]
    pub fn pro_cache_mmu_mem_force_pd(&mut self) -> PRO_CACHE_MMU_MEM_FORCE_PD_W<1> {
        PRO_CACHE_MMU_MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 2 - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power up"]
    #[inline(always)]
    pub fn pro_cache_mmu_mem_force_pu(&mut self) -> PRO_CACHE_MMU_MEM_FORCE_PU_W<2> {
        PRO_CACHE_MMU_MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_cache_mmu_power_ctrl](index.html) module"]
pub struct PRO_CACHE_MMU_POWER_CTRL_SPEC;
impl crate::RegisterSpec for PRO_CACHE_MMU_POWER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_cache_mmu_power_ctrl::R](R) reader structure"]
impl crate::Readable for PRO_CACHE_MMU_POWER_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_cache_mmu_power_ctrl::W](W) writer structure"]
impl crate::Writable for PRO_CACHE_MMU_POWER_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_CACHE_MMU_POWER_CTRL to value 0x05"]
impl crate::Resettable for PRO_CACHE_MMU_POWER_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05
    }
}