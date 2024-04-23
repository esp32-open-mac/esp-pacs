#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `TX_DATA_BUF_THLD` reader - Transmit Buffer threshold status enable."]
pub type TX_DATA_BUF_THLD_R = crate::BitReader;
#[doc = "Field `TX_DATA_BUF_THLD` writer - Transmit Buffer threshold status enable."]
pub type TX_DATA_BUF_THLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DATA_BUF_THLD` reader - Receive Buffer threshold status enable."]
pub type RX_DATA_BUF_THLD_R = crate::BitReader;
#[doc = "Field `RX_DATA_BUF_THLD` writer - Receive Buffer threshold status enable."]
pub type RX_DATA_BUF_THLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBI_STATUS_THLD` reader - Only used in master mode. IBI Buffer threshold status enable."]
pub type IBI_STATUS_THLD_R = crate::BitReader;
#[doc = "Field `IBI_STATUS_THLD` writer - Only used in master mode. IBI Buffer threshold status enable."]
pub type IBI_STATUS_THLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_BUF_EMPTY_THLD` reader - Command buffer ready status enable."]
pub type CMD_BUF_EMPTY_THLD_R = crate::BitReader;
#[doc = "Field `CMD_BUF_EMPTY_THLD` writer - Command buffer ready status enable."]
pub type CMD_BUF_EMPTY_THLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP_READY` reader - Response buffer ready status enable."]
pub type RESP_READY_R = crate::BitReader;
#[doc = "Field `RESP_READY` writer - Response buffer ready status enable."]
pub type RESP_READY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NXT_CMD_REQ_ERR` reader - next command request error status enable"]
pub type NXT_CMD_REQ_ERR_R = crate::BitReader;
#[doc = "Field `NXT_CMD_REQ_ERR` writer - next command request error status enable"]
pub type NXT_CMD_REQ_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSFER_ERR` reader - Transfer error status enable"]
pub type TRANSFER_ERR_R = crate::BitReader;
#[doc = "Field `TRANSFER_ERR` writer - Transfer error status enable"]
pub type TRANSFER_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSFER_COMPLETE` reader - NA"]
pub type TRANSFER_COMPLETE_R = crate::BitReader;
#[doc = "Field `TRANSFER_COMPLETE` writer - NA"]
pub type TRANSFER_COMPLETE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMMAND_DONE` reader - NA"]
pub type COMMAND_DONE_R = crate::BitReader;
#[doc = "Field `COMMAND_DONE` writer - NA"]
pub type COMMAND_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DETECT_START` reader - NA"]
pub type DETECT_START_R = crate::BitReader;
#[doc = "Field `DETECT_START` writer - NA"]
pub type DETECT_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP_BUF_OVF` reader - NA"]
pub type RESP_BUF_OVF_R = crate::BitReader;
#[doc = "Field `RESP_BUF_OVF` writer - NA"]
pub type RESP_BUF_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBI_DATA_BUF_OVF` reader - NA"]
pub type IBI_DATA_BUF_OVF_R = crate::BitReader;
#[doc = "Field `IBI_DATA_BUF_OVF` writer - NA"]
pub type IBI_DATA_BUF_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBI_STATUS_BUF_OVF` reader - NA"]
pub type IBI_STATUS_BUF_OVF_R = crate::BitReader;
#[doc = "Field `IBI_STATUS_BUF_OVF` writer - NA"]
pub type IBI_STATUS_BUF_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBI_HANDLE_DONE` reader - NA"]
pub type IBI_HANDLE_DONE_R = crate::BitReader;
#[doc = "Field `IBI_HANDLE_DONE` writer - NA"]
pub type IBI_HANDLE_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBI_DETECT` reader - NA"]
pub type IBI_DETECT_R = crate::BitReader;
#[doc = "Field `IBI_DETECT` writer - NA"]
pub type IBI_DETECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_CCC_MISMATCH` reader - NA"]
pub type CMD_CCC_MISMATCH_R = crate::BitReader;
#[doc = "Field `CMD_CCC_MISMATCH` writer - NA"]
pub type CMD_CCC_MISMATCH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit Buffer threshold status enable."]
    #[inline(always)]
    pub fn tx_data_buf_thld(&self) -> TX_DATA_BUF_THLD_R {
        TX_DATA_BUF_THLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Buffer threshold status enable."]
    #[inline(always)]
    pub fn rx_data_buf_thld(&self) -> RX_DATA_BUF_THLD_R {
        RX_DATA_BUF_THLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Only used in master mode. IBI Buffer threshold status enable."]
    #[inline(always)]
    pub fn ibi_status_thld(&self) -> IBI_STATUS_THLD_R {
        IBI_STATUS_THLD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Command buffer ready status enable."]
    #[inline(always)]
    pub fn cmd_buf_empty_thld(&self) -> CMD_BUF_EMPTY_THLD_R {
        CMD_BUF_EMPTY_THLD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Response buffer ready status enable."]
    #[inline(always)]
    pub fn resp_ready(&self) -> RESP_READY_R {
        RESP_READY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - next command request error status enable"]
    #[inline(always)]
    pub fn nxt_cmd_req_err(&self) -> NXT_CMD_REQ_ERR_R {
        NXT_CMD_REQ_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer error status enable"]
    #[inline(always)]
    pub fn transfer_err(&self) -> TRANSFER_ERR_R {
        TRANSFER_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn transfer_complete(&self) -> TRANSFER_COMPLETE_R {
        TRANSFER_COMPLETE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn command_done(&self) -> COMMAND_DONE_R {
        COMMAND_DONE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn detect_start(&self) -> DETECT_START_R {
        DETECT_START_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn resp_buf_ovf(&self) -> RESP_BUF_OVF_R {
        RESP_BUF_OVF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn ibi_data_buf_ovf(&self) -> IBI_DATA_BUF_OVF_R {
        IBI_DATA_BUF_OVF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn ibi_status_buf_ovf(&self) -> IBI_STATUS_BUF_OVF_R {
        IBI_STATUS_BUF_OVF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn ibi_handle_done(&self) -> IBI_HANDLE_DONE_R {
        IBI_HANDLE_DONE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn ibi_detect(&self) -> IBI_DETECT_R {
        IBI_DETECT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn cmd_ccc_mismatch(&self) -> CMD_CCC_MISMATCH_R {
        CMD_CCC_MISMATCH_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "tx_data_buf_thld",
                &format_args!("{}", self.tx_data_buf_thld().bit()),
            )
            .field(
                "rx_data_buf_thld",
                &format_args!("{}", self.rx_data_buf_thld().bit()),
            )
            .field(
                "ibi_status_thld",
                &format_args!("{}", self.ibi_status_thld().bit()),
            )
            .field(
                "cmd_buf_empty_thld",
                &format_args!("{}", self.cmd_buf_empty_thld().bit()),
            )
            .field("resp_ready", &format_args!("{}", self.resp_ready().bit()))
            .field(
                "nxt_cmd_req_err",
                &format_args!("{}", self.nxt_cmd_req_err().bit()),
            )
            .field(
                "transfer_err",
                &format_args!("{}", self.transfer_err().bit()),
            )
            .field(
                "transfer_complete",
                &format_args!("{}", self.transfer_complete().bit()),
            )
            .field(
                "command_done",
                &format_args!("{}", self.command_done().bit()),
            )
            .field(
                "detect_start",
                &format_args!("{}", self.detect_start().bit()),
            )
            .field(
                "resp_buf_ovf",
                &format_args!("{}", self.resp_buf_ovf().bit()),
            )
            .field(
                "ibi_data_buf_ovf",
                &format_args!("{}", self.ibi_data_buf_ovf().bit()),
            )
            .field(
                "ibi_status_buf_ovf",
                &format_args!("{}", self.ibi_status_buf_ovf().bit()),
            )
            .field(
                "ibi_handle_done",
                &format_args!("{}", self.ibi_handle_done().bit()),
            )
            .field("ibi_detect", &format_args!("{}", self.ibi_detect().bit()))
            .field(
                "cmd_ccc_mismatch",
                &format_args!("{}", self.cmd_ccc_mismatch().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Buffer threshold status enable."]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_buf_thld(&mut self) -> TX_DATA_BUF_THLD_W<INT_ENA_SPEC> {
        TX_DATA_BUF_THLD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Buffer threshold status enable."]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_buf_thld(&mut self) -> RX_DATA_BUF_THLD_W<INT_ENA_SPEC> {
        RX_DATA_BUF_THLD_W::new(self, 1)
    }
    #[doc = "Bit 2 - Only used in master mode. IBI Buffer threshold status enable."]
    #[inline(always)]
    #[must_use]
    pub fn ibi_status_thld(&mut self) -> IBI_STATUS_THLD_W<INT_ENA_SPEC> {
        IBI_STATUS_THLD_W::new(self, 2)
    }
    #[doc = "Bit 3 - Command buffer ready status enable."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_buf_empty_thld(&mut self) -> CMD_BUF_EMPTY_THLD_W<INT_ENA_SPEC> {
        CMD_BUF_EMPTY_THLD_W::new(self, 3)
    }
    #[doc = "Bit 4 - Response buffer ready status enable."]
    #[inline(always)]
    #[must_use]
    pub fn resp_ready(&mut self) -> RESP_READY_W<INT_ENA_SPEC> {
        RESP_READY_W::new(self, 4)
    }
    #[doc = "Bit 5 - next command request error status enable"]
    #[inline(always)]
    #[must_use]
    pub fn nxt_cmd_req_err(&mut self) -> NXT_CMD_REQ_ERR_W<INT_ENA_SPEC> {
        NXT_CMD_REQ_ERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transfer error status enable"]
    #[inline(always)]
    #[must_use]
    pub fn transfer_err(&mut self) -> TRANSFER_ERR_W<INT_ENA_SPEC> {
        TRANSFER_ERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn transfer_complete(&mut self) -> TRANSFER_COMPLETE_W<INT_ENA_SPEC> {
        TRANSFER_COMPLETE_W::new(self, 7)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn command_done(&mut self) -> COMMAND_DONE_W<INT_ENA_SPEC> {
        COMMAND_DONE_W::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn detect_start(&mut self) -> DETECT_START_W<INT_ENA_SPEC> {
        DETECT_START_W::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn resp_buf_ovf(&mut self) -> RESP_BUF_OVF_W<INT_ENA_SPEC> {
        RESP_BUF_OVF_W::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ibi_data_buf_ovf(&mut self) -> IBI_DATA_BUF_OVF_W<INT_ENA_SPEC> {
        IBI_DATA_BUF_OVF_W::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ibi_status_buf_ovf(&mut self) -> IBI_STATUS_BUF_OVF_W<INT_ENA_SPEC> {
        IBI_STATUS_BUF_OVF_W::new(self, 12)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ibi_handle_done(&mut self) -> IBI_HANDLE_DONE_W<INT_ENA_SPEC> {
        IBI_HANDLE_DONE_W::new(self, 13)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ibi_detect(&mut self) -> IBI_DETECT_W<INT_ENA_SPEC> {
        IBI_DETECT_W::new(self, 14)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_ccc_mismatch(&mut self) -> CMD_CCC_MISMATCH_W<INT_ENA_SPEC> {
        CMD_CCC_MISMATCH_W::new(self, 15)
    }
}
#[doc = "The Interrupt status will be updated in INTR_STATUS register if corresponding Status Enable bit set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
