#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sf_ctrl_0: SfCtrl0,
    sf_ctrl_1: SfCtrl1,
    sf_if_sahb_0: SfIfSahb0,
    sf_if_sahb_1: SfIfSahb1,
    sf_if_sahb_2: SfIfSahb2,
    sf_if_iahb_0: SfIfIahb0,
    sf_if_iahb_1: SfIfIahb1,
    sf_if_iahb_2: SfIfIahb2,
    sf_if_status_0: SfIfStatus0,
    sf_if_status_1: SfIfStatus1,
    sf_aes: SfAes,
    sf_ahb2sif_status: SfAhb2sifStatus,
    sf_if_io_dly_0: SfIfIoDly0,
    sf_if_io_dly_1: SfIfIoDly1,
    sf_if_io_dly_2: SfIfIoDly2,
    sf_if_io_dly_3: SfIfIoDly3,
    sf_if_io_dly_4: SfIfIoDly4,
    sf_reserved: SfReserved,
    sf2_if_io_dly_0: Sf2IfIoDly0,
    sf2_if_io_dly_1: Sf2IfIoDly1,
    sf2_if_io_dly_2: Sf2IfIoDly2,
    sf2_if_io_dly_3: Sf2IfIoDly3,
    sf2_if_io_dly_4: Sf2IfIoDly4,
    sf3_if_io_dly_0: Sf3IfIoDly0,
    sf3_if_io_dly_1: Sf3IfIoDly1,
    sf3_if_io_dly_2: Sf3IfIoDly2,
    sf3_if_io_dly_3: Sf3IfIoDly3,
    sf3_if_io_dly_4: Sf3IfIoDly4,
    sf_ctrl_2: SfCtrl2,
    sf_ctrl_3: SfCtrl3,
    sf_if_iahb_3: SfIfIahb3,
    sf_if_iahb_4: SfIfIahb4,
    sf_if_iahb_5: SfIfIahb5,
    sf_if_iahb_6: SfIfIahb6,
    sf_if_iahb_7: SfIfIahb7,
    _reserved35: [u8; 0x74],
    sf_ctrl_prot_en_rd: SfCtrlProtEnRd,
    sf_ctrl_prot_en: SfCtrlProtEn,
    _reserved37: [u8; 0xf8],
    sf_aes_key_r0_0: SfAesKeyR0_0,
    sf_aes_key_r0_1: SfAesKeyR0_1,
    sf_aes_key_r0_2: SfAesKeyR0_2,
    sf_aes_key_r0_3: SfAesKeyR0_3,
    sf_aes_key_r0_4: SfAesKeyR0_4,
    sf_aes_key_r0_5: SfAesKeyR0_5,
    sf_aes_key_r0_6: SfAesKeyR0_6,
    sf_aes_key_r0_7: SfAesKeyR0_7,
    sf_aes_iv_r0_w0: SfAesIvR0W0,
    sf_aes_iv_r0_w1: SfAesIvR0W1,
    sf_aes_iv_r0_w2: SfAesIvR0W2,
    sf_aes_iv_r0_w3: SfAesIvR0W3,
    sf_aes_cfg_r0: SfAesCfgR0,
    _reserved50: [u8; 0xcc],
    sf_aes_key_r1_0: SfAesKeyR1_0,
    sf_aes_key_r1_1: SfAesKeyR1_1,
    sf_aes_key_r1_2: SfAesKeyR1_2,
    sf_aes_key_r1_3: SfAesKeyR1_3,
    sf_aes_key_r1_4: SfAesKeyR1_4,
    sf_aes_key_r1_5: SfAesKeyR1_5,
    sf_aes_key_r1_6: SfAesKeyR1_6,
    sf_aes_key_r1_7: SfAesKeyR1_7,
    sf_aes_iv_r1_w0: SfAesIvR1W0,
    sf_aes_iv_r1_w1: SfAesIvR1W1,
    sf_aes_iv_r1_w2: SfAesIvR1W2,
    sf_aes_iv_r1_w3: SfAesIvR1W3,
    sf_aes_r1: SfAesR1,
    _reserved63: [u8; 0xcc],
    sf_aes_key_r2_0: SfAesKeyR2_0,
    sf_aes_key_r2_1: SfAesKeyR2_1,
    sf_aes_key_r2_2: SfAesKeyR2_2,
    sf_aes_key_r2_3: SfAesKeyR2_3,
    sf_aes_key_r2_4: SfAesKeyR2_4,
    sf_aes_key_r2_5: SfAesKeyR2_5,
    sf_aes_key_r2_6: SfAesKeyR2_6,
    sf_aes_key_r2_7: SfAesKeyR2_7,
    sf_aes_iv_r2_w0: SfAesIvR2W0,
    sf_aes_iv_r2_w1: SfAesIvR2W1,
    sf_aes_iv_r2_w2: SfAesIvR2W2,
    sf_aes_iv_r2_w3: SfAesIvR2W3,
    sf_aes_r2: SfAesR2,
    sf_id0_offset: SfId0Offset,
    sf_id1_offset: SfId1Offset,
}
impl RegisterBlock {
    #[doc = "0x00 - sf_ctrl_0."]
    #[inline(always)]
    pub const fn sf_ctrl_0(&self) -> &SfCtrl0 {
        &self.sf_ctrl_0
    }
    #[doc = "0x04 - sf_ctrl_1."]
    #[inline(always)]
    pub const fn sf_ctrl_1(&self) -> &SfCtrl1 {
        &self.sf_ctrl_1
    }
    #[doc = "0x08 - sf_if_sahb_0."]
    #[inline(always)]
    pub const fn sf_if_sahb_0(&self) -> &SfIfSahb0 {
        &self.sf_if_sahb_0
    }
    #[doc = "0x0c - sf_if_sahb_1."]
    #[inline(always)]
    pub const fn sf_if_sahb_1(&self) -> &SfIfSahb1 {
        &self.sf_if_sahb_1
    }
    #[doc = "0x10 - sf_if_sahb_2."]
    #[inline(always)]
    pub const fn sf_if_sahb_2(&self) -> &SfIfSahb2 {
        &self.sf_if_sahb_2
    }
    #[doc = "0x14 - sf_if_iahb_0."]
    #[inline(always)]
    pub const fn sf_if_iahb_0(&self) -> &SfIfIahb0 {
        &self.sf_if_iahb_0
    }
    #[doc = "0x18 - sf_if_iahb_1."]
    #[inline(always)]
    pub const fn sf_if_iahb_1(&self) -> &SfIfIahb1 {
        &self.sf_if_iahb_1
    }
    #[doc = "0x1c - sf_if_iahb_2."]
    #[inline(always)]
    pub const fn sf_if_iahb_2(&self) -> &SfIfIahb2 {
        &self.sf_if_iahb_2
    }
    #[doc = "0x20 - sf_if_status_0."]
    #[inline(always)]
    pub const fn sf_if_status_0(&self) -> &SfIfStatus0 {
        &self.sf_if_status_0
    }
    #[doc = "0x24 - sf_if_status_1."]
    #[inline(always)]
    pub const fn sf_if_status_1(&self) -> &SfIfStatus1 {
        &self.sf_if_status_1
    }
    #[doc = "0x28 - sf_aes."]
    #[inline(always)]
    pub const fn sf_aes(&self) -> &SfAes {
        &self.sf_aes
    }
    #[doc = "0x2c - sf_ahb2sif_status."]
    #[inline(always)]
    pub const fn sf_ahb2sif_status(&self) -> &SfAhb2sifStatus {
        &self.sf_ahb2sif_status
    }
    #[doc = "0x30 - sf_if_io_dly_0."]
    #[inline(always)]
    pub const fn sf_if_io_dly_0(&self) -> &SfIfIoDly0 {
        &self.sf_if_io_dly_0
    }
    #[doc = "0x34 - sf_if_io_dly_1."]
    #[inline(always)]
    pub const fn sf_if_io_dly_1(&self) -> &SfIfIoDly1 {
        &self.sf_if_io_dly_1
    }
    #[doc = "0x38 - sf_if_io_dly_2."]
    #[inline(always)]
    pub const fn sf_if_io_dly_2(&self) -> &SfIfIoDly2 {
        &self.sf_if_io_dly_2
    }
    #[doc = "0x3c - sf_if_io_dly_3."]
    #[inline(always)]
    pub const fn sf_if_io_dly_3(&self) -> &SfIfIoDly3 {
        &self.sf_if_io_dly_3
    }
    #[doc = "0x40 - sf_if_io_dly_4."]
    #[inline(always)]
    pub const fn sf_if_io_dly_4(&self) -> &SfIfIoDly4 {
        &self.sf_if_io_dly_4
    }
    #[doc = "0x44 - sf_reserved."]
    #[inline(always)]
    pub const fn sf_reserved(&self) -> &SfReserved {
        &self.sf_reserved
    }
    #[doc = "0x48 - sf2_if_io_dly_0."]
    #[inline(always)]
    pub const fn sf2_if_io_dly_0(&self) -> &Sf2IfIoDly0 {
        &self.sf2_if_io_dly_0
    }
    #[doc = "0x4c - sf2_if_io_dly_1."]
    #[inline(always)]
    pub const fn sf2_if_io_dly_1(&self) -> &Sf2IfIoDly1 {
        &self.sf2_if_io_dly_1
    }
    #[doc = "0x50 - sf2_if_io_dly_2."]
    #[inline(always)]
    pub const fn sf2_if_io_dly_2(&self) -> &Sf2IfIoDly2 {
        &self.sf2_if_io_dly_2
    }
    #[doc = "0x54 - sf2_if_io_dly_3."]
    #[inline(always)]
    pub const fn sf2_if_io_dly_3(&self) -> &Sf2IfIoDly3 {
        &self.sf2_if_io_dly_3
    }
    #[doc = "0x58 - sf2_if_io_dly_4."]
    #[inline(always)]
    pub const fn sf2_if_io_dly_4(&self) -> &Sf2IfIoDly4 {
        &self.sf2_if_io_dly_4
    }
    #[doc = "0x5c - sf3_if_io_dly_0."]
    #[inline(always)]
    pub const fn sf3_if_io_dly_0(&self) -> &Sf3IfIoDly0 {
        &self.sf3_if_io_dly_0
    }
    #[doc = "0x60 - sf3_if_io_dly_1."]
    #[inline(always)]
    pub const fn sf3_if_io_dly_1(&self) -> &Sf3IfIoDly1 {
        &self.sf3_if_io_dly_1
    }
    #[doc = "0x64 - sf3_if_io_dly_2."]
    #[inline(always)]
    pub const fn sf3_if_io_dly_2(&self) -> &Sf3IfIoDly2 {
        &self.sf3_if_io_dly_2
    }
    #[doc = "0x68 - sf3_if_io_dly_3."]
    #[inline(always)]
    pub const fn sf3_if_io_dly_3(&self) -> &Sf3IfIoDly3 {
        &self.sf3_if_io_dly_3
    }
    #[doc = "0x6c - sf3_if_io_dly_4."]
    #[inline(always)]
    pub const fn sf3_if_io_dly_4(&self) -> &Sf3IfIoDly4 {
        &self.sf3_if_io_dly_4
    }
    #[doc = "0x70 - sf_ctrl_2."]
    #[inline(always)]
    pub const fn sf_ctrl_2(&self) -> &SfCtrl2 {
        &self.sf_ctrl_2
    }
    #[doc = "0x74 - sf_ctrl_3."]
    #[inline(always)]
    pub const fn sf_ctrl_3(&self) -> &SfCtrl3 {
        &self.sf_ctrl_3
    }
    #[doc = "0x78 - sf_if_iahb_3."]
    #[inline(always)]
    pub const fn sf_if_iahb_3(&self) -> &SfIfIahb3 {
        &self.sf_if_iahb_3
    }
    #[doc = "0x7c - sf_if_iahb_4."]
    #[inline(always)]
    pub const fn sf_if_iahb_4(&self) -> &SfIfIahb4 {
        &self.sf_if_iahb_4
    }
    #[doc = "0x80 - sf_if_iahb_5."]
    #[inline(always)]
    pub const fn sf_if_iahb_5(&self) -> &SfIfIahb5 {
        &self.sf_if_iahb_5
    }
    #[doc = "0x84 - sf_if_iahb_6."]
    #[inline(always)]
    pub const fn sf_if_iahb_6(&self) -> &SfIfIahb6 {
        &self.sf_if_iahb_6
    }
    #[doc = "0x88 - sf_if_iahb_7."]
    #[inline(always)]
    pub const fn sf_if_iahb_7(&self) -> &SfIfIahb7 {
        &self.sf_if_iahb_7
    }
    #[doc = "0x100 - sf_ctrl_prot_en_rd."]
    #[inline(always)]
    pub const fn sf_ctrl_prot_en_rd(&self) -> &SfCtrlProtEnRd {
        &self.sf_ctrl_prot_en_rd
    }
    #[doc = "0x104 - sf_ctrl_prot_en."]
    #[inline(always)]
    pub const fn sf_ctrl_prot_en(&self) -> &SfCtrlProtEn {
        &self.sf_ctrl_prot_en
    }
    #[doc = "0x200 - sf_aes_key_r0_0."]
    #[inline(always)]
    pub const fn sf_aes_key_r0_0(&self) -> &SfAesKeyR0_0 {
        &self.sf_aes_key_r0_0
    }
    #[doc = "0x204 - sf_aes_key_r0_1."]
    #[inline(always)]
    pub const fn sf_aes_key_r0_1(&self) -> &SfAesKeyR0_1 {
        &self.sf_aes_key_r0_1
    }
    #[doc = "0x208 - sf_aes_key_r0_2."]
    #[inline(always)]
    pub const fn sf_aes_key_r0_2(&self) -> &SfAesKeyR0_2 {
        &self.sf_aes_key_r0_2
    }
    #[doc = "0x20c - sf_aes_key_r0_3."]
    #[inline(always)]
    pub const fn sf_aes_key_r0_3(&self) -> &SfAesKeyR0_3 {
        &self.sf_aes_key_r0_3
    }
    #[doc = "0x210 - sf_aes_key_r0_4."]
    #[inline(always)]
    pub const fn sf_aes_key_r0_4(&self) -> &SfAesKeyR0_4 {
        &self.sf_aes_key_r0_4
    }
    #[doc = "0x214 - sf_aes_key_r0_5."]
    #[inline(always)]
    pub const fn sf_aes_key_r0_5(&self) -> &SfAesKeyR0_5 {
        &self.sf_aes_key_r0_5
    }
    #[doc = "0x218 - sf_aes_key_r0_6."]
    #[inline(always)]
    pub const fn sf_aes_key_r0_6(&self) -> &SfAesKeyR0_6 {
        &self.sf_aes_key_r0_6
    }
    #[doc = "0x21c - sf_aes_key_r0_7."]
    #[inline(always)]
    pub const fn sf_aes_key_r0_7(&self) -> &SfAesKeyR0_7 {
        &self.sf_aes_key_r0_7
    }
    #[doc = "0x220 - sf_aes_iv_r0_w0."]
    #[inline(always)]
    pub const fn sf_aes_iv_r0_w0(&self) -> &SfAesIvR0W0 {
        &self.sf_aes_iv_r0_w0
    }
    #[doc = "0x224 - sf_aes_iv_r0_w1."]
    #[inline(always)]
    pub const fn sf_aes_iv_r0_w1(&self) -> &SfAesIvR0W1 {
        &self.sf_aes_iv_r0_w1
    }
    #[doc = "0x228 - sf_aes_iv_r0_w2."]
    #[inline(always)]
    pub const fn sf_aes_iv_r0_w2(&self) -> &SfAesIvR0W2 {
        &self.sf_aes_iv_r0_w2
    }
    #[doc = "0x22c - sf_aes_iv_r0_w3."]
    #[inline(always)]
    pub const fn sf_aes_iv_r0_w3(&self) -> &SfAesIvR0W3 {
        &self.sf_aes_iv_r0_w3
    }
    #[doc = "0x230 - sf_aes_cfg_r0."]
    #[inline(always)]
    pub const fn sf_aes_cfg_r0(&self) -> &SfAesCfgR0 {
        &self.sf_aes_cfg_r0
    }
    #[doc = "0x300 - sf_aes_key_r1_0."]
    #[inline(always)]
    pub const fn sf_aes_key_r1_0(&self) -> &SfAesKeyR1_0 {
        &self.sf_aes_key_r1_0
    }
    #[doc = "0x304 - sf_aes_key_r1_1."]
    #[inline(always)]
    pub const fn sf_aes_key_r1_1(&self) -> &SfAesKeyR1_1 {
        &self.sf_aes_key_r1_1
    }
    #[doc = "0x308 - sf_aes_key_r1_2."]
    #[inline(always)]
    pub const fn sf_aes_key_r1_2(&self) -> &SfAesKeyR1_2 {
        &self.sf_aes_key_r1_2
    }
    #[doc = "0x30c - sf_aes_key_r1_3."]
    #[inline(always)]
    pub const fn sf_aes_key_r1_3(&self) -> &SfAesKeyR1_3 {
        &self.sf_aes_key_r1_3
    }
    #[doc = "0x310 - sf_aes_key_r1_4."]
    #[inline(always)]
    pub const fn sf_aes_key_r1_4(&self) -> &SfAesKeyR1_4 {
        &self.sf_aes_key_r1_4
    }
    #[doc = "0x314 - sf_aes_key_r1_5."]
    #[inline(always)]
    pub const fn sf_aes_key_r1_5(&self) -> &SfAesKeyR1_5 {
        &self.sf_aes_key_r1_5
    }
    #[doc = "0x318 - sf_aes_key_r1_6."]
    #[inline(always)]
    pub const fn sf_aes_key_r1_6(&self) -> &SfAesKeyR1_6 {
        &self.sf_aes_key_r1_6
    }
    #[doc = "0x31c - sf_aes_key_r1_7."]
    #[inline(always)]
    pub const fn sf_aes_key_r1_7(&self) -> &SfAesKeyR1_7 {
        &self.sf_aes_key_r1_7
    }
    #[doc = "0x320 - sf_aes_iv_r1_w0."]
    #[inline(always)]
    pub const fn sf_aes_iv_r1_w0(&self) -> &SfAesIvR1W0 {
        &self.sf_aes_iv_r1_w0
    }
    #[doc = "0x324 - sf_aes_iv_r1_w1."]
    #[inline(always)]
    pub const fn sf_aes_iv_r1_w1(&self) -> &SfAesIvR1W1 {
        &self.sf_aes_iv_r1_w1
    }
    #[doc = "0x328 - sf_aes_iv_r1_w2."]
    #[inline(always)]
    pub const fn sf_aes_iv_r1_w2(&self) -> &SfAesIvR1W2 {
        &self.sf_aes_iv_r1_w2
    }
    #[doc = "0x32c - sf_aes_iv_r1_w3."]
    #[inline(always)]
    pub const fn sf_aes_iv_r1_w3(&self) -> &SfAesIvR1W3 {
        &self.sf_aes_iv_r1_w3
    }
    #[doc = "0x330 - sf_aes_r1."]
    #[inline(always)]
    pub const fn sf_aes_r1(&self) -> &SfAesR1 {
        &self.sf_aes_r1
    }
    #[doc = "0x400 - sf_aes_key_r2_0."]
    #[inline(always)]
    pub const fn sf_aes_key_r2_0(&self) -> &SfAesKeyR2_0 {
        &self.sf_aes_key_r2_0
    }
    #[doc = "0x404 - sf_aes_key_r2_1."]
    #[inline(always)]
    pub const fn sf_aes_key_r2_1(&self) -> &SfAesKeyR2_1 {
        &self.sf_aes_key_r2_1
    }
    #[doc = "0x408 - sf_aes_key_r2_2."]
    #[inline(always)]
    pub const fn sf_aes_key_r2_2(&self) -> &SfAesKeyR2_2 {
        &self.sf_aes_key_r2_2
    }
    #[doc = "0x40c - sf_aes_key_r2_3."]
    #[inline(always)]
    pub const fn sf_aes_key_r2_3(&self) -> &SfAesKeyR2_3 {
        &self.sf_aes_key_r2_3
    }
    #[doc = "0x410 - sf_aes_key_r2_4."]
    #[inline(always)]
    pub const fn sf_aes_key_r2_4(&self) -> &SfAesKeyR2_4 {
        &self.sf_aes_key_r2_4
    }
    #[doc = "0x414 - sf_aes_key_r2_5."]
    #[inline(always)]
    pub const fn sf_aes_key_r2_5(&self) -> &SfAesKeyR2_5 {
        &self.sf_aes_key_r2_5
    }
    #[doc = "0x418 - sf_aes_key_r2_6."]
    #[inline(always)]
    pub const fn sf_aes_key_r2_6(&self) -> &SfAesKeyR2_6 {
        &self.sf_aes_key_r2_6
    }
    #[doc = "0x41c - sf_aes_key_r2_7."]
    #[inline(always)]
    pub const fn sf_aes_key_r2_7(&self) -> &SfAesKeyR2_7 {
        &self.sf_aes_key_r2_7
    }
    #[doc = "0x420 - sf_aes_iv_r2_w0."]
    #[inline(always)]
    pub const fn sf_aes_iv_r2_w0(&self) -> &SfAesIvR2W0 {
        &self.sf_aes_iv_r2_w0
    }
    #[doc = "0x424 - sf_aes_iv_r2_w1."]
    #[inline(always)]
    pub const fn sf_aes_iv_r2_w1(&self) -> &SfAesIvR2W1 {
        &self.sf_aes_iv_r2_w1
    }
    #[doc = "0x428 - sf_aes_iv_r2_w2."]
    #[inline(always)]
    pub const fn sf_aes_iv_r2_w2(&self) -> &SfAesIvR2W2 {
        &self.sf_aes_iv_r2_w2
    }
    #[doc = "0x42c - sf_aes_iv_r2_w3."]
    #[inline(always)]
    pub const fn sf_aes_iv_r2_w3(&self) -> &SfAesIvR2W3 {
        &self.sf_aes_iv_r2_w3
    }
    #[doc = "0x430 - sf_aes_r2."]
    #[inline(always)]
    pub const fn sf_aes_r2(&self) -> &SfAesR2 {
        &self.sf_aes_r2
    }
    #[doc = "0x434 - sf_id0_offset."]
    #[inline(always)]
    pub const fn sf_id0_offset(&self) -> &SfId0Offset {
        &self.sf_id0_offset
    }
    #[doc = "0x438 - sf_id1_offset."]
    #[inline(always)]
    pub const fn sf_id1_offset(&self) -> &SfId1Offset {
        &self.sf_id1_offset
    }
}
#[doc = "sf_ctrl_0 (rw) register accessor: sf_ctrl_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ctrl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_ctrl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_ctrl_0`] module"]
#[doc(alias = "sf_ctrl_0")]
pub type SfCtrl0 = crate::Reg<sf_ctrl_0::SfCtrl0Spec>;
#[doc = "sf_ctrl_0."]
pub mod sf_ctrl_0;
#[doc = "sf_ctrl_1 (rw) register accessor: sf_ctrl_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ctrl_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_ctrl_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_ctrl_1`] module"]
#[doc(alias = "sf_ctrl_1")]
pub type SfCtrl1 = crate::Reg<sf_ctrl_1::SfCtrl1Spec>;
#[doc = "sf_ctrl_1."]
pub mod sf_ctrl_1;
#[doc = "sf_if_sahb_0 (rw) register accessor: sf_if_sahb_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_sahb_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_sahb_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_sahb_0`] module"]
#[doc(alias = "sf_if_sahb_0")]
pub type SfIfSahb0 = crate::Reg<sf_if_sahb_0::SfIfSahb0Spec>;
#[doc = "sf_if_sahb_0."]
pub mod sf_if_sahb_0;
#[doc = "sf_if_sahb_1 (rw) register accessor: sf_if_sahb_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_sahb_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_sahb_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_sahb_1`] module"]
#[doc(alias = "sf_if_sahb_1")]
pub type SfIfSahb1 = crate::Reg<sf_if_sahb_1::SfIfSahb1Spec>;
#[doc = "sf_if_sahb_1."]
pub mod sf_if_sahb_1;
#[doc = "sf_if_sahb_2 (rw) register accessor: sf_if_sahb_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_sahb_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_sahb_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_sahb_2`] module"]
#[doc(alias = "sf_if_sahb_2")]
pub type SfIfSahb2 = crate::Reg<sf_if_sahb_2::SfIfSahb2Spec>;
#[doc = "sf_if_sahb_2."]
pub mod sf_if_sahb_2;
#[doc = "sf_if_iahb_0 (rw) register accessor: sf_if_iahb_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_iahb_0`] module"]
#[doc(alias = "sf_if_iahb_0")]
pub type SfIfIahb0 = crate::Reg<sf_if_iahb_0::SfIfIahb0Spec>;
#[doc = "sf_if_iahb_0."]
pub mod sf_if_iahb_0;
#[doc = "sf_if_iahb_1 (rw) register accessor: sf_if_iahb_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_iahb_1`] module"]
#[doc(alias = "sf_if_iahb_1")]
pub type SfIfIahb1 = crate::Reg<sf_if_iahb_1::SfIfIahb1Spec>;
#[doc = "sf_if_iahb_1."]
pub mod sf_if_iahb_1;
#[doc = "sf_if_iahb_2 (rw) register accessor: sf_if_iahb_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_iahb_2`] module"]
#[doc(alias = "sf_if_iahb_2")]
pub type SfIfIahb2 = crate::Reg<sf_if_iahb_2::SfIfIahb2Spec>;
#[doc = "sf_if_iahb_2."]
pub mod sf_if_iahb_2;
#[doc = "sf_if_status_0 (r) register accessor: sf_if_status_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_status_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_status_0`] module"]
#[doc(alias = "sf_if_status_0")]
pub type SfIfStatus0 = crate::Reg<sf_if_status_0::SfIfStatus0Spec>;
#[doc = "sf_if_status_0."]
pub mod sf_if_status_0;
#[doc = "sf_if_status_1 (r) register accessor: sf_if_status_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_status_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_status_1`] module"]
#[doc(alias = "sf_if_status_1")]
pub type SfIfStatus1 = crate::Reg<sf_if_status_1::SfIfStatus1Spec>;
#[doc = "sf_if_status_1."]
pub mod sf_if_status_1;
#[doc = "sf_aes (rw) register accessor: sf_aes.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes`] module"]
#[doc(alias = "sf_aes")]
pub type SfAes = crate::Reg<sf_aes::SfAesSpec>;
#[doc = "sf_aes."]
pub mod sf_aes;
#[doc = "sf_ahb2sif_status (r) register accessor: sf_ahb2sif_status.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ahb2sif_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_ahb2sif_status`] module"]
#[doc(alias = "sf_ahb2sif_status")]
pub type SfAhb2sifStatus = crate::Reg<sf_ahb2sif_status::SfAhb2sifStatusSpec>;
#[doc = "sf_ahb2sif_status."]
pub mod sf_ahb2sif_status;
#[doc = "sf_if_io_dly_0 (rw) register accessor: sf_if_io_dly_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_io_dly_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_io_dly_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_io_dly_0`] module"]
#[doc(alias = "sf_if_io_dly_0")]
pub type SfIfIoDly0 = crate::Reg<sf_if_io_dly_0::SfIfIoDly0Spec>;
#[doc = "sf_if_io_dly_0."]
pub mod sf_if_io_dly_0;
#[doc = "sf_if_io_dly_1 (rw) register accessor: sf_if_io_dly_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_io_dly_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_io_dly_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_io_dly_1`] module"]
#[doc(alias = "sf_if_io_dly_1")]
pub type SfIfIoDly1 = crate::Reg<sf_if_io_dly_1::SfIfIoDly1Spec>;
#[doc = "sf_if_io_dly_1."]
pub mod sf_if_io_dly_1;
#[doc = "sf_if_io_dly_2 (rw) register accessor: sf_if_io_dly_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_io_dly_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_io_dly_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_io_dly_2`] module"]
#[doc(alias = "sf_if_io_dly_2")]
pub type SfIfIoDly2 = crate::Reg<sf_if_io_dly_2::SfIfIoDly2Spec>;
#[doc = "sf_if_io_dly_2."]
pub mod sf_if_io_dly_2;
#[doc = "sf_if_io_dly_3 (rw) register accessor: sf_if_io_dly_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_io_dly_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_io_dly_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_io_dly_3`] module"]
#[doc(alias = "sf_if_io_dly_3")]
pub type SfIfIoDly3 = crate::Reg<sf_if_io_dly_3::SfIfIoDly3Spec>;
#[doc = "sf_if_io_dly_3."]
pub mod sf_if_io_dly_3;
#[doc = "sf_if_io_dly_4 (rw) register accessor: sf_if_io_dly_4.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_io_dly_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_io_dly_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_io_dly_4`] module"]
#[doc(alias = "sf_if_io_dly_4")]
pub type SfIfIoDly4 = crate::Reg<sf_if_io_dly_4::SfIfIoDly4Spec>;
#[doc = "sf_if_io_dly_4."]
pub mod sf_if_io_dly_4;
#[doc = "sf_reserved (rw) register accessor: sf_reserved.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_reserved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_reserved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_reserved`] module"]
#[doc(alias = "sf_reserved")]
pub type SfReserved = crate::Reg<sf_reserved::SfReservedSpec>;
#[doc = "sf_reserved."]
pub mod sf_reserved;
#[doc = "sf2_if_io_dly_0 (rw) register accessor: sf2_if_io_dly_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf2_if_io_dly_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf2_if_io_dly_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf2_if_io_dly_0`] module"]
#[doc(alias = "sf2_if_io_dly_0")]
pub type Sf2IfIoDly0 = crate::Reg<sf2_if_io_dly_0::Sf2IfIoDly0Spec>;
#[doc = "sf2_if_io_dly_0."]
pub mod sf2_if_io_dly_0;
#[doc = "sf2_if_io_dly_1 (rw) register accessor: sf2_if_io_dly_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf2_if_io_dly_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf2_if_io_dly_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf2_if_io_dly_1`] module"]
#[doc(alias = "sf2_if_io_dly_1")]
pub type Sf2IfIoDly1 = crate::Reg<sf2_if_io_dly_1::Sf2IfIoDly1Spec>;
#[doc = "sf2_if_io_dly_1."]
pub mod sf2_if_io_dly_1;
#[doc = "sf2_if_io_dly_2 (rw) register accessor: sf2_if_io_dly_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf2_if_io_dly_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf2_if_io_dly_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf2_if_io_dly_2`] module"]
#[doc(alias = "sf2_if_io_dly_2")]
pub type Sf2IfIoDly2 = crate::Reg<sf2_if_io_dly_2::Sf2IfIoDly2Spec>;
#[doc = "sf2_if_io_dly_2."]
pub mod sf2_if_io_dly_2;
#[doc = "sf2_if_io_dly_3 (rw) register accessor: sf2_if_io_dly_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf2_if_io_dly_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf2_if_io_dly_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf2_if_io_dly_3`] module"]
#[doc(alias = "sf2_if_io_dly_3")]
pub type Sf2IfIoDly3 = crate::Reg<sf2_if_io_dly_3::Sf2IfIoDly3Spec>;
#[doc = "sf2_if_io_dly_3."]
pub mod sf2_if_io_dly_3;
#[doc = "sf2_if_io_dly_4 (rw) register accessor: sf2_if_io_dly_4.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf2_if_io_dly_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf2_if_io_dly_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf2_if_io_dly_4`] module"]
#[doc(alias = "sf2_if_io_dly_4")]
pub type Sf2IfIoDly4 = crate::Reg<sf2_if_io_dly_4::Sf2IfIoDly4Spec>;
#[doc = "sf2_if_io_dly_4."]
pub mod sf2_if_io_dly_4;
#[doc = "sf3_if_io_dly_0 (rw) register accessor: sf3_if_io_dly_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf3_if_io_dly_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf3_if_io_dly_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf3_if_io_dly_0`] module"]
#[doc(alias = "sf3_if_io_dly_0")]
pub type Sf3IfIoDly0 = crate::Reg<sf3_if_io_dly_0::Sf3IfIoDly0Spec>;
#[doc = "sf3_if_io_dly_0."]
pub mod sf3_if_io_dly_0;
#[doc = "sf3_if_io_dly_1 (rw) register accessor: sf3_if_io_dly_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf3_if_io_dly_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf3_if_io_dly_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf3_if_io_dly_1`] module"]
#[doc(alias = "sf3_if_io_dly_1")]
pub type Sf3IfIoDly1 = crate::Reg<sf3_if_io_dly_1::Sf3IfIoDly1Spec>;
#[doc = "sf3_if_io_dly_1."]
pub mod sf3_if_io_dly_1;
#[doc = "sf3_if_io_dly_2 (rw) register accessor: sf3_if_io_dly_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf3_if_io_dly_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf3_if_io_dly_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf3_if_io_dly_2`] module"]
#[doc(alias = "sf3_if_io_dly_2")]
pub type Sf3IfIoDly2 = crate::Reg<sf3_if_io_dly_2::Sf3IfIoDly2Spec>;
#[doc = "sf3_if_io_dly_2."]
pub mod sf3_if_io_dly_2;
#[doc = "sf3_if_io_dly_3 (rw) register accessor: sf3_if_io_dly_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf3_if_io_dly_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf3_if_io_dly_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf3_if_io_dly_3`] module"]
#[doc(alias = "sf3_if_io_dly_3")]
pub type Sf3IfIoDly3 = crate::Reg<sf3_if_io_dly_3::Sf3IfIoDly3Spec>;
#[doc = "sf3_if_io_dly_3."]
pub mod sf3_if_io_dly_3;
#[doc = "sf3_if_io_dly_4 (rw) register accessor: sf3_if_io_dly_4.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf3_if_io_dly_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf3_if_io_dly_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf3_if_io_dly_4`] module"]
#[doc(alias = "sf3_if_io_dly_4")]
pub type Sf3IfIoDly4 = crate::Reg<sf3_if_io_dly_4::Sf3IfIoDly4Spec>;
#[doc = "sf3_if_io_dly_4."]
pub mod sf3_if_io_dly_4;
#[doc = "sf_ctrl_2 (rw) register accessor: sf_ctrl_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ctrl_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_ctrl_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_ctrl_2`] module"]
#[doc(alias = "sf_ctrl_2")]
pub type SfCtrl2 = crate::Reg<sf_ctrl_2::SfCtrl2Spec>;
#[doc = "sf_ctrl_2."]
pub mod sf_ctrl_2;
#[doc = "sf_ctrl_3 (rw) register accessor: sf_ctrl_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ctrl_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_ctrl_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_ctrl_3`] module"]
#[doc(alias = "sf_ctrl_3")]
pub type SfCtrl3 = crate::Reg<sf_ctrl_3::SfCtrl3Spec>;
#[doc = "sf_ctrl_3."]
pub mod sf_ctrl_3;
#[doc = "sf_if_iahb_3 (rw) register accessor: sf_if_iahb_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_iahb_3`] module"]
#[doc(alias = "sf_if_iahb_3")]
pub type SfIfIahb3 = crate::Reg<sf_if_iahb_3::SfIfIahb3Spec>;
#[doc = "sf_if_iahb_3."]
pub mod sf_if_iahb_3;
#[doc = "sf_if_iahb_4 (rw) register accessor: sf_if_iahb_4.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_iahb_4`] module"]
#[doc(alias = "sf_if_iahb_4")]
pub type SfIfIahb4 = crate::Reg<sf_if_iahb_4::SfIfIahb4Spec>;
#[doc = "sf_if_iahb_4."]
pub mod sf_if_iahb_4;
#[doc = "sf_if_iahb_5 (rw) register accessor: sf_if_iahb_5.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_iahb_5`] module"]
#[doc(alias = "sf_if_iahb_5")]
pub type SfIfIahb5 = crate::Reg<sf_if_iahb_5::SfIfIahb5Spec>;
#[doc = "sf_if_iahb_5."]
pub mod sf_if_iahb_5;
#[doc = "sf_if_iahb_6 (rw) register accessor: sf_if_iahb_6.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_iahb_6`] module"]
#[doc(alias = "sf_if_iahb_6")]
pub type SfIfIahb6 = crate::Reg<sf_if_iahb_6::SfIfIahb6Spec>;
#[doc = "sf_if_iahb_6."]
pub mod sf_if_iahb_6;
#[doc = "sf_if_iahb_7 (rw) register accessor: sf_if_iahb_7.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_iahb_7`] module"]
#[doc(alias = "sf_if_iahb_7")]
pub type SfIfIahb7 = crate::Reg<sf_if_iahb_7::SfIfIahb7Spec>;
#[doc = "sf_if_iahb_7."]
pub mod sf_if_iahb_7;
#[doc = "sf_ctrl_prot_en_rd (r) register accessor: sf_ctrl_prot_en_rd.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ctrl_prot_en_rd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_ctrl_prot_en_rd`] module"]
#[doc(alias = "sf_ctrl_prot_en_rd")]
pub type SfCtrlProtEnRd = crate::Reg<sf_ctrl_prot_en_rd::SfCtrlProtEnRdSpec>;
#[doc = "sf_ctrl_prot_en_rd."]
pub mod sf_ctrl_prot_en_rd;
#[doc = "sf_ctrl_prot_en (rw) register accessor: sf_ctrl_prot_en.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ctrl_prot_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_ctrl_prot_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_ctrl_prot_en`] module"]
#[doc(alias = "sf_ctrl_prot_en")]
pub type SfCtrlProtEn = crate::Reg<sf_ctrl_prot_en::SfCtrlProtEnSpec>;
#[doc = "sf_ctrl_prot_en."]
pub mod sf_ctrl_prot_en;
#[doc = "sf_aes_key_r0_0 (rw) register accessor: sf_aes_key_r0_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r0_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r0_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_key_r0_0`] module"]
#[doc(alias = "sf_aes_key_r0_0")]
pub type SfAesKeyR0_0 = crate::Reg<sf_aes_key_r0_0::SfAesKeyR0_0Spec>;
#[doc = "sf_aes_key_r0_0."]
pub mod sf_aes_key_r0_0;
#[doc = "sf_aes_key_r0_1 (rw) register accessor: sf_aes_key_r0_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r0_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r0_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_key_r0_1`] module"]
#[doc(alias = "sf_aes_key_r0_1")]
pub type SfAesKeyR0_1 = crate::Reg<sf_aes_key_r0_1::SfAesKeyR0_1Spec>;
#[doc = "sf_aes_key_r0_1."]
pub mod sf_aes_key_r0_1;
#[doc = "sf_aes_key_r0_2 (rw) register accessor: sf_aes_key_r0_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r0_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r0_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_key_r0_2`] module"]
#[doc(alias = "sf_aes_key_r0_2")]
pub type SfAesKeyR0_2 = crate::Reg<sf_aes_key_r0_2::SfAesKeyR0_2Spec>;
#[doc = "sf_aes_key_r0_2."]
pub mod sf_aes_key_r0_2;
#[doc = "sf_aes_key_r0_3 (rw) register accessor: sf_aes_key_r0_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r0_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r0_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_key_r0_3`] module"]
#[doc(alias = "sf_aes_key_r0_3")]
pub type SfAesKeyR0_3 = crate::Reg<sf_aes_key_r0_3::SfAesKeyR0_3Spec>;
#[doc = "sf_aes_key_r0_3."]
pub mod sf_aes_key_r0_3;
#[doc = "sf_aes_key_r0_4 (rw) register accessor: sf_aes_key_r0_4.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r0_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r0_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_key_r0_4`] module"]
#[doc(alias = "sf_aes_key_r0_4")]
pub type SfAesKeyR0_4 = crate::Reg<sf_aes_key_r0_4::SfAesKeyR0_4Spec>;
#[doc = "sf_aes_key_r0_4."]
pub mod sf_aes_key_r0_4;
#[doc = "sf_aes_key_r0_5 (rw) register accessor: sf_aes_key_r0_5.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r0_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r0_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_key_r0_5`] module"]
#[doc(alias = "sf_aes_key_r0_5")]
pub type SfAesKeyR0_5 = crate::Reg<sf_aes_key_r0_5::SfAesKeyR0_5Spec>;
#[doc = "sf_aes_key_r0_5."]
pub mod sf_aes_key_r0_5;
#[doc = "sf_aes_key_r0_6 (rw) register accessor: sf_aes_key_r0_6.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r0_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r0_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_key_r0_6`] module"]
#[doc(alias = "sf_aes_key_r0_6")]
pub type SfAesKeyR0_6 = crate::Reg<sf_aes_key_r0_6::SfAesKeyR0_6Spec>;
#[doc = "sf_aes_key_r0_6."]
pub mod sf_aes_key_r0_6;
#[doc = "sf_aes_key_r0_7 (rw) register accessor: sf_aes_key_r0_7.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r0_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r0_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_key_r0_7`] module"]
#[doc(alias = "sf_aes_key_r0_7")]
pub type SfAesKeyR0_7 = crate::Reg<sf_aes_key_r0_7::SfAesKeyR0_7Spec>;
#[doc = "sf_aes_key_r0_7."]
pub mod sf_aes_key_r0_7;
#[doc = "sf_aes_iv_r0_w0 (rw) register accessor: sf_aes_iv_r0_w0.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_iv_r0_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_iv_r0_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_iv_r0_w0`] module"]
#[doc(alias = "sf_aes_iv_r0_w0")]
pub type SfAesIvR0W0 = crate::Reg<sf_aes_iv_r0_w0::SfAesIvR0W0Spec>;
#[doc = "sf_aes_iv_r0_w0."]
pub mod sf_aes_iv_r0_w0;
#[doc = "sf_aes_iv_r0_w1 (rw) register accessor: sf_aes_iv_r0_w1.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_iv_r0_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_iv_r0_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_iv_r0_w1`] module"]
#[doc(alias = "sf_aes_iv_r0_w1")]
pub type SfAesIvR0W1 = crate::Reg<sf_aes_iv_r0_w1::SfAesIvR0W1Spec>;
#[doc = "sf_aes_iv_r0_w1."]
pub mod sf_aes_iv_r0_w1;
#[doc = "sf_aes_iv_r0_w2 (rw) register accessor: sf_aes_iv_r0_w2.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_iv_r0_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_iv_r0_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_iv_r0_w2`] module"]
#[doc(alias = "sf_aes_iv_r0_w2")]
pub type SfAesIvR0W2 = crate::Reg<sf_aes_iv_r0_w2::SfAesIvR0W2Spec>;
#[doc = "sf_aes_iv_r0_w2."]
pub mod sf_aes_iv_r0_w2;
#[doc = "sf_aes_iv_r0_w3 (rw) register accessor: sf_aes_iv_r0_w3.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_iv_r0_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_iv_r0_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_iv_r0_w3`] module"]
#[doc(alias = "sf_aes_iv_r0_w3")]
pub type SfAesIvR0W3 = crate::Reg<sf_aes_iv_r0_w3::SfAesIvR0W3Spec>;
#[doc = "sf_aes_iv_r0_w3."]
pub mod sf_aes_iv_r0_w3;
#[doc = "sf_aes_cfg_r0 (rw) register accessor: sf_aes_cfg_r0.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_cfg_r0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_cfg_r0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_cfg_r0`] module"]
#[doc(alias = "sf_aes_cfg_r0")]
pub type SfAesCfgR0 = crate::Reg<sf_aes_cfg_r0::SfAesCfgR0Spec>;
#[doc = "sf_aes_cfg_r0."]
pub mod sf_aes_cfg_r0;
#[doc = "sf_aes_key_r1_0 (rw) register accessor: sf_aes_key_r1_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r1_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r1_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_key_r1_0`] module"]
#[doc(alias = "sf_aes_key_r1_0")]
pub type SfAesKeyR1_0 = crate::Reg<sf_aes_key_r1_0::SfAesKeyR1_0Spec>;
#[doc = "sf_aes_key_r1_0."]
pub mod sf_aes_key_r1_0;
#[doc = "sf_aes_key_r1_1 (rw) register accessor: sf_aes_key_r1_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r1_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r1_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_key_r1_1`] module"]
#[doc(alias = "sf_aes_key_r1_1")]
pub type SfAesKeyR1_1 = crate::Reg<sf_aes_key_r1_1::SfAesKeyR1_1Spec>;
#[doc = "sf_aes_key_r1_1."]
pub mod sf_aes_key_r1_1;
#[doc = "sf_aes_key_r1_2 (rw) register accessor: sf_aes_key_r1_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r1_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r1_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_key_r1_2`] module"]
#[doc(alias = "sf_aes_key_r1_2")]
pub type SfAesKeyR1_2 = crate::Reg<sf_aes_key_r1_2::SfAesKeyR1_2Spec>;
#[doc = "sf_aes_key_r1_2."]
pub mod sf_aes_key_r1_2;
#[doc = "sf_aes_key_r1_3 (rw) register accessor: sf_aes_key_r1_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r1_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r1_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_key_r1_3`] module"]
#[doc(alias = "sf_aes_key_r1_3")]
pub type SfAesKeyR1_3 = crate::Reg<sf_aes_key_r1_3::SfAesKeyR1_3Spec>;
#[doc = "sf_aes_key_r1_3."]
pub mod sf_aes_key_r1_3;
#[doc = "sf_aes_key_r1_4 (rw) register accessor: sf_aes_key_r1_4.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r1_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r1_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_key_r1_4`] module"]
#[doc(alias = "sf_aes_key_r1_4")]
pub type SfAesKeyR1_4 = crate::Reg<sf_aes_key_r1_4::SfAesKeyR1_4Spec>;
#[doc = "sf_aes_key_r1_4."]
pub mod sf_aes_key_r1_4;
#[doc = "sf_aes_key_r1_5 (rw) register accessor: sf_aes_key_r1_5.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r1_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r1_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_key_r1_5`] module"]
#[doc(alias = "sf_aes_key_r1_5")]
pub type SfAesKeyR1_5 = crate::Reg<sf_aes_key_r1_5::SfAesKeyR1_5Spec>;
#[doc = "sf_aes_key_r1_5."]
pub mod sf_aes_key_r1_5;
#[doc = "sf_aes_key_r1_6 (rw) register accessor: sf_aes_key_r1_6.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r1_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r1_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_key_r1_6`] module"]
#[doc(alias = "sf_aes_key_r1_6")]
pub type SfAesKeyR1_6 = crate::Reg<sf_aes_key_r1_6::SfAesKeyR1_6Spec>;
#[doc = "sf_aes_key_r1_6."]
pub mod sf_aes_key_r1_6;
#[doc = "sf_aes_key_r1_7 (rw) register accessor: sf_aes_key_r1_7.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r1_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r1_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_key_r1_7`] module"]
#[doc(alias = "sf_aes_key_r1_7")]
pub type SfAesKeyR1_7 = crate::Reg<sf_aes_key_r1_7::SfAesKeyR1_7Spec>;
#[doc = "sf_aes_key_r1_7."]
pub mod sf_aes_key_r1_7;
#[doc = "sf_aes_iv_r1_w0 (rw) register accessor: sf_aes_iv_r1_w0.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_iv_r1_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_iv_r1_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_iv_r1_w0`] module"]
#[doc(alias = "sf_aes_iv_r1_w0")]
pub type SfAesIvR1W0 = crate::Reg<sf_aes_iv_r1_w0::SfAesIvR1W0Spec>;
#[doc = "sf_aes_iv_r1_w0."]
pub mod sf_aes_iv_r1_w0;
#[doc = "sf_aes_iv_r1_w1 (rw) register accessor: sf_aes_iv_r1_w1.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_iv_r1_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_iv_r1_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_iv_r1_w1`] module"]
#[doc(alias = "sf_aes_iv_r1_w1")]
pub type SfAesIvR1W1 = crate::Reg<sf_aes_iv_r1_w1::SfAesIvR1W1Spec>;
#[doc = "sf_aes_iv_r1_w1."]
pub mod sf_aes_iv_r1_w1;
#[doc = "sf_aes_iv_r1_w2 (rw) register accessor: sf_aes_iv_r1_w2.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_iv_r1_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_iv_r1_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_iv_r1_w2`] module"]
#[doc(alias = "sf_aes_iv_r1_w2")]
pub type SfAesIvR1W2 = crate::Reg<sf_aes_iv_r1_w2::SfAesIvR1W2Spec>;
#[doc = "sf_aes_iv_r1_w2."]
pub mod sf_aes_iv_r1_w2;
#[doc = "sf_aes_iv_r1_w3 (rw) register accessor: sf_aes_iv_r1_w3.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_iv_r1_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_iv_r1_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_iv_r1_w3`] module"]
#[doc(alias = "sf_aes_iv_r1_w3")]
pub type SfAesIvR1W3 = crate::Reg<sf_aes_iv_r1_w3::SfAesIvR1W3Spec>;
#[doc = "sf_aes_iv_r1_w3."]
pub mod sf_aes_iv_r1_w3;
#[doc = "sf_aes_r1 (rw) register accessor: sf_aes_r1.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_r1`] module"]
#[doc(alias = "sf_aes_r1")]
pub type SfAesR1 = crate::Reg<sf_aes_r1::SfAesR1Spec>;
#[doc = "sf_aes_r1."]
pub mod sf_aes_r1;
#[doc = "sf_aes_key_r2_0 (rw) register accessor: sf_aes_key_r2_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r2_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r2_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_key_r2_0`] module"]
#[doc(alias = "sf_aes_key_r2_0")]
pub type SfAesKeyR2_0 = crate::Reg<sf_aes_key_r2_0::SfAesKeyR2_0Spec>;
#[doc = "sf_aes_key_r2_0."]
pub mod sf_aes_key_r2_0;
#[doc = "sf_aes_key_r2_1 (rw) register accessor: sf_aes_key_r2_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r2_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r2_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_key_r2_1`] module"]
#[doc(alias = "sf_aes_key_r2_1")]
pub type SfAesKeyR2_1 = crate::Reg<sf_aes_key_r2_1::SfAesKeyR2_1Spec>;
#[doc = "sf_aes_key_r2_1."]
pub mod sf_aes_key_r2_1;
#[doc = "sf_aes_key_r2_2 (rw) register accessor: sf_aes_key_r2_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r2_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r2_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_key_r2_2`] module"]
#[doc(alias = "sf_aes_key_r2_2")]
pub type SfAesKeyR2_2 = crate::Reg<sf_aes_key_r2_2::SfAesKeyR2_2Spec>;
#[doc = "sf_aes_key_r2_2."]
pub mod sf_aes_key_r2_2;
#[doc = "sf_aes_key_r2_3 (rw) register accessor: sf_aes_key_r2_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r2_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r2_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_key_r2_3`] module"]
#[doc(alias = "sf_aes_key_r2_3")]
pub type SfAesKeyR2_3 = crate::Reg<sf_aes_key_r2_3::SfAesKeyR2_3Spec>;
#[doc = "sf_aes_key_r2_3."]
pub mod sf_aes_key_r2_3;
#[doc = "sf_aes_key_r2_4 (rw) register accessor: sf_aes_key_r2_4.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r2_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r2_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_key_r2_4`] module"]
#[doc(alias = "sf_aes_key_r2_4")]
pub type SfAesKeyR2_4 = crate::Reg<sf_aes_key_r2_4::SfAesKeyR2_4Spec>;
#[doc = "sf_aes_key_r2_4."]
pub mod sf_aes_key_r2_4;
#[doc = "sf_aes_key_r2_5 (rw) register accessor: sf_aes_key_r2_5.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r2_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r2_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_key_r2_5`] module"]
#[doc(alias = "sf_aes_key_r2_5")]
pub type SfAesKeyR2_5 = crate::Reg<sf_aes_key_r2_5::SfAesKeyR2_5Spec>;
#[doc = "sf_aes_key_r2_5."]
pub mod sf_aes_key_r2_5;
#[doc = "sf_aes_key_r2_6 (rw) register accessor: sf_aes_key_r2_6.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r2_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r2_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_key_r2_6`] module"]
#[doc(alias = "sf_aes_key_r2_6")]
pub type SfAesKeyR2_6 = crate::Reg<sf_aes_key_r2_6::SfAesKeyR2_6Spec>;
#[doc = "sf_aes_key_r2_6."]
pub mod sf_aes_key_r2_6;
#[doc = "sf_aes_key_r2_7 (rw) register accessor: sf_aes_key_r2_7.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_key_r2_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_key_r2_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_key_r2_7`] module"]
#[doc(alias = "sf_aes_key_r2_7")]
pub type SfAesKeyR2_7 = crate::Reg<sf_aes_key_r2_7::SfAesKeyR2_7Spec>;
#[doc = "sf_aes_key_r2_7."]
pub mod sf_aes_key_r2_7;
#[doc = "sf_aes_iv_r2_w0 (rw) register accessor: sf_aes_iv_r2_w0.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_iv_r2_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_iv_r2_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_iv_r2_w0`] module"]
#[doc(alias = "sf_aes_iv_r2_w0")]
pub type SfAesIvR2W0 = crate::Reg<sf_aes_iv_r2_w0::SfAesIvR2W0Spec>;
#[doc = "sf_aes_iv_r2_w0."]
pub mod sf_aes_iv_r2_w0;
#[doc = "sf_aes_iv_r2_w1 (rw) register accessor: sf_aes_iv_r2_w1.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_iv_r2_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_iv_r2_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_iv_r2_w1`] module"]
#[doc(alias = "sf_aes_iv_r2_w1")]
pub type SfAesIvR2W1 = crate::Reg<sf_aes_iv_r2_w1::SfAesIvR2W1Spec>;
#[doc = "sf_aes_iv_r2_w1."]
pub mod sf_aes_iv_r2_w1;
#[doc = "sf_aes_iv_r2_w2 (rw) register accessor: sf_aes_iv_r2_w2.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_iv_r2_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_iv_r2_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_iv_r2_w2`] module"]
#[doc(alias = "sf_aes_iv_r2_w2")]
pub type SfAesIvR2W2 = crate::Reg<sf_aes_iv_r2_w2::SfAesIvR2W2Spec>;
#[doc = "sf_aes_iv_r2_w2."]
pub mod sf_aes_iv_r2_w2;
#[doc = "sf_aes_iv_r2_w3 (rw) register accessor: sf_aes_iv_r2_w3.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_iv_r2_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_iv_r2_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_iv_r2_w3`] module"]
#[doc(alias = "sf_aes_iv_r2_w3")]
pub type SfAesIvR2W3 = crate::Reg<sf_aes_iv_r2_w3::SfAesIvR2W3Spec>;
#[doc = "sf_aes_iv_r2_w3."]
pub mod sf_aes_iv_r2_w3;
#[doc = "sf_aes_r2 (rw) register accessor: sf_aes_r2.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes_r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes_r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes_r2`] module"]
#[doc(alias = "sf_aes_r2")]
pub type SfAesR2 = crate::Reg<sf_aes_r2::SfAesR2Spec>;
#[doc = "sf_aes_r2."]
pub mod sf_aes_r2;
#[doc = "sf_id0_offset (rw) register accessor: sf_id0_offset.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_id0_offset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_id0_offset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_id0_offset`] module"]
#[doc(alias = "sf_id0_offset")]
pub type SfId0Offset = crate::Reg<sf_id0_offset::SfId0OffsetSpec>;
#[doc = "sf_id0_offset."]
pub mod sf_id0_offset;
#[doc = "sf_id1_offset (rw) register accessor: sf_id1_offset.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_id1_offset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_id1_offset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_id1_offset`] module"]
#[doc(alias = "sf_id1_offset")]
pub type SfId1Offset = crate::Reg<sf_id1_offset::SfId1OffsetSpec>;
#[doc = "sf_id1_offset."]
pub mod sf_id1_offset;
