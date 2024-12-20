#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster Q%s, containing Q?_WORD0, Q?_WORD1"]
pub struct Q {
    word0: WORD0,
    word1: WORD1,
}
impl Q {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn word0(&self) -> &WORD0 {
        &self.word0
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn word1(&self) -> &WORD1 {
        &self.word1
    }
}
#[doc = "WORD0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word0`] module"]
pub type WORD0 = crate::Reg<word0::WORD0_SPEC>;
#[doc = ""]
pub mod word0;
#[doc = "WORD1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word1`] module"]
pub type WORD1 = crate::Reg<word1::WORD1_SPEC>;
#[doc = ""]
pub mod word1;
