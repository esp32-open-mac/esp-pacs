#[doc = "Register `DCACHE_LOCK_ADDR` reader"]
pub type R = crate::R<DCACHE_LOCK_ADDR_SPEC>;
#[doc = "Register `DCACHE_LOCK_ADDR` writer"]
pub type W = crate::W<DCACHE_LOCK_ADDR_SPEC>;
#[doc = "Field `DCACHE_LOCK_ADDR` reader - The bits are used to configure the start virtual address for lock operations. It should be combined with DCACHE_LOCK_SIZE_REG."]
pub type DCACHE_LOCK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DCACHE_LOCK_ADDR` writer - The bits are used to configure the start virtual address for lock operations. It should be combined with DCACHE_LOCK_SIZE_REG."]
pub type DCACHE_LOCK_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address for lock operations. It should be combined with DCACHE_LOCK_SIZE_REG."]
    #[inline(always)]
    pub fn dcache_lock_addr(&self) -> DCACHE_LOCK_ADDR_R {
        DCACHE_LOCK_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_LOCK_ADDR")
            .field("dcache_lock_addr", &self.dcache_lock_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address for lock operations. It should be combined with DCACHE_LOCK_SIZE_REG."]
    #[inline(always)]
    pub fn dcache_lock_addr(&mut self) -> DCACHE_LOCK_ADDR_W<DCACHE_LOCK_ADDR_SPEC> {
        DCACHE_LOCK_ADDR_W::new(self, 0)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_lock_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_lock_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_LOCK_ADDR_SPEC;
impl crate::RegisterSpec for DCACHE_LOCK_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_lock_addr::R`](R) reader structure"]
impl crate::Readable for DCACHE_LOCK_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcache_lock_addr::W`](W) writer structure"]
impl crate::Writable for DCACHE_LOCK_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCACHE_LOCK_ADDR to value 0"]
impl crate::Resettable for DCACHE_LOCK_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
