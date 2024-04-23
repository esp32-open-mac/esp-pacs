#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `DONE` reader - This enable interrupt bit turns to high level when JPEG finishes encoding a picture.."]
pub type DONE_R = crate::BitReader;
#[doc = "Field `DONE` writer - This enable interrupt bit turns to high level when JPEG finishes encoding a picture.."]
pub type DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RLE_PARALLEL_ERR` reader - The enable interrupt bit to sign that rle parallel error when decoding."]
pub type RLE_PARALLEL_ERR_R = crate::BitReader;
#[doc = "Field `RLE_PARALLEL_ERR` writer - The enable interrupt bit to sign that rle parallel error when decoding."]
pub type RLE_PARALLEL_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CID_ERR` reader - The enable interrupt bit to sign that scan id check with component fails when decoding."]
pub type CID_ERR_R = crate::BitReader;
#[doc = "Field `CID_ERR` writer - The enable interrupt bit to sign that scan id check with component fails when decoding."]
pub type CID_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_DHT_DC_ID_ERR` reader - The enable interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding."]
pub type C_DHT_DC_ID_ERR_R = crate::BitReader;
#[doc = "Field `C_DHT_DC_ID_ERR` writer - The enable interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding."]
pub type C_DHT_DC_ID_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_DHT_AC_ID_ERR` reader - The enable interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding."]
pub type C_DHT_AC_ID_ERR_R = crate::BitReader;
#[doc = "Field `C_DHT_AC_ID_ERR` writer - The enable interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding."]
pub type C_DHT_AC_ID_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_DQT_ID_ERR` reader - The enable interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding."]
pub type C_DQT_ID_ERR_R = crate::BitReader;
#[doc = "Field `C_DQT_ID_ERR` writer - The enable interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding."]
pub type C_DQT_ID_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_UXP_ERR` reader - The enable interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding."]
pub type RST_UXP_ERR_R = crate::BitReader;
#[doc = "Field `RST_UXP_ERR` writer - The enable interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding."]
pub type RST_UXP_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_CHECK_NONE_ERR` reader - The enable interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding."]
pub type RST_CHECK_NONE_ERR_R = crate::BitReader;
#[doc = "Field `RST_CHECK_NONE_ERR` writer - The enable interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding."]
pub type RST_CHECK_NONE_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_CHECK_POS_ERR` reader - The enable interrupt bit to sign that RST header marker position mismatches with restart interval when decoding."]
pub type RST_CHECK_POS_ERR_R = crate::BitReader;
#[doc = "Field `RST_CHECK_POS_ERR` writer - The enable interrupt bit to sign that RST header marker position mismatches with restart interval when decoding."]
pub type RST_CHECK_POS_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF` reader - The enable interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel."]
pub type OUT_EOF_R = crate::BitReader;
#[doc = "Field `OUT_EOF` writer - The enable interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel."]
pub type OUT_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SR_COLOR_MODE_ERR` reader - The enable interrupt bit to sign that the selected source color mode is not supported."]
pub type SR_COLOR_MODE_ERR_R = crate::BitReader;
#[doc = "Field `SR_COLOR_MODE_ERR` writer - The enable interrupt bit to sign that the selected source color mode is not supported."]
pub type SR_COLOR_MODE_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCT_DONE` reader - The enable interrupt bit to sign that one dct calculation is finished."]
pub type DCT_DONE_R = crate::BitReader;
#[doc = "Field `DCT_DONE` writer - The enable interrupt bit to sign that one dct calculation is finished."]
pub type DCT_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS_LAST_BLOCK_EOF` reader - The enable interrupt bit to sign that the coding process for last block is finished."]
pub type BS_LAST_BLOCK_EOF_R = crate::BitReader;
#[doc = "Field `BS_LAST_BLOCK_EOF` writer - The enable interrupt bit to sign that the coding process for last block is finished."]
pub type BS_LAST_BLOCK_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN_CHECK_NONE_ERR` reader - The enable interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded."]
pub type SCAN_CHECK_NONE_ERR_R = crate::BitReader;
#[doc = "Field `SCAN_CHECK_NONE_ERR` writer - The enable interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded."]
pub type SCAN_CHECK_NONE_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN_CHECK_POS_ERR` reader - The enable interrupt bit to sign that SOS header marker position wrong when decoding."]
pub type SCAN_CHECK_POS_ERR_R = crate::BitReader;
#[doc = "Field `SCAN_CHECK_POS_ERR` writer - The enable interrupt bit to sign that SOS header marker position wrong when decoding."]
pub type SCAN_CHECK_POS_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UXP_DET` reader - The enable interrupt bit to sign that unsupported header marker is detected when decoding."]
pub type UXP_DET_R = crate::BitReader;
#[doc = "Field `UXP_DET` writer - The enable interrupt bit to sign that unsupported header marker is detected when decoding."]
pub type UXP_DET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_FRAME_EOF_ERR` reader - The enable interrupt bit to sign that received pixel blocks are smaller than expected when encoding."]
pub type EN_FRAME_EOF_ERR_R = crate::BitReader;
#[doc = "Field `EN_FRAME_EOF_ERR` writer - The enable interrupt bit to sign that received pixel blocks are smaller than expected when encoding."]
pub type EN_FRAME_EOF_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_FRAME_EOF_LACK` reader - The enable interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough."]
pub type EN_FRAME_EOF_LACK_R = crate::BitReader;
#[doc = "Field `EN_FRAME_EOF_LACK` writer - The enable interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough."]
pub type EN_FRAME_EOF_LACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DE_FRAME_EOF_ERR` reader - The enable interrupt bit to sign that decoded blocks are smaller than expected when decoding."]
pub type DE_FRAME_EOF_ERR_R = crate::BitReader;
#[doc = "Field `DE_FRAME_EOF_ERR` writer - The enable interrupt bit to sign that decoded blocks are smaller than expected when decoding."]
pub type DE_FRAME_EOF_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DE_FRAME_EOF_LACK` reader - The enable interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough."]
pub type DE_FRAME_EOF_LACK_R = crate::BitReader;
#[doc = "Field `DE_FRAME_EOF_LACK` writer - The enable interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough."]
pub type DE_FRAME_EOF_LACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOS_UNMATCH_ERR` reader - The enable interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding."]
pub type SOS_UNMATCH_ERR_R = crate::BitReader;
#[doc = "Field `SOS_UNMATCH_ERR` writer - The enable interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding."]
pub type SOS_UNMATCH_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MARKER_ERR_FST_SCAN` reader - The enable interrupt bit to sign that the first scan has header marker error when decoding."]
pub type MARKER_ERR_FST_SCAN_R = crate::BitReader;
#[doc = "Field `MARKER_ERR_FST_SCAN` writer - The enable interrupt bit to sign that the first scan has header marker error when decoding."]
pub type MARKER_ERR_FST_SCAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MARKER_ERR_OTHER_SCAN` reader - The enable interrupt bit to sign that the following scans but not the first scan have header marker error when decoding."]
pub type MARKER_ERR_OTHER_SCAN_R = crate::BitReader;
#[doc = "Field `MARKER_ERR_OTHER_SCAN` writer - The enable interrupt bit to sign that the following scans but not the first scan have header marker error when decoding."]
pub type MARKER_ERR_OTHER_SCAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDET` reader - The enable interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding."]
pub type UNDET_R = crate::BitReader;
#[doc = "Field `UNDET` writer - The enable interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding."]
pub type UNDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DECODE_TIMEOUT` reader - The enable interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding."]
pub type DECODE_TIMEOUT_R = crate::BitReader;
#[doc = "Field `DECODE_TIMEOUT` writer - The enable interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding."]
pub type DECODE_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This enable interrupt bit turns to high level when JPEG finishes encoding a picture.."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable interrupt bit to sign that rle parallel error when decoding."]
    #[inline(always)]
    pub fn rle_parallel_err(&self) -> RLE_PARALLEL_ERR_R {
        RLE_PARALLEL_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The enable interrupt bit to sign that scan id check with component fails when decoding."]
    #[inline(always)]
    pub fn cid_err(&self) -> CID_ERR_R {
        CID_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The enable interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding."]
    #[inline(always)]
    pub fn c_dht_dc_id_err(&self) -> C_DHT_DC_ID_ERR_R {
        C_DHT_DC_ID_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The enable interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding."]
    #[inline(always)]
    pub fn c_dht_ac_id_err(&self) -> C_DHT_AC_ID_ERR_R {
        C_DHT_AC_ID_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The enable interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding."]
    #[inline(always)]
    pub fn c_dqt_id_err(&self) -> C_DQT_ID_ERR_R {
        C_DQT_ID_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The enable interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding."]
    #[inline(always)]
    pub fn rst_uxp_err(&self) -> RST_UXP_ERR_R {
        RST_UXP_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The enable interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding."]
    #[inline(always)]
    pub fn rst_check_none_err(&self) -> RST_CHECK_NONE_ERR_R {
        RST_CHECK_NONE_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The enable interrupt bit to sign that RST header marker position mismatches with restart interval when decoding."]
    #[inline(always)]
    pub fn rst_check_pos_err(&self) -> RST_CHECK_POS_ERR_R {
        RST_CHECK_POS_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The enable interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel."]
    #[inline(always)]
    pub fn out_eof(&self) -> OUT_EOF_R {
        OUT_EOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The enable interrupt bit to sign that the selected source color mode is not supported."]
    #[inline(always)]
    pub fn sr_color_mode_err(&self) -> SR_COLOR_MODE_ERR_R {
        SR_COLOR_MODE_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The enable interrupt bit to sign that one dct calculation is finished."]
    #[inline(always)]
    pub fn dct_done(&self) -> DCT_DONE_R {
        DCT_DONE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The enable interrupt bit to sign that the coding process for last block is finished."]
    #[inline(always)]
    pub fn bs_last_block_eof(&self) -> BS_LAST_BLOCK_EOF_R {
        BS_LAST_BLOCK_EOF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The enable interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded."]
    #[inline(always)]
    pub fn scan_check_none_err(&self) -> SCAN_CHECK_NONE_ERR_R {
        SCAN_CHECK_NONE_ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The enable interrupt bit to sign that SOS header marker position wrong when decoding."]
    #[inline(always)]
    pub fn scan_check_pos_err(&self) -> SCAN_CHECK_POS_ERR_R {
        SCAN_CHECK_POS_ERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The enable interrupt bit to sign that unsupported header marker is detected when decoding."]
    #[inline(always)]
    pub fn uxp_det(&self) -> UXP_DET_R {
        UXP_DET_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The enable interrupt bit to sign that received pixel blocks are smaller than expected when encoding."]
    #[inline(always)]
    pub fn en_frame_eof_err(&self) -> EN_FRAME_EOF_ERR_R {
        EN_FRAME_EOF_ERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The enable interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough."]
    #[inline(always)]
    pub fn en_frame_eof_lack(&self) -> EN_FRAME_EOF_LACK_R {
        EN_FRAME_EOF_LACK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The enable interrupt bit to sign that decoded blocks are smaller than expected when decoding."]
    #[inline(always)]
    pub fn de_frame_eof_err(&self) -> DE_FRAME_EOF_ERR_R {
        DE_FRAME_EOF_ERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The enable interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough."]
    #[inline(always)]
    pub fn de_frame_eof_lack(&self) -> DE_FRAME_EOF_LACK_R {
        DE_FRAME_EOF_LACK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The enable interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding."]
    #[inline(always)]
    pub fn sos_unmatch_err(&self) -> SOS_UNMATCH_ERR_R {
        SOS_UNMATCH_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The enable interrupt bit to sign that the first scan has header marker error when decoding."]
    #[inline(always)]
    pub fn marker_err_fst_scan(&self) -> MARKER_ERR_FST_SCAN_R {
        MARKER_ERR_FST_SCAN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The enable interrupt bit to sign that the following scans but not the first scan have header marker error when decoding."]
    #[inline(always)]
    pub fn marker_err_other_scan(&self) -> MARKER_ERR_OTHER_SCAN_R {
        MARKER_ERR_OTHER_SCAN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The enable interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding."]
    #[inline(always)]
    pub fn undet(&self) -> UNDET_R {
        UNDET_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The enable interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding."]
    #[inline(always)]
    pub fn decode_timeout(&self) -> DECODE_TIMEOUT_R {
        DECODE_TIMEOUT_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("done", &format_args!("{}", self.done().bit()))
            .field(
                "rle_parallel_err",
                &format_args!("{}", self.rle_parallel_err().bit()),
            )
            .field("cid_err", &format_args!("{}", self.cid_err().bit()))
            .field(
                "c_dht_dc_id_err",
                &format_args!("{}", self.c_dht_dc_id_err().bit()),
            )
            .field(
                "c_dht_ac_id_err",
                &format_args!("{}", self.c_dht_ac_id_err().bit()),
            )
            .field(
                "c_dqt_id_err",
                &format_args!("{}", self.c_dqt_id_err().bit()),
            )
            .field("rst_uxp_err", &format_args!("{}", self.rst_uxp_err().bit()))
            .field(
                "rst_check_none_err",
                &format_args!("{}", self.rst_check_none_err().bit()),
            )
            .field(
                "rst_check_pos_err",
                &format_args!("{}", self.rst_check_pos_err().bit()),
            )
            .field("out_eof", &format_args!("{}", self.out_eof().bit()))
            .field(
                "sr_color_mode_err",
                &format_args!("{}", self.sr_color_mode_err().bit()),
            )
            .field("dct_done", &format_args!("{}", self.dct_done().bit()))
            .field(
                "bs_last_block_eof",
                &format_args!("{}", self.bs_last_block_eof().bit()),
            )
            .field(
                "scan_check_none_err",
                &format_args!("{}", self.scan_check_none_err().bit()),
            )
            .field(
                "scan_check_pos_err",
                &format_args!("{}", self.scan_check_pos_err().bit()),
            )
            .field("uxp_det", &format_args!("{}", self.uxp_det().bit()))
            .field(
                "en_frame_eof_err",
                &format_args!("{}", self.en_frame_eof_err().bit()),
            )
            .field(
                "en_frame_eof_lack",
                &format_args!("{}", self.en_frame_eof_lack().bit()),
            )
            .field(
                "de_frame_eof_err",
                &format_args!("{}", self.de_frame_eof_err().bit()),
            )
            .field(
                "de_frame_eof_lack",
                &format_args!("{}", self.de_frame_eof_lack().bit()),
            )
            .field(
                "sos_unmatch_err",
                &format_args!("{}", self.sos_unmatch_err().bit()),
            )
            .field(
                "marker_err_fst_scan",
                &format_args!("{}", self.marker_err_fst_scan().bit()),
            )
            .field(
                "marker_err_other_scan",
                &format_args!("{}", self.marker_err_other_scan().bit()),
            )
            .field("undet", &format_args!("{}", self.undet().bit()))
            .field(
                "decode_timeout",
                &format_args!("{}", self.decode_timeout().bit()),
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
    #[doc = "Bit 0 - This enable interrupt bit turns to high level when JPEG finishes encoding a picture.."]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<INT_ENA_SPEC> {
        DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - The enable interrupt bit to sign that rle parallel error when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn rle_parallel_err(&mut self) -> RLE_PARALLEL_ERR_W<INT_ENA_SPEC> {
        RLE_PARALLEL_ERR_W::new(self, 1)
    }
    #[doc = "Bit 2 - The enable interrupt bit to sign that scan id check with component fails when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn cid_err(&mut self) -> CID_ERR_W<INT_ENA_SPEC> {
        CID_ERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - The enable interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn c_dht_dc_id_err(&mut self) -> C_DHT_DC_ID_ERR_W<INT_ENA_SPEC> {
        C_DHT_DC_ID_ERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - The enable interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn c_dht_ac_id_err(&mut self) -> C_DHT_AC_ID_ERR_W<INT_ENA_SPEC> {
        C_DHT_AC_ID_ERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - The enable interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn c_dqt_id_err(&mut self) -> C_DQT_ID_ERR_W<INT_ENA_SPEC> {
        C_DQT_ID_ERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - The enable interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn rst_uxp_err(&mut self) -> RST_UXP_ERR_W<INT_ENA_SPEC> {
        RST_UXP_ERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - The enable interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn rst_check_none_err(&mut self) -> RST_CHECK_NONE_ERR_W<INT_ENA_SPEC> {
        RST_CHECK_NONE_ERR_W::new(self, 7)
    }
    #[doc = "Bit 8 - The enable interrupt bit to sign that RST header marker position mismatches with restart interval when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn rst_check_pos_err(&mut self) -> RST_CHECK_POS_ERR_W<INT_ENA_SPEC> {
        RST_CHECK_POS_ERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - The enable interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel."]
    #[inline(always)]
    #[must_use]
    pub fn out_eof(&mut self) -> OUT_EOF_W<INT_ENA_SPEC> {
        OUT_EOF_W::new(self, 9)
    }
    #[doc = "Bit 10 - The enable interrupt bit to sign that the selected source color mode is not supported."]
    #[inline(always)]
    #[must_use]
    pub fn sr_color_mode_err(&mut self) -> SR_COLOR_MODE_ERR_W<INT_ENA_SPEC> {
        SR_COLOR_MODE_ERR_W::new(self, 10)
    }
    #[doc = "Bit 11 - The enable interrupt bit to sign that one dct calculation is finished."]
    #[inline(always)]
    #[must_use]
    pub fn dct_done(&mut self) -> DCT_DONE_W<INT_ENA_SPEC> {
        DCT_DONE_W::new(self, 11)
    }
    #[doc = "Bit 12 - The enable interrupt bit to sign that the coding process for last block is finished."]
    #[inline(always)]
    #[must_use]
    pub fn bs_last_block_eof(&mut self) -> BS_LAST_BLOCK_EOF_W<INT_ENA_SPEC> {
        BS_LAST_BLOCK_EOF_W::new(self, 12)
    }
    #[doc = "Bit 13 - The enable interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded."]
    #[inline(always)]
    #[must_use]
    pub fn scan_check_none_err(&mut self) -> SCAN_CHECK_NONE_ERR_W<INT_ENA_SPEC> {
        SCAN_CHECK_NONE_ERR_W::new(self, 13)
    }
    #[doc = "Bit 14 - The enable interrupt bit to sign that SOS header marker position wrong when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn scan_check_pos_err(&mut self) -> SCAN_CHECK_POS_ERR_W<INT_ENA_SPEC> {
        SCAN_CHECK_POS_ERR_W::new(self, 14)
    }
    #[doc = "Bit 15 - The enable interrupt bit to sign that unsupported header marker is detected when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn uxp_det(&mut self) -> UXP_DET_W<INT_ENA_SPEC> {
        UXP_DET_W::new(self, 15)
    }
    #[doc = "Bit 16 - The enable interrupt bit to sign that received pixel blocks are smaller than expected when encoding."]
    #[inline(always)]
    #[must_use]
    pub fn en_frame_eof_err(&mut self) -> EN_FRAME_EOF_ERR_W<INT_ENA_SPEC> {
        EN_FRAME_EOF_ERR_W::new(self, 16)
    }
    #[doc = "Bit 17 - The enable interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough."]
    #[inline(always)]
    #[must_use]
    pub fn en_frame_eof_lack(&mut self) -> EN_FRAME_EOF_LACK_W<INT_ENA_SPEC> {
        EN_FRAME_EOF_LACK_W::new(self, 17)
    }
    #[doc = "Bit 18 - The enable interrupt bit to sign that decoded blocks are smaller than expected when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn de_frame_eof_err(&mut self) -> DE_FRAME_EOF_ERR_W<INT_ENA_SPEC> {
        DE_FRAME_EOF_ERR_W::new(self, 18)
    }
    #[doc = "Bit 19 - The enable interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough."]
    #[inline(always)]
    #[must_use]
    pub fn de_frame_eof_lack(&mut self) -> DE_FRAME_EOF_LACK_W<INT_ENA_SPEC> {
        DE_FRAME_EOF_LACK_W::new(self, 19)
    }
    #[doc = "Bit 20 - The enable interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn sos_unmatch_err(&mut self) -> SOS_UNMATCH_ERR_W<INT_ENA_SPEC> {
        SOS_UNMATCH_ERR_W::new(self, 20)
    }
    #[doc = "Bit 21 - The enable interrupt bit to sign that the first scan has header marker error when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn marker_err_fst_scan(&mut self) -> MARKER_ERR_FST_SCAN_W<INT_ENA_SPEC> {
        MARKER_ERR_FST_SCAN_W::new(self, 21)
    }
    #[doc = "Bit 22 - The enable interrupt bit to sign that the following scans but not the first scan have header marker error when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn marker_err_other_scan(&mut self) -> MARKER_ERR_OTHER_SCAN_W<INT_ENA_SPEC> {
        MARKER_ERR_OTHER_SCAN_W::new(self, 22)
    }
    #[doc = "Bit 23 - The enable interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn undet(&mut self) -> UNDET_W<INT_ENA_SPEC> {
        UNDET_W::new(self, 23)
    }
    #[doc = "Bit 24 - The enable interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn decode_timeout(&mut self) -> DECODE_TIMEOUT_W<INT_ENA_SPEC> {
        DECODE_TIMEOUT_W::new(self, 24)
    }
}
#[doc = "Interrupt enable registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
