#[doc = "Register `L2_CACHE_PRELOAD_SIZE` reader"]
pub struct R(crate::R<L2_CACHE_PRELOAD_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_CACHE_PRELOAD_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_CACHE_PRELOAD_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_CACHE_PRELOAD_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_CACHE_PRELOAD_SIZE` reader - Those bits are used to configure the size of the first section of prelock on L2-Cache, which should be used together with L2_CACHE_PRELOAD_ADDR_REG"]
pub type L2_CACHE_PRELOAD_SIZE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Those bits are used to configure the size of the first section of prelock on L2-Cache, which should be used together with L2_CACHE_PRELOAD_ADDR_REG"]
    #[inline(always)]
    pub fn l2_cache_preload_size(&self) -> L2_CACHE_PRELOAD_SIZE_R {
        L2_CACHE_PRELOAD_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "L2 Cache preload size configure register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_cache_preload_size](index.html) module"]
pub struct L2_CACHE_PRELOAD_SIZE_SPEC;
impl crate::RegisterSpec for L2_CACHE_PRELOAD_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_cache_preload_size::R](R) reader structure"]
impl crate::Readable for L2_CACHE_PRELOAD_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_CACHE_PRELOAD_SIZE to value 0"]
impl crate::Resettable for L2_CACHE_PRELOAD_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}