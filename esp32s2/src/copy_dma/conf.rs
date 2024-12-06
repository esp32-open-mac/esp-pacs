#[doc = "Register `CONF` reader"]
pub type R = crate::R<CONF_SPEC>;
#[doc = "Field `IN_RST` reader - Set this bit to reset in DMA FSM."]
pub type IN_RST_R = crate::BitReader;
#[doc = "Field `OUT_RST` reader - Set this bit to reset out DMA FSM."]
pub type OUT_RST_R = crate::BitReader;
#[doc = "Field `CMDFIFO_RST` reader - Set this bit to reset in_cmd FIFO and out_cmd FIFO."]
pub type CMDFIFO_RST_R = crate::BitReader;
#[doc = "Field `FIFO_RST` reader - Set this bit to reset data in RX FIFO."]
pub type FIFO_RST_R = crate::BitReader;
#[doc = "Field `OUT_OWNER` reader - This is used to configure the owner bit in transmit descriptor. This is effective only when you set OUT_AUTO_WRBACK."]
pub type OUT_OWNER_R = crate::BitReader;
#[doc = "Field `IN_OWNER` reader - This is used to configure the owner bit in receive descriptor."]
pub type IN_OWNER_R = crate::BitReader;
#[doc = "Field `OUT_AUTO_WRBACK` reader - This bit is used to write back out descriptor when hardware has already used this descriptor."]
pub type OUT_AUTO_WRBACK_R = crate::BitReader;
#[doc = "Field `CHECK_OWNER` reader - Set this bit to enable owner bit check in descriptor."]
pub type CHECK_OWNER_R = crate::BitReader;
#[doc = "1’b1: Force clock on for register. 1’b0: Support clock only when application writes registers.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK_EN {
    #[doc = "0: Support clock only when application writes registers"]
    OnWrite = 0,
    #[doc = "1: Force clock on for register"]
    Force = 1,
}
impl From<CLK_EN> for bool {
    #[inline(always)]
    fn from(variant: CLK_EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_EN` reader - 1’b1: Force clock on for register. 1’b0: Support clock only when application writes registers."]
pub type CLK_EN_R = crate::BitReader<CLK_EN>;
impl CLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLK_EN {
        match self.bits {
            false => CLK_EN::OnWrite,
            true => CLK_EN::Force,
        }
    }
    #[doc = "Support clock only when application writes registers"]
    #[inline(always)]
    pub fn is_on_write(&self) -> bool {
        *self == CLK_EN::OnWrite
    }
    #[doc = "Force clock on for register"]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        *self == CLK_EN::Force
    }
}
impl R {
    #[doc = "Bit 0 - Set this bit to reset in DMA FSM."]
    #[inline(always)]
    pub fn in_rst(&self) -> IN_RST_R {
        IN_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to reset out DMA FSM."]
    #[inline(always)]
    pub fn out_rst(&self) -> OUT_RST_R {
        OUT_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to reset in_cmd FIFO and out_cmd FIFO."]
    #[inline(always)]
    pub fn cmdfifo_rst(&self) -> CMDFIFO_RST_R {
        CMDFIFO_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to reset data in RX FIFO."]
    #[inline(always)]
    pub fn fifo_rst(&self) -> FIFO_RST_R {
        FIFO_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is used to configure the owner bit in transmit descriptor. This is effective only when you set OUT_AUTO_WRBACK."]
    #[inline(always)]
    pub fn out_owner(&self) -> OUT_OWNER_R {
        OUT_OWNER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is used to configure the owner bit in receive descriptor."]
    #[inline(always)]
    pub fn in_owner(&self) -> IN_OWNER_R {
        IN_OWNER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is used to write back out descriptor when hardware has already used this descriptor."]
    #[inline(always)]
    pub fn out_auto_wrback(&self) -> OUT_AUTO_WRBACK_R {
        OUT_AUTO_WRBACK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to enable owner bit check in descriptor."]
    #[inline(always)]
    pub fn check_owner(&self) -> CHECK_OWNER_R {
        CHECK_OWNER_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - 1’b1: Force clock on for register. 1’b0: Support clock only when application writes registers."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("in_rst", &self.in_rst())
            .field("out_rst", &self.out_rst())
            .field("cmdfifo_rst", &self.cmdfifo_rst())
            .field("fifo_rst", &self.fifo_rst())
            .field("out_owner", &self.out_owner())
            .field("in_owner", &self.in_owner())
            .field("out_auto_wrback", &self.out_auto_wrback())
            .field("check_owner", &self.check_owner())
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
#[doc = "Copy DMA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for CONF_SPEC {}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
