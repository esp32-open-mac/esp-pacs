#[doc = "Register `RSA_PD_CTRL` reader"]
pub struct R(crate::R<RSA_PD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSA_PD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSA_PD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSA_PD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSA_PD_CTRL` writer"]
pub struct W(crate::W<RSA_PD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSA_PD_CTRL_SPEC>;
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
impl From<crate::W<RSA_PD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSA_PD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSA_MEM_PD` reader - reg_rsa_mem_pd"]
pub type RSA_MEM_PD_R = crate::BitReader<bool>;
#[doc = "Field `RSA_MEM_PD` writer - reg_rsa_mem_pd"]
pub type RSA_MEM_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSA_PD_CTRL_SPEC, bool, O>;
#[doc = "Field `RSA_MEM_FORCE_PU` reader - reg_rsa_mem_force_pu"]
pub type RSA_MEM_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `RSA_MEM_FORCE_PU` writer - reg_rsa_mem_force_pu"]
pub type RSA_MEM_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSA_PD_CTRL_SPEC, bool, O>;
#[doc = "Field `RSA_MEM_FORCE_PD` reader - reg_rsa_mem_force_pd"]
pub type RSA_MEM_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `RSA_MEM_FORCE_PD` writer - reg_rsa_mem_force_pd"]
pub type RSA_MEM_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSA_PD_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - reg_rsa_mem_pd"]
    #[inline(always)]
    pub fn rsa_mem_pd(&self) -> RSA_MEM_PD_R {
        RSA_MEM_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_rsa_mem_force_pu"]
    #[inline(always)]
    pub fn rsa_mem_force_pu(&self) -> RSA_MEM_FORCE_PU_R {
        RSA_MEM_FORCE_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reg_rsa_mem_force_pd"]
    #[inline(always)]
    pub fn rsa_mem_force_pd(&self) -> RSA_MEM_FORCE_PD_R {
        RSA_MEM_FORCE_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reg_rsa_mem_pd"]
    #[inline(always)]
    pub fn rsa_mem_pd(&mut self) -> RSA_MEM_PD_W<0> {
        RSA_MEM_PD_W::new(self)
    }
    #[doc = "Bit 1 - reg_rsa_mem_force_pu"]
    #[inline(always)]
    pub fn rsa_mem_force_pu(&mut self) -> RSA_MEM_FORCE_PU_W<1> {
        RSA_MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 2 - reg_rsa_mem_force_pd"]
    #[inline(always)]
    pub fn rsa_mem_force_pd(&mut self) -> RSA_MEM_FORCE_PD_W<2> {
        RSA_MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rsa memory power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsa_pd_ctrl](index.html) module"]
pub struct RSA_PD_CTRL_SPEC;
impl crate::RegisterSpec for RSA_PD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsa_pd_ctrl::R](R) reader structure"]
impl crate::Readable for RSA_PD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsa_pd_ctrl::W](W) writer structure"]
impl crate::Writable for RSA_PD_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSA_PD_CTRL to value 0x01"]
impl crate::Resettable for RSA_PD_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}