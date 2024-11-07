#[doc = "Register `I2C_EXT0_INTR_MAP` reader"]
pub type R = crate::R<I2C_EXT0_INTR_MAP_SPEC>;
#[doc = "Register `I2C_EXT0_INTR_MAP` writer"]
pub type W = crate::W<I2C_EXT0_INTR_MAP_SPEC>;
#[doc = "Field `I2C_EXT0_INTR_MAP` reader - reg_core0_i2c_ext0_intr_map"]
pub type I2C_EXT0_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `I2C_EXT0_INTR_MAP` writer - reg_core0_i2c_ext0_intr_map"]
pub type I2C_EXT0_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - reg_core0_i2c_ext0_intr_map"]
    #[inline(always)]
    pub fn i2c_ext0_intr_map(&self) -> I2C_EXT0_INTR_MAP_R {
        I2C_EXT0_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_EXT0_INTR_MAP")
            .field("i2c_ext0_intr_map", &self.i2c_ext0_intr_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - reg_core0_i2c_ext0_intr_map"]
    #[inline(always)]
    pub fn i2c_ext0_intr_map(&mut self) -> I2C_EXT0_INTR_MAP_W<I2C_EXT0_INTR_MAP_SPEC> {
        I2C_EXT0_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "i2c intr map register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_ext0_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_ext0_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_EXT0_INTR_MAP_SPEC;
impl crate::RegisterSpec for I2C_EXT0_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_ext0_intr_map::R`](R) reader structure"]
impl crate::Readable for I2C_EXT0_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_ext0_intr_map::W`](W) writer structure"]
impl crate::Writable for I2C_EXT0_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_EXT0_INTR_MAP to value 0"]
impl crate::Resettable for I2C_EXT0_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
