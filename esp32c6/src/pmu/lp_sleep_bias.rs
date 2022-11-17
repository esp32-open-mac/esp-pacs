#[doc = "Register `LP_SLEEP_BIAS` reader"]
pub struct R(crate::R<LP_SLEEP_BIAS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LP_SLEEP_BIAS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LP_SLEEP_BIAS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LP_SLEEP_BIAS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LP_SLEEP_BIAS` writer"]
pub struct W(crate::W<LP_SLEEP_BIAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LP_SLEEP_BIAS_SPEC>;
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
impl From<crate::W<LP_SLEEP_BIAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LP_SLEEP_BIAS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_SLEEP_XPD_BIAS` reader - need_des"]
pub type LP_SLEEP_XPD_BIAS_R = crate::BitReader<bool>;
#[doc = "Field `LP_SLEEP_XPD_BIAS` writer - need_des"]
pub type LP_SLEEP_XPD_BIAS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_SLEEP_BIAS_SPEC, bool, O>;
#[doc = "Field `LP_SLEEP_DBG_ATTEN` reader - need_des"]
pub type LP_SLEEP_DBG_ATTEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LP_SLEEP_DBG_ATTEN` writer - need_des"]
pub type LP_SLEEP_DBG_ATTEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LP_SLEEP_BIAS_SPEC, u8, u8, 4, O>;
#[doc = "Field `LP_SLEEP_PD_CUR` reader - need_des"]
pub type LP_SLEEP_PD_CUR_R = crate::BitReader<bool>;
#[doc = "Field `LP_SLEEP_PD_CUR` writer - need_des"]
pub type LP_SLEEP_PD_CUR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_SLEEP_BIAS_SPEC, bool, O>;
#[doc = "Field `SLEEP` reader - need_des"]
pub type SLEEP_R = crate::BitReader<bool>;
#[doc = "Field `SLEEP` writer - need_des"]
pub type SLEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, LP_SLEEP_BIAS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_xpd_bias(&self) -> LP_SLEEP_XPD_BIAS_R {
        LP_SLEEP_XPD_BIAS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:29 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_dbg_atten(&self) -> LP_SLEEP_DBG_ATTEN_R {
        LP_SLEEP_DBG_ATTEN_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_pd_cur(&self) -> LP_SLEEP_PD_CUR_R {
        LP_SLEEP_PD_CUR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_sleep_xpd_bias(&mut self) -> LP_SLEEP_XPD_BIAS_W<25> {
        LP_SLEEP_XPD_BIAS_W::new(self)
    }
    #[doc = "Bits 26:29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_sleep_dbg_atten(&mut self) -> LP_SLEEP_DBG_ATTEN_W<26> {
        LP_SLEEP_DBG_ATTEN_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_sleep_pd_cur(&mut self) -> LP_SLEEP_PD_CUR_W<30> {
        LP_SLEEP_PD_CUR_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<31> {
        SLEEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lp_sleep_bias](index.html) module"]
pub struct LP_SLEEP_BIAS_SPEC;
impl crate::RegisterSpec for LP_SLEEP_BIAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lp_sleep_bias::R](R) reader structure"]
impl crate::Readable for LP_SLEEP_BIAS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lp_sleep_bias::W](W) writer structure"]
impl crate::Writable for LP_SLEEP_BIAS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_SLEEP_BIAS to value 0"]
impl crate::Resettable for LP_SLEEP_BIAS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}