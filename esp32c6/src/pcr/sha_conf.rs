#[doc = "Register `SHA_CONF` reader"]
pub struct R(crate::R<SHA_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHA_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHA_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHA_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHA_CONF` writer"]
pub struct W(crate::W<SHA_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHA_CONF_SPEC>;
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
impl From<crate::W<SHA_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHA_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHA_CLK_EN` reader - Set 1 to enable sha clock"]
pub type SHA_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `SHA_CLK_EN` writer - Set 1 to enable sha clock"]
pub type SHA_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHA_CONF_SPEC, bool, O>;
#[doc = "Field `SHA_RST_EN` reader - Set 0 to reset sha module"]
pub type SHA_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `SHA_RST_EN` writer - Set 0 to reset sha module"]
pub type SHA_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHA_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable sha clock"]
    #[inline(always)]
    pub fn sha_clk_en(&self) -> SHA_CLK_EN_R {
        SHA_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset sha module"]
    #[inline(always)]
    pub fn sha_rst_en(&self) -> SHA_RST_EN_R {
        SHA_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable sha clock"]
    #[inline(always)]
    #[must_use]
    pub fn sha_clk_en(&mut self) -> SHA_CLK_EN_W<0> {
        SHA_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset sha module"]
    #[inline(always)]
    #[must_use]
    pub fn sha_rst_en(&mut self) -> SHA_RST_EN_W<1> {
        SHA_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SHA configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sha_conf](index.html) module"]
pub struct SHA_CONF_SPEC;
impl crate::RegisterSpec for SHA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sha_conf::R](R) reader structure"]
impl crate::Readable for SHA_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sha_conf::W](W) writer structure"]
impl crate::Writable for SHA_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHA_CONF to value 0x01"]
impl crate::Resettable for SHA_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}