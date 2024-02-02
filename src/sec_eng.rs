#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    se_sha_0_ctrl: SeSha0Ctrl,
    se_sha_0_msa: SeSha0Msa,
    se_sha_0_status: SeSha0Status,
    se_sha_0_endian: SeSha0Endian,
    se_sha_0_hash_l_0: SeSha0HashL0,
    se_sha_0_hash_l_1: SeSha0HashL1,
    se_sha_0_hash_l_2: SeSha0HashL2,
    se_sha_0_hash_l_3: SeSha0HashL3,
    se_sha_0_hash_l_4: SeSha0HashL4,
    se_sha_0_hash_l_5: SeSha0HashL5,
    se_sha_0_hash_l_6: SeSha0HashL6,
    se_sha_0_hash_l_7: SeSha0HashL7,
    se_sha_0_hash_h_0: SeSha0HashH0,
    se_sha_0_hash_h_1: SeSha0HashH1,
    se_sha_0_hash_h_2: SeSha0HashH2,
    se_sha_0_hash_h_3: SeSha0HashH3,
    se_sha_0_hash_h_4: SeSha0HashH4,
    se_sha_0_hash_h_5: SeSha0HashH5,
    se_sha_0_hash_h_6: SeSha0HashH6,
    se_sha_0_hash_h_7: SeSha0HashH7,
    se_sha_0_link: SeSha0Link,
    _reserved21: [u8; 0xa8],
    se_sha_0_ctrl_prot: SeSha0CtrlProt,
    se_aes_0_ctrl: SeAes0Ctrl,
    se_aes_0_msa: SeAes0Msa,
    se_aes_0_mda: SeAes0Mda,
    se_aes_0_status: SeAes0Status,
    se_aes_0_iv_0: SeAes0Iv0,
    se_aes_0_iv_1: SeAes0Iv1,
    se_aes_0_iv_2: SeAes0Iv2,
    se_aes_0_iv_3: SeAes0Iv3,
    se_aes_0_key_0: SeAes0Key0,
    se_aes_0_key_1: SeAes0Key1,
    se_aes_0_key_2: SeAes0Key2,
    se_aes_0_key_3: SeAes0Key3,
    se_aes_0_key_4: SeAes0Key4,
    se_aes_0_key_5: SeAes0Key5,
    se_aes_0_key_6: SeAes0Key6,
    se_aes_0_key_7: SeAes0Key7,
    se_aes_0_key_sel_0: SeAes0KeySel0,
    se_aes_0_key_sel_1: SeAes0KeySel1,
    se_aes_0_endian: SeAes0Endian,
    se_aes_0_sboot: SeAes0Sboot,
    se_aes_0_link: SeAes0Link,
    _reserved43: [u8; 0xa8],
    se_aes_0_ctrl_prot: SeAes0CtrlProt,
    se_trng_0_ctrl_0: SeTrng0Ctrl0,
    se_trng_0_status: SeTrng0Status,
    se_trng_0_dout_0: SeTrng0Dout0,
    se_trng_0_dout_1: SeTrng0Dout1,
    se_trng_0_dout_2: SeTrng0Dout2,
    se_trng_0_dout_3: SeTrng0Dout3,
    se_trng_0_dout_4: SeTrng0Dout4,
    se_trng_0_dout_5: SeTrng0Dout5,
    se_trng_0_dout_6: SeTrng0Dout6,
    se_trng_0_dout_7: SeTrng0Dout7,
    se_trng_0_test: SeTrng0Test,
    se_trng_0_ctrl_1: SeTrng0Ctrl1,
    se_trng_0_ctrl_2: SeTrng0Ctrl2,
    se_trng_0_ctrl_3: SeTrng0Ctrl3,
    _reserved58: [u8; 0x08],
    se_trng_0_test_out_0: SeTrng0TestOut0,
    se_trng_0_test_out_1: SeTrng0TestOut1,
    se_trng_0_test_out_2: SeTrng0TestOut2,
    se_trng_0_test_out_3: SeTrng0TestOut3,
    _reserved62: [u8; 0xac],
    se_trng_0_ctrl_prot: SeTrng0CtrlProt,
    se_pka_0_ctrl_0: SePka0Ctrl0,
    _reserved64: [u8; 0x08],
    se_pka_0_seed: SePka0Seed,
    se_pka_0_ctrl_1: SePka0Ctrl1,
    _reserved66: [u8; 0x2c],
    se_pka_0_rw: SePka0Rw,
    _reserved67: [u8; 0x1c],
    se_pka_0_rw_burst: SePka0RwBurst,
    _reserved68: [u8; 0x98],
    se_pka_0_ctrl_prot: SePka0CtrlProt,
    se_cdet_0_ctrl_0: SeCdet0Ctrl0,
    se_cdet_0_ctrl_1: SeCdet0Ctrl1,
    _reserved71: [u8; 0xf4],
    se_cdet_0_ctrl_prot: SeCdet0CtrlProt,
    se_gmac_0_ctrl_0: SeGmac0Ctrl0,
    se_gmac_0_lca: SeGmac0Lca,
    se_gmac_0_status: SeGmac0Status,
    _reserved75: [u8; 0xf0],
    se_gmac_0_ctrl_prot: SeGmac0CtrlProt,
    _reserved76: [u8; 0x0900],
    se_ctrl_prot_rd: SeCtrlProtRd,
    se_ctrl_reserved_0: SeCtrlReserved0,
    se_ctrl_reserved_1: SeCtrlReserved1,
    se_ctrl_reserved_2: SeCtrlReserved2,
}
impl RegisterBlock {
    #[doc = "0x00 - se_sha_0_ctrl."]
    #[inline(always)]
    pub const fn se_sha_0_ctrl(&self) -> &SeSha0Ctrl {
        &self.se_sha_0_ctrl
    }
    #[doc = "0x04 - se_sha_0_msa."]
    #[inline(always)]
    pub const fn se_sha_0_msa(&self) -> &SeSha0Msa {
        &self.se_sha_0_msa
    }
    #[doc = "0x08 - se_sha_0_status."]
    #[inline(always)]
    pub const fn se_sha_0_status(&self) -> &SeSha0Status {
        &self.se_sha_0_status
    }
    #[doc = "0x0c - se_sha_0_endian."]
    #[inline(always)]
    pub const fn se_sha_0_endian(&self) -> &SeSha0Endian {
        &self.se_sha_0_endian
    }
    #[doc = "0x10 - se_sha_0_hash_l_0."]
    #[inline(always)]
    pub const fn se_sha_0_hash_l_0(&self) -> &SeSha0HashL0 {
        &self.se_sha_0_hash_l_0
    }
    #[doc = "0x14 - se_sha_0_hash_l_1."]
    #[inline(always)]
    pub const fn se_sha_0_hash_l_1(&self) -> &SeSha0HashL1 {
        &self.se_sha_0_hash_l_1
    }
    #[doc = "0x18 - se_sha_0_hash_l_2."]
    #[inline(always)]
    pub const fn se_sha_0_hash_l_2(&self) -> &SeSha0HashL2 {
        &self.se_sha_0_hash_l_2
    }
    #[doc = "0x1c - se_sha_0_hash_l_3."]
    #[inline(always)]
    pub const fn se_sha_0_hash_l_3(&self) -> &SeSha0HashL3 {
        &self.se_sha_0_hash_l_3
    }
    #[doc = "0x20 - se_sha_0_hash_l_4."]
    #[inline(always)]
    pub const fn se_sha_0_hash_l_4(&self) -> &SeSha0HashL4 {
        &self.se_sha_0_hash_l_4
    }
    #[doc = "0x24 - se_sha_0_hash_l_5."]
    #[inline(always)]
    pub const fn se_sha_0_hash_l_5(&self) -> &SeSha0HashL5 {
        &self.se_sha_0_hash_l_5
    }
    #[doc = "0x28 - se_sha_0_hash_l_6."]
    #[inline(always)]
    pub const fn se_sha_0_hash_l_6(&self) -> &SeSha0HashL6 {
        &self.se_sha_0_hash_l_6
    }
    #[doc = "0x2c - se_sha_0_hash_l_7."]
    #[inline(always)]
    pub const fn se_sha_0_hash_l_7(&self) -> &SeSha0HashL7 {
        &self.se_sha_0_hash_l_7
    }
    #[doc = "0x30 - se_sha_0_hash_h_0."]
    #[inline(always)]
    pub const fn se_sha_0_hash_h_0(&self) -> &SeSha0HashH0 {
        &self.se_sha_0_hash_h_0
    }
    #[doc = "0x34 - se_sha_0_hash_h_1."]
    #[inline(always)]
    pub const fn se_sha_0_hash_h_1(&self) -> &SeSha0HashH1 {
        &self.se_sha_0_hash_h_1
    }
    #[doc = "0x38 - se_sha_0_hash_h_2."]
    #[inline(always)]
    pub const fn se_sha_0_hash_h_2(&self) -> &SeSha0HashH2 {
        &self.se_sha_0_hash_h_2
    }
    #[doc = "0x3c - se_sha_0_hash_h_3."]
    #[inline(always)]
    pub const fn se_sha_0_hash_h_3(&self) -> &SeSha0HashH3 {
        &self.se_sha_0_hash_h_3
    }
    #[doc = "0x40 - se_sha_0_hash_h_4."]
    #[inline(always)]
    pub const fn se_sha_0_hash_h_4(&self) -> &SeSha0HashH4 {
        &self.se_sha_0_hash_h_4
    }
    #[doc = "0x44 - se_sha_0_hash_h_5."]
    #[inline(always)]
    pub const fn se_sha_0_hash_h_5(&self) -> &SeSha0HashH5 {
        &self.se_sha_0_hash_h_5
    }
    #[doc = "0x48 - se_sha_0_hash_h_6."]
    #[inline(always)]
    pub const fn se_sha_0_hash_h_6(&self) -> &SeSha0HashH6 {
        &self.se_sha_0_hash_h_6
    }
    #[doc = "0x4c - se_sha_0_hash_h_7."]
    #[inline(always)]
    pub const fn se_sha_0_hash_h_7(&self) -> &SeSha0HashH7 {
        &self.se_sha_0_hash_h_7
    }
    #[doc = "0x50 - se_sha_0_link."]
    #[inline(always)]
    pub const fn se_sha_0_link(&self) -> &SeSha0Link {
        &self.se_sha_0_link
    }
    #[doc = "0xfc - se_sha_0_ctrl_prot."]
    #[inline(always)]
    pub const fn se_sha_0_ctrl_prot(&self) -> &SeSha0CtrlProt {
        &self.se_sha_0_ctrl_prot
    }
    #[doc = "0x100 - se_aes_0_ctrl."]
    #[inline(always)]
    pub const fn se_aes_0_ctrl(&self) -> &SeAes0Ctrl {
        &self.se_aes_0_ctrl
    }
    #[doc = "0x104 - se_aes_0_msa."]
    #[inline(always)]
    pub const fn se_aes_0_msa(&self) -> &SeAes0Msa {
        &self.se_aes_0_msa
    }
    #[doc = "0x108 - se_aes_0_mda."]
    #[inline(always)]
    pub const fn se_aes_0_mda(&self) -> &SeAes0Mda {
        &self.se_aes_0_mda
    }
    #[doc = "0x10c - se_aes_0_status."]
    #[inline(always)]
    pub const fn se_aes_0_status(&self) -> &SeAes0Status {
        &self.se_aes_0_status
    }
    #[doc = "0x110 - se_aes_0_iv_0."]
    #[inline(always)]
    pub const fn se_aes_0_iv_0(&self) -> &SeAes0Iv0 {
        &self.se_aes_0_iv_0
    }
    #[doc = "0x114 - se_aes_0_iv_1."]
    #[inline(always)]
    pub const fn se_aes_0_iv_1(&self) -> &SeAes0Iv1 {
        &self.se_aes_0_iv_1
    }
    #[doc = "0x118 - se_aes_0_iv_2."]
    #[inline(always)]
    pub const fn se_aes_0_iv_2(&self) -> &SeAes0Iv2 {
        &self.se_aes_0_iv_2
    }
    #[doc = "0x11c - se_aes_0_iv_3."]
    #[inline(always)]
    pub const fn se_aes_0_iv_3(&self) -> &SeAes0Iv3 {
        &self.se_aes_0_iv_3
    }
    #[doc = "0x120 - se_aes_0_key_0."]
    #[inline(always)]
    pub const fn se_aes_0_key_0(&self) -> &SeAes0Key0 {
        &self.se_aes_0_key_0
    }
    #[doc = "0x124 - se_aes_0_key_1."]
    #[inline(always)]
    pub const fn se_aes_0_key_1(&self) -> &SeAes0Key1 {
        &self.se_aes_0_key_1
    }
    #[doc = "0x128 - se_aes_0_key_2."]
    #[inline(always)]
    pub const fn se_aes_0_key_2(&self) -> &SeAes0Key2 {
        &self.se_aes_0_key_2
    }
    #[doc = "0x12c - se_aes_0_key_3."]
    #[inline(always)]
    pub const fn se_aes_0_key_3(&self) -> &SeAes0Key3 {
        &self.se_aes_0_key_3
    }
    #[doc = "0x130 - se_aes_0_key_4."]
    #[inline(always)]
    pub const fn se_aes_0_key_4(&self) -> &SeAes0Key4 {
        &self.se_aes_0_key_4
    }
    #[doc = "0x134 - se_aes_0_key_5."]
    #[inline(always)]
    pub const fn se_aes_0_key_5(&self) -> &SeAes0Key5 {
        &self.se_aes_0_key_5
    }
    #[doc = "0x138 - se_aes_0_key_6."]
    #[inline(always)]
    pub const fn se_aes_0_key_6(&self) -> &SeAes0Key6 {
        &self.se_aes_0_key_6
    }
    #[doc = "0x13c - se_aes_0_key_7."]
    #[inline(always)]
    pub const fn se_aes_0_key_7(&self) -> &SeAes0Key7 {
        &self.se_aes_0_key_7
    }
    #[doc = "0x140 - se_aes_0_key_sel_0."]
    #[inline(always)]
    pub const fn se_aes_0_key_sel_0(&self) -> &SeAes0KeySel0 {
        &self.se_aes_0_key_sel_0
    }
    #[doc = "0x144 - se_aes_0_key_sel_1."]
    #[inline(always)]
    pub const fn se_aes_0_key_sel_1(&self) -> &SeAes0KeySel1 {
        &self.se_aes_0_key_sel_1
    }
    #[doc = "0x148 - se_aes_0_endian."]
    #[inline(always)]
    pub const fn se_aes_0_endian(&self) -> &SeAes0Endian {
        &self.se_aes_0_endian
    }
    #[doc = "0x14c - se_aes_0_sboot."]
    #[inline(always)]
    pub const fn se_aes_0_sboot(&self) -> &SeAes0Sboot {
        &self.se_aes_0_sboot
    }
    #[doc = "0x150 - se_aes_0_link."]
    #[inline(always)]
    pub const fn se_aes_0_link(&self) -> &SeAes0Link {
        &self.se_aes_0_link
    }
    #[doc = "0x1fc - se_aes_0_ctrl_prot."]
    #[inline(always)]
    pub const fn se_aes_0_ctrl_prot(&self) -> &SeAes0CtrlProt {
        &self.se_aes_0_ctrl_prot
    }
    #[doc = "0x200 - se_trng_0_ctrl_0."]
    #[inline(always)]
    pub const fn se_trng_0_ctrl_0(&self) -> &SeTrng0Ctrl0 {
        &self.se_trng_0_ctrl_0
    }
    #[doc = "0x204 - se_trng_0_status."]
    #[inline(always)]
    pub const fn se_trng_0_status(&self) -> &SeTrng0Status {
        &self.se_trng_0_status
    }
    #[doc = "0x208 - se_trng_0_dout_0."]
    #[inline(always)]
    pub const fn se_trng_0_dout_0(&self) -> &SeTrng0Dout0 {
        &self.se_trng_0_dout_0
    }
    #[doc = "0x20c - se_trng_0_dout_1."]
    #[inline(always)]
    pub const fn se_trng_0_dout_1(&self) -> &SeTrng0Dout1 {
        &self.se_trng_0_dout_1
    }
    #[doc = "0x210 - se_trng_0_dout_2."]
    #[inline(always)]
    pub const fn se_trng_0_dout_2(&self) -> &SeTrng0Dout2 {
        &self.se_trng_0_dout_2
    }
    #[doc = "0x214 - se_trng_0_dout_3."]
    #[inline(always)]
    pub const fn se_trng_0_dout_3(&self) -> &SeTrng0Dout3 {
        &self.se_trng_0_dout_3
    }
    #[doc = "0x218 - se_trng_0_dout_4."]
    #[inline(always)]
    pub const fn se_trng_0_dout_4(&self) -> &SeTrng0Dout4 {
        &self.se_trng_0_dout_4
    }
    #[doc = "0x21c - se_trng_0_dout_5."]
    #[inline(always)]
    pub const fn se_trng_0_dout_5(&self) -> &SeTrng0Dout5 {
        &self.se_trng_0_dout_5
    }
    #[doc = "0x220 - se_trng_0_dout_6."]
    #[inline(always)]
    pub const fn se_trng_0_dout_6(&self) -> &SeTrng0Dout6 {
        &self.se_trng_0_dout_6
    }
    #[doc = "0x224 - se_trng_0_dout_7."]
    #[inline(always)]
    pub const fn se_trng_0_dout_7(&self) -> &SeTrng0Dout7 {
        &self.se_trng_0_dout_7
    }
    #[doc = "0x228 - se_trng_0_test."]
    #[inline(always)]
    pub const fn se_trng_0_test(&self) -> &SeTrng0Test {
        &self.se_trng_0_test
    }
    #[doc = "0x22c - se_trng_0_ctrl_1."]
    #[inline(always)]
    pub const fn se_trng_0_ctrl_1(&self) -> &SeTrng0Ctrl1 {
        &self.se_trng_0_ctrl_1
    }
    #[doc = "0x230 - se_trng_0_ctrl_2."]
    #[inline(always)]
    pub const fn se_trng_0_ctrl_2(&self) -> &SeTrng0Ctrl2 {
        &self.se_trng_0_ctrl_2
    }
    #[doc = "0x234 - se_trng_0_ctrl_3."]
    #[inline(always)]
    pub const fn se_trng_0_ctrl_3(&self) -> &SeTrng0Ctrl3 {
        &self.se_trng_0_ctrl_3
    }
    #[doc = "0x240 - se_trng_0_test_out_0."]
    #[inline(always)]
    pub const fn se_trng_0_test_out_0(&self) -> &SeTrng0TestOut0 {
        &self.se_trng_0_test_out_0
    }
    #[doc = "0x244 - se_trng_0_test_out_1."]
    #[inline(always)]
    pub const fn se_trng_0_test_out_1(&self) -> &SeTrng0TestOut1 {
        &self.se_trng_0_test_out_1
    }
    #[doc = "0x248 - se_trng_0_test_out_2."]
    #[inline(always)]
    pub const fn se_trng_0_test_out_2(&self) -> &SeTrng0TestOut2 {
        &self.se_trng_0_test_out_2
    }
    #[doc = "0x24c - se_trng_0_test_out_3."]
    #[inline(always)]
    pub const fn se_trng_0_test_out_3(&self) -> &SeTrng0TestOut3 {
        &self.se_trng_0_test_out_3
    }
    #[doc = "0x2fc - se_trng_0_ctrl_prot."]
    #[inline(always)]
    pub const fn se_trng_0_ctrl_prot(&self) -> &SeTrng0CtrlProt {
        &self.se_trng_0_ctrl_prot
    }
    #[doc = "0x300 - se_pka_0_ctrl_0."]
    #[inline(always)]
    pub const fn se_pka_0_ctrl_0(&self) -> &SePka0Ctrl0 {
        &self.se_pka_0_ctrl_0
    }
    #[doc = "0x30c - se_pka_0_seed."]
    #[inline(always)]
    pub const fn se_pka_0_seed(&self) -> &SePka0Seed {
        &self.se_pka_0_seed
    }
    #[doc = "0x310 - se_pka_0_ctrl_1."]
    #[inline(always)]
    pub const fn se_pka_0_ctrl_1(&self) -> &SePka0Ctrl1 {
        &self.se_pka_0_ctrl_1
    }
    #[doc = "0x340 - se_pka_0_rw."]
    #[inline(always)]
    pub const fn se_pka_0_rw(&self) -> &SePka0Rw {
        &self.se_pka_0_rw
    }
    #[doc = "0x360 - se_pka_0_rw_burst."]
    #[inline(always)]
    pub const fn se_pka_0_rw_burst(&self) -> &SePka0RwBurst {
        &self.se_pka_0_rw_burst
    }
    #[doc = "0x3fc - se_pka_0_ctrl_prot."]
    #[inline(always)]
    pub const fn se_pka_0_ctrl_prot(&self) -> &SePka0CtrlProt {
        &self.se_pka_0_ctrl_prot
    }
    #[doc = "0x400 - se_cdet_0_ctrl_0."]
    #[inline(always)]
    pub const fn se_cdet_0_ctrl_0(&self) -> &SeCdet0Ctrl0 {
        &self.se_cdet_0_ctrl_0
    }
    #[doc = "0x404 - se_cdet_0_ctrl_1."]
    #[inline(always)]
    pub const fn se_cdet_0_ctrl_1(&self) -> &SeCdet0Ctrl1 {
        &self.se_cdet_0_ctrl_1
    }
    #[doc = "0x4fc - se_cdet_0_ctrl_prot."]
    #[inline(always)]
    pub const fn se_cdet_0_ctrl_prot(&self) -> &SeCdet0CtrlProt {
        &self.se_cdet_0_ctrl_prot
    }
    #[doc = "0x500 - se_gmac_0_ctrl_0."]
    #[inline(always)]
    pub const fn se_gmac_0_ctrl_0(&self) -> &SeGmac0Ctrl0 {
        &self.se_gmac_0_ctrl_0
    }
    #[doc = "0x504 - se_gmac_0_lca."]
    #[inline(always)]
    pub const fn se_gmac_0_lca(&self) -> &SeGmac0Lca {
        &self.se_gmac_0_lca
    }
    #[doc = "0x508 - se_gmac_0_status."]
    #[inline(always)]
    pub const fn se_gmac_0_status(&self) -> &SeGmac0Status {
        &self.se_gmac_0_status
    }
    #[doc = "0x5fc - se_gmac_0_ctrl_prot."]
    #[inline(always)]
    pub const fn se_gmac_0_ctrl_prot(&self) -> &SeGmac0CtrlProt {
        &self.se_gmac_0_ctrl_prot
    }
    #[doc = "0xf00 - se_ctrl_prot_rd."]
    #[inline(always)]
    pub const fn se_ctrl_prot_rd(&self) -> &SeCtrlProtRd {
        &self.se_ctrl_prot_rd
    }
    #[doc = "0xf04 - se_ctrl_reserved_0."]
    #[inline(always)]
    pub const fn se_ctrl_reserved_0(&self) -> &SeCtrlReserved0 {
        &self.se_ctrl_reserved_0
    }
    #[doc = "0xf08 - se_ctrl_reserved_1."]
    #[inline(always)]
    pub const fn se_ctrl_reserved_1(&self) -> &SeCtrlReserved1 {
        &self.se_ctrl_reserved_1
    }
    #[doc = "0xf0c - se_ctrl_reserved_2."]
    #[inline(always)]
    pub const fn se_ctrl_reserved_2(&self) -> &SeCtrlReserved2 {
        &self.se_ctrl_reserved_2
    }
}
#[doc = "se_sha_0_ctrl (rw) register accessor: se_sha_0_ctrl.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_sha_0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_ctrl`] module"]
#[doc(alias = "se_sha_0_ctrl")]
pub type SeSha0Ctrl = crate::Reg<se_sha_0_ctrl::SeSha0CtrlSpec>;
#[doc = "se_sha_0_ctrl."]
pub mod se_sha_0_ctrl;
#[doc = "se_sha_0_msa (rw) register accessor: se_sha_0_msa.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_msa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_sha_0_msa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_msa`] module"]
#[doc(alias = "se_sha_0_msa")]
pub type SeSha0Msa = crate::Reg<se_sha_0_msa::SeSha0MsaSpec>;
#[doc = "se_sha_0_msa."]
pub mod se_sha_0_msa;
#[doc = "se_sha_0_status (r) register accessor: se_sha_0_status.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_status`] module"]
#[doc(alias = "se_sha_0_status")]
pub type SeSha0Status = crate::Reg<se_sha_0_status::SeSha0StatusSpec>;
#[doc = "se_sha_0_status."]
pub mod se_sha_0_status;
#[doc = "se_sha_0_endian (rw) register accessor: se_sha_0_endian.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_endian::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_sha_0_endian::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_endian`] module"]
#[doc(alias = "se_sha_0_endian")]
pub type SeSha0Endian = crate::Reg<se_sha_0_endian::SeSha0EndianSpec>;
#[doc = "se_sha_0_endian."]
pub mod se_sha_0_endian;
#[doc = "se_sha_0_hash_l_0 (r) register accessor: se_sha_0_hash_l_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_l_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_hash_l_0`] module"]
#[doc(alias = "se_sha_0_hash_l_0")]
pub type SeSha0HashL0 = crate::Reg<se_sha_0_hash_l_0::SeSha0HashL0Spec>;
#[doc = "se_sha_0_hash_l_0."]
pub mod se_sha_0_hash_l_0;
#[doc = "se_sha_0_hash_l_1 (r) register accessor: se_sha_0_hash_l_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_l_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_hash_l_1`] module"]
#[doc(alias = "se_sha_0_hash_l_1")]
pub type SeSha0HashL1 = crate::Reg<se_sha_0_hash_l_1::SeSha0HashL1Spec>;
#[doc = "se_sha_0_hash_l_1."]
pub mod se_sha_0_hash_l_1;
#[doc = "se_sha_0_hash_l_2 (r) register accessor: se_sha_0_hash_l_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_l_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_hash_l_2`] module"]
#[doc(alias = "se_sha_0_hash_l_2")]
pub type SeSha0HashL2 = crate::Reg<se_sha_0_hash_l_2::SeSha0HashL2Spec>;
#[doc = "se_sha_0_hash_l_2."]
pub mod se_sha_0_hash_l_2;
#[doc = "se_sha_0_hash_l_3 (r) register accessor: se_sha_0_hash_l_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_l_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_hash_l_3`] module"]
#[doc(alias = "se_sha_0_hash_l_3")]
pub type SeSha0HashL3 = crate::Reg<se_sha_0_hash_l_3::SeSha0HashL3Spec>;
#[doc = "se_sha_0_hash_l_3."]
pub mod se_sha_0_hash_l_3;
#[doc = "se_sha_0_hash_l_4 (r) register accessor: se_sha_0_hash_l_4.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_l_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_hash_l_4`] module"]
#[doc(alias = "se_sha_0_hash_l_4")]
pub type SeSha0HashL4 = crate::Reg<se_sha_0_hash_l_4::SeSha0HashL4Spec>;
#[doc = "se_sha_0_hash_l_4."]
pub mod se_sha_0_hash_l_4;
#[doc = "se_sha_0_hash_l_5 (r) register accessor: se_sha_0_hash_l_5.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_l_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_hash_l_5`] module"]
#[doc(alias = "se_sha_0_hash_l_5")]
pub type SeSha0HashL5 = crate::Reg<se_sha_0_hash_l_5::SeSha0HashL5Spec>;
#[doc = "se_sha_0_hash_l_5."]
pub mod se_sha_0_hash_l_5;
#[doc = "se_sha_0_hash_l_6 (r) register accessor: se_sha_0_hash_l_6.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_l_6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_hash_l_6`] module"]
#[doc(alias = "se_sha_0_hash_l_6")]
pub type SeSha0HashL6 = crate::Reg<se_sha_0_hash_l_6::SeSha0HashL6Spec>;
#[doc = "se_sha_0_hash_l_6."]
pub mod se_sha_0_hash_l_6;
#[doc = "se_sha_0_hash_l_7 (r) register accessor: se_sha_0_hash_l_7.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_l_7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_hash_l_7`] module"]
#[doc(alias = "se_sha_0_hash_l_7")]
pub type SeSha0HashL7 = crate::Reg<se_sha_0_hash_l_7::SeSha0HashL7Spec>;
#[doc = "se_sha_0_hash_l_7."]
pub mod se_sha_0_hash_l_7;
#[doc = "se_sha_0_hash_h_0 (r) register accessor: se_sha_0_hash_h_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_h_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_hash_h_0`] module"]
#[doc(alias = "se_sha_0_hash_h_0")]
pub type SeSha0HashH0 = crate::Reg<se_sha_0_hash_h_0::SeSha0HashH0Spec>;
#[doc = "se_sha_0_hash_h_0."]
pub mod se_sha_0_hash_h_0;
#[doc = "se_sha_0_hash_h_1 (r) register accessor: se_sha_0_hash_h_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_h_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_hash_h_1`] module"]
#[doc(alias = "se_sha_0_hash_h_1")]
pub type SeSha0HashH1 = crate::Reg<se_sha_0_hash_h_1::SeSha0HashH1Spec>;
#[doc = "se_sha_0_hash_h_1."]
pub mod se_sha_0_hash_h_1;
#[doc = "se_sha_0_hash_h_2 (r) register accessor: se_sha_0_hash_h_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_h_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_hash_h_2`] module"]
#[doc(alias = "se_sha_0_hash_h_2")]
pub type SeSha0HashH2 = crate::Reg<se_sha_0_hash_h_2::SeSha0HashH2Spec>;
#[doc = "se_sha_0_hash_h_2."]
pub mod se_sha_0_hash_h_2;
#[doc = "se_sha_0_hash_h_3 (r) register accessor: se_sha_0_hash_h_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_h_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_hash_h_3`] module"]
#[doc(alias = "se_sha_0_hash_h_3")]
pub type SeSha0HashH3 = crate::Reg<se_sha_0_hash_h_3::SeSha0HashH3Spec>;
#[doc = "se_sha_0_hash_h_3."]
pub mod se_sha_0_hash_h_3;
#[doc = "se_sha_0_hash_h_4 (r) register accessor: se_sha_0_hash_h_4.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_h_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_hash_h_4`] module"]
#[doc(alias = "se_sha_0_hash_h_4")]
pub type SeSha0HashH4 = crate::Reg<se_sha_0_hash_h_4::SeSha0HashH4Spec>;
#[doc = "se_sha_0_hash_h_4."]
pub mod se_sha_0_hash_h_4;
#[doc = "se_sha_0_hash_h_5 (r) register accessor: se_sha_0_hash_h_5.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_h_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_hash_h_5`] module"]
#[doc(alias = "se_sha_0_hash_h_5")]
pub type SeSha0HashH5 = crate::Reg<se_sha_0_hash_h_5::SeSha0HashH5Spec>;
#[doc = "se_sha_0_hash_h_5."]
pub mod se_sha_0_hash_h_5;
#[doc = "se_sha_0_hash_h_6 (r) register accessor: se_sha_0_hash_h_6.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_h_6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_hash_h_6`] module"]
#[doc(alias = "se_sha_0_hash_h_6")]
pub type SeSha0HashH6 = crate::Reg<se_sha_0_hash_h_6::SeSha0HashH6Spec>;
#[doc = "se_sha_0_hash_h_6."]
pub mod se_sha_0_hash_h_6;
#[doc = "se_sha_0_hash_h_7 (r) register accessor: se_sha_0_hash_h_7.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_h_7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_hash_h_7`] module"]
#[doc(alias = "se_sha_0_hash_h_7")]
pub type SeSha0HashH7 = crate::Reg<se_sha_0_hash_h_7::SeSha0HashH7Spec>;
#[doc = "se_sha_0_hash_h_7."]
pub mod se_sha_0_hash_h_7;
#[doc = "se_sha_0_link (rw) register accessor: se_sha_0_link.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_sha_0_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_link`] module"]
#[doc(alias = "se_sha_0_link")]
pub type SeSha0Link = crate::Reg<se_sha_0_link::SeSha0LinkSpec>;
#[doc = "se_sha_0_link."]
pub mod se_sha_0_link;
#[doc = "se_sha_0_ctrl_prot (rw) register accessor: se_sha_0_ctrl_prot.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_ctrl_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_sha_0_ctrl_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_ctrl_prot`] module"]
#[doc(alias = "se_sha_0_ctrl_prot")]
pub type SeSha0CtrlProt = crate::Reg<se_sha_0_ctrl_prot::SeSha0CtrlProtSpec>;
#[doc = "se_sha_0_ctrl_prot."]
pub mod se_sha_0_ctrl_prot;
#[doc = "se_aes_0_ctrl (rw) register accessor: se_aes_0_ctrl.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_ctrl`] module"]
#[doc(alias = "se_aes_0_ctrl")]
pub type SeAes0Ctrl = crate::Reg<se_aes_0_ctrl::SeAes0CtrlSpec>;
#[doc = "se_aes_0_ctrl."]
pub mod se_aes_0_ctrl;
#[doc = "se_aes_0_msa (rw) register accessor: se_aes_0_msa.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_msa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_msa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_msa`] module"]
#[doc(alias = "se_aes_0_msa")]
pub type SeAes0Msa = crate::Reg<se_aes_0_msa::SeAes0MsaSpec>;
#[doc = "se_aes_0_msa."]
pub mod se_aes_0_msa;
#[doc = "se_aes_0_mda (rw) register accessor: se_aes_0_mda.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_mda::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_mda::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_mda`] module"]
#[doc(alias = "se_aes_0_mda")]
pub type SeAes0Mda = crate::Reg<se_aes_0_mda::SeAes0MdaSpec>;
#[doc = "se_aes_0_mda."]
pub mod se_aes_0_mda;
#[doc = "se_aes_0_status (r) register accessor: se_aes_0_status.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_status`] module"]
#[doc(alias = "se_aes_0_status")]
pub type SeAes0Status = crate::Reg<se_aes_0_status::SeAes0StatusSpec>;
#[doc = "se_aes_0_status."]
pub mod se_aes_0_status;
#[doc = "se_aes_0_iv_0 (rw) register accessor: se_aes_0_iv_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_iv_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_iv_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_iv_0`] module"]
#[doc(alias = "se_aes_0_iv_0")]
pub type SeAes0Iv0 = crate::Reg<se_aes_0_iv_0::SeAes0Iv0Spec>;
#[doc = "se_aes_0_iv_0."]
pub mod se_aes_0_iv_0;
#[doc = "se_aes_0_iv_1 (rw) register accessor: se_aes_0_iv_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_iv_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_iv_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_iv_1`] module"]
#[doc(alias = "se_aes_0_iv_1")]
pub type SeAes0Iv1 = crate::Reg<se_aes_0_iv_1::SeAes0Iv1Spec>;
#[doc = "se_aes_0_iv_1."]
pub mod se_aes_0_iv_1;
#[doc = "se_aes_0_iv_2 (rw) register accessor: se_aes_0_iv_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_iv_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_iv_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_iv_2`] module"]
#[doc(alias = "se_aes_0_iv_2")]
pub type SeAes0Iv2 = crate::Reg<se_aes_0_iv_2::SeAes0Iv2Spec>;
#[doc = "se_aes_0_iv_2."]
pub mod se_aes_0_iv_2;
#[doc = "se_aes_0_iv_3 (rw) register accessor: se_aes_0_iv_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_iv_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_iv_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_iv_3`] module"]
#[doc(alias = "se_aes_0_iv_3")]
pub type SeAes0Iv3 = crate::Reg<se_aes_0_iv_3::SeAes0Iv3Spec>;
#[doc = "se_aes_0_iv_3."]
pub mod se_aes_0_iv_3;
#[doc = "se_aes_0_key_0 (rw) register accessor: se_aes_0_key_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_key_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_key_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_key_0`] module"]
#[doc(alias = "se_aes_0_key_0")]
pub type SeAes0Key0 = crate::Reg<se_aes_0_key_0::SeAes0Key0Spec>;
#[doc = "se_aes_0_key_0."]
pub mod se_aes_0_key_0;
#[doc = "se_aes_0_key_1 (rw) register accessor: se_aes_0_key_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_key_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_key_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_key_1`] module"]
#[doc(alias = "se_aes_0_key_1")]
pub type SeAes0Key1 = crate::Reg<se_aes_0_key_1::SeAes0Key1Spec>;
#[doc = "se_aes_0_key_1."]
pub mod se_aes_0_key_1;
#[doc = "se_aes_0_key_2 (rw) register accessor: se_aes_0_key_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_key_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_key_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_key_2`] module"]
#[doc(alias = "se_aes_0_key_2")]
pub type SeAes0Key2 = crate::Reg<se_aes_0_key_2::SeAes0Key2Spec>;
#[doc = "se_aes_0_key_2."]
pub mod se_aes_0_key_2;
#[doc = "se_aes_0_key_3 (rw) register accessor: se_aes_0_key_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_key_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_key_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_key_3`] module"]
#[doc(alias = "se_aes_0_key_3")]
pub type SeAes0Key3 = crate::Reg<se_aes_0_key_3::SeAes0Key3Spec>;
#[doc = "se_aes_0_key_3."]
pub mod se_aes_0_key_3;
#[doc = "se_aes_0_key_4 (rw) register accessor: se_aes_0_key_4.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_key_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_key_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_key_4`] module"]
#[doc(alias = "se_aes_0_key_4")]
pub type SeAes0Key4 = crate::Reg<se_aes_0_key_4::SeAes0Key4Spec>;
#[doc = "se_aes_0_key_4."]
pub mod se_aes_0_key_4;
#[doc = "se_aes_0_key_5 (rw) register accessor: se_aes_0_key_5.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_key_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_key_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_key_5`] module"]
#[doc(alias = "se_aes_0_key_5")]
pub type SeAes0Key5 = crate::Reg<se_aes_0_key_5::SeAes0Key5Spec>;
#[doc = "se_aes_0_key_5."]
pub mod se_aes_0_key_5;
#[doc = "se_aes_0_key_6 (rw) register accessor: se_aes_0_key_6.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_key_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_key_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_key_6`] module"]
#[doc(alias = "se_aes_0_key_6")]
pub type SeAes0Key6 = crate::Reg<se_aes_0_key_6::SeAes0Key6Spec>;
#[doc = "se_aes_0_key_6."]
pub mod se_aes_0_key_6;
#[doc = "se_aes_0_key_7 (rw) register accessor: se_aes_0_key_7.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_key_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_key_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_key_7`] module"]
#[doc(alias = "se_aes_0_key_7")]
pub type SeAes0Key7 = crate::Reg<se_aes_0_key_7::SeAes0Key7Spec>;
#[doc = "se_aes_0_key_7."]
pub mod se_aes_0_key_7;
#[doc = "se_aes_0_key_sel_0 (rw) register accessor: se_aes_0_key_sel_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_key_sel_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_key_sel_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_key_sel_0`] module"]
#[doc(alias = "se_aes_0_key_sel_0")]
pub type SeAes0KeySel0 = crate::Reg<se_aes_0_key_sel_0::SeAes0KeySel0Spec>;
#[doc = "se_aes_0_key_sel_0."]
pub mod se_aes_0_key_sel_0;
#[doc = "se_aes_0_key_sel_1 (rw) register accessor: se_aes_0_key_sel_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_key_sel_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_key_sel_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_key_sel_1`] module"]
#[doc(alias = "se_aes_0_key_sel_1")]
pub type SeAes0KeySel1 = crate::Reg<se_aes_0_key_sel_1::SeAes0KeySel1Spec>;
#[doc = "se_aes_0_key_sel_1."]
pub mod se_aes_0_key_sel_1;
#[doc = "se_aes_0_endian (rw) register accessor: se_aes_0_endian.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_endian::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_endian::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_endian`] module"]
#[doc(alias = "se_aes_0_endian")]
pub type SeAes0Endian = crate::Reg<se_aes_0_endian::SeAes0EndianSpec>;
#[doc = "se_aes_0_endian."]
pub mod se_aes_0_endian;
#[doc = "se_aes_0_sboot (rw) register accessor: se_aes_0_sboot.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_sboot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_sboot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_sboot`] module"]
#[doc(alias = "se_aes_0_sboot")]
pub type SeAes0Sboot = crate::Reg<se_aes_0_sboot::SeAes0SbootSpec>;
#[doc = "se_aes_0_sboot."]
pub mod se_aes_0_sboot;
#[doc = "se_aes_0_link (rw) register accessor: se_aes_0_link.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_link`] module"]
#[doc(alias = "se_aes_0_link")]
pub type SeAes0Link = crate::Reg<se_aes_0_link::SeAes0LinkSpec>;
#[doc = "se_aes_0_link."]
pub mod se_aes_0_link;
#[doc = "se_aes_0_ctrl_prot (rw) register accessor: se_aes_0_ctrl_prot.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_ctrl_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_ctrl_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_ctrl_prot`] module"]
#[doc(alias = "se_aes_0_ctrl_prot")]
pub type SeAes0CtrlProt = crate::Reg<se_aes_0_ctrl_prot::SeAes0CtrlProtSpec>;
#[doc = "se_aes_0_ctrl_prot."]
pub mod se_aes_0_ctrl_prot;
#[doc = "se_trng_0_ctrl_0 (rw) register accessor: se_trng_0_ctrl_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_ctrl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_trng_0_ctrl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_ctrl_0`] module"]
#[doc(alias = "se_trng_0_ctrl_0")]
pub type SeTrng0Ctrl0 = crate::Reg<se_trng_0_ctrl_0::SeTrng0Ctrl0Spec>;
#[doc = "se_trng_0_ctrl_0."]
pub mod se_trng_0_ctrl_0;
#[doc = "se_trng_0_status (r) register accessor: se_trng_0_status.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_status`] module"]
#[doc(alias = "se_trng_0_status")]
pub type SeTrng0Status = crate::Reg<se_trng_0_status::SeTrng0StatusSpec>;
#[doc = "se_trng_0_status."]
pub mod se_trng_0_status;
#[doc = "se_trng_0_dout_0 (r) register accessor: se_trng_0_dout_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_dout_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_dout_0`] module"]
#[doc(alias = "se_trng_0_dout_0")]
pub type SeTrng0Dout0 = crate::Reg<se_trng_0_dout_0::SeTrng0Dout0Spec>;
#[doc = "se_trng_0_dout_0."]
pub mod se_trng_0_dout_0;
#[doc = "se_trng_0_dout_1 (r) register accessor: se_trng_0_dout_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_dout_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_dout_1`] module"]
#[doc(alias = "se_trng_0_dout_1")]
pub type SeTrng0Dout1 = crate::Reg<se_trng_0_dout_1::SeTrng0Dout1Spec>;
#[doc = "se_trng_0_dout_1."]
pub mod se_trng_0_dout_1;
#[doc = "se_trng_0_dout_2 (r) register accessor: se_trng_0_dout_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_dout_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_dout_2`] module"]
#[doc(alias = "se_trng_0_dout_2")]
pub type SeTrng0Dout2 = crate::Reg<se_trng_0_dout_2::SeTrng0Dout2Spec>;
#[doc = "se_trng_0_dout_2."]
pub mod se_trng_0_dout_2;
#[doc = "se_trng_0_dout_3 (r) register accessor: se_trng_0_dout_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_dout_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_dout_3`] module"]
#[doc(alias = "se_trng_0_dout_3")]
pub type SeTrng0Dout3 = crate::Reg<se_trng_0_dout_3::SeTrng0Dout3Spec>;
#[doc = "se_trng_0_dout_3."]
pub mod se_trng_0_dout_3;
#[doc = "se_trng_0_dout_4 (r) register accessor: se_trng_0_dout_4.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_dout_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_dout_4`] module"]
#[doc(alias = "se_trng_0_dout_4")]
pub type SeTrng0Dout4 = crate::Reg<se_trng_0_dout_4::SeTrng0Dout4Spec>;
#[doc = "se_trng_0_dout_4."]
pub mod se_trng_0_dout_4;
#[doc = "se_trng_0_dout_5 (r) register accessor: se_trng_0_dout_5.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_dout_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_dout_5`] module"]
#[doc(alias = "se_trng_0_dout_5")]
pub type SeTrng0Dout5 = crate::Reg<se_trng_0_dout_5::SeTrng0Dout5Spec>;
#[doc = "se_trng_0_dout_5."]
pub mod se_trng_0_dout_5;
#[doc = "se_trng_0_dout_6 (r) register accessor: se_trng_0_dout_6.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_dout_6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_dout_6`] module"]
#[doc(alias = "se_trng_0_dout_6")]
pub type SeTrng0Dout6 = crate::Reg<se_trng_0_dout_6::SeTrng0Dout6Spec>;
#[doc = "se_trng_0_dout_6."]
pub mod se_trng_0_dout_6;
#[doc = "se_trng_0_dout_7 (r) register accessor: se_trng_0_dout_7.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_dout_7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_dout_7`] module"]
#[doc(alias = "se_trng_0_dout_7")]
pub type SeTrng0Dout7 = crate::Reg<se_trng_0_dout_7::SeTrng0Dout7Spec>;
#[doc = "se_trng_0_dout_7."]
pub mod se_trng_0_dout_7;
#[doc = "se_trng_0_test (rw) register accessor: se_trng_0_test.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_trng_0_test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_test`] module"]
#[doc(alias = "se_trng_0_test")]
pub type SeTrng0Test = crate::Reg<se_trng_0_test::SeTrng0TestSpec>;
#[doc = "se_trng_0_test."]
pub mod se_trng_0_test;
#[doc = "se_trng_0_ctrl_1 (rw) register accessor: se_trng_0_ctrl_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_ctrl_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_trng_0_ctrl_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_ctrl_1`] module"]
#[doc(alias = "se_trng_0_ctrl_1")]
pub type SeTrng0Ctrl1 = crate::Reg<se_trng_0_ctrl_1::SeTrng0Ctrl1Spec>;
#[doc = "se_trng_0_ctrl_1."]
pub mod se_trng_0_ctrl_1;
#[doc = "se_trng_0_ctrl_2 (rw) register accessor: se_trng_0_ctrl_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_ctrl_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_trng_0_ctrl_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_ctrl_2`] module"]
#[doc(alias = "se_trng_0_ctrl_2")]
pub type SeTrng0Ctrl2 = crate::Reg<se_trng_0_ctrl_2::SeTrng0Ctrl2Spec>;
#[doc = "se_trng_0_ctrl_2."]
pub mod se_trng_0_ctrl_2;
#[doc = "se_trng_0_ctrl_3 (rw) register accessor: se_trng_0_ctrl_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_ctrl_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_trng_0_ctrl_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_ctrl_3`] module"]
#[doc(alias = "se_trng_0_ctrl_3")]
pub type SeTrng0Ctrl3 = crate::Reg<se_trng_0_ctrl_3::SeTrng0Ctrl3Spec>;
#[doc = "se_trng_0_ctrl_3."]
pub mod se_trng_0_ctrl_3;
#[doc = "se_trng_0_test_out_0 (r) register accessor: se_trng_0_test_out_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_test_out_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_test_out_0`] module"]
#[doc(alias = "se_trng_0_test_out_0")]
pub type SeTrng0TestOut0 = crate::Reg<se_trng_0_test_out_0::SeTrng0TestOut0Spec>;
#[doc = "se_trng_0_test_out_0."]
pub mod se_trng_0_test_out_0;
#[doc = "se_trng_0_test_out_1 (r) register accessor: se_trng_0_test_out_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_test_out_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_test_out_1`] module"]
#[doc(alias = "se_trng_0_test_out_1")]
pub type SeTrng0TestOut1 = crate::Reg<se_trng_0_test_out_1::SeTrng0TestOut1Spec>;
#[doc = "se_trng_0_test_out_1."]
pub mod se_trng_0_test_out_1;
#[doc = "se_trng_0_test_out_2 (r) register accessor: se_trng_0_test_out_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_test_out_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_test_out_2`] module"]
#[doc(alias = "se_trng_0_test_out_2")]
pub type SeTrng0TestOut2 = crate::Reg<se_trng_0_test_out_2::SeTrng0TestOut2Spec>;
#[doc = "se_trng_0_test_out_2."]
pub mod se_trng_0_test_out_2;
#[doc = "se_trng_0_test_out_3 (r) register accessor: se_trng_0_test_out_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_test_out_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_test_out_3`] module"]
#[doc(alias = "se_trng_0_test_out_3")]
pub type SeTrng0TestOut3 = crate::Reg<se_trng_0_test_out_3::SeTrng0TestOut3Spec>;
#[doc = "se_trng_0_test_out_3."]
pub mod se_trng_0_test_out_3;
#[doc = "se_trng_0_ctrl_prot (rw) register accessor: se_trng_0_ctrl_prot.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_ctrl_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_trng_0_ctrl_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_ctrl_prot`] module"]
#[doc(alias = "se_trng_0_ctrl_prot")]
pub type SeTrng0CtrlProt = crate::Reg<se_trng_0_ctrl_prot::SeTrng0CtrlProtSpec>;
#[doc = "se_trng_0_ctrl_prot."]
pub mod se_trng_0_ctrl_prot;
#[doc = "se_pka_0_ctrl_0 (rw) register accessor: se_pka_0_ctrl_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_pka_0_ctrl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_pka_0_ctrl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_pka_0_ctrl_0`] module"]
#[doc(alias = "se_pka_0_ctrl_0")]
pub type SePka0Ctrl0 = crate::Reg<se_pka_0_ctrl_0::SePka0Ctrl0Spec>;
#[doc = "se_pka_0_ctrl_0."]
pub mod se_pka_0_ctrl_0;
#[doc = "se_pka_0_seed (rw) register accessor: se_pka_0_seed.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_pka_0_seed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_pka_0_seed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_pka_0_seed`] module"]
#[doc(alias = "se_pka_0_seed")]
pub type SePka0Seed = crate::Reg<se_pka_0_seed::SePka0SeedSpec>;
#[doc = "se_pka_0_seed."]
pub mod se_pka_0_seed;
#[doc = "se_pka_0_ctrl_1 (rw) register accessor: se_pka_0_ctrl_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_pka_0_ctrl_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_pka_0_ctrl_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_pka_0_ctrl_1`] module"]
#[doc(alias = "se_pka_0_ctrl_1")]
pub type SePka0Ctrl1 = crate::Reg<se_pka_0_ctrl_1::SePka0Ctrl1Spec>;
#[doc = "se_pka_0_ctrl_1."]
pub mod se_pka_0_ctrl_1;
#[doc = "se_pka_0_rw (rw) register accessor: se_pka_0_rw.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_pka_0_rw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_pka_0_rw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_pka_0_rw`] module"]
#[doc(alias = "se_pka_0_rw")]
pub type SePka0Rw = crate::Reg<se_pka_0_rw::SePka0RwSpec>;
#[doc = "se_pka_0_rw."]
pub mod se_pka_0_rw;
#[doc = "se_pka_0_rw_burst (rw) register accessor: se_pka_0_rw_burst.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_pka_0_rw_burst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_pka_0_rw_burst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_pka_0_rw_burst`] module"]
#[doc(alias = "se_pka_0_rw_burst")]
pub type SePka0RwBurst = crate::Reg<se_pka_0_rw_burst::SePka0RwBurstSpec>;
#[doc = "se_pka_0_rw_burst."]
pub mod se_pka_0_rw_burst;
#[doc = "se_pka_0_ctrl_prot (rw) register accessor: se_pka_0_ctrl_prot.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_pka_0_ctrl_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_pka_0_ctrl_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_pka_0_ctrl_prot`] module"]
#[doc(alias = "se_pka_0_ctrl_prot")]
pub type SePka0CtrlProt = crate::Reg<se_pka_0_ctrl_prot::SePka0CtrlProtSpec>;
#[doc = "se_pka_0_ctrl_prot."]
pub mod se_pka_0_ctrl_prot;
#[doc = "se_cdet_0_ctrl_0 (rw) register accessor: se_cdet_0_ctrl_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_cdet_0_ctrl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_cdet_0_ctrl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_cdet_0_ctrl_0`] module"]
#[doc(alias = "se_cdet_0_ctrl_0")]
pub type SeCdet0Ctrl0 = crate::Reg<se_cdet_0_ctrl_0::SeCdet0Ctrl0Spec>;
#[doc = "se_cdet_0_ctrl_0."]
pub mod se_cdet_0_ctrl_0;
#[doc = "se_cdet_0_ctrl_1 (rw) register accessor: se_cdet_0_ctrl_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_cdet_0_ctrl_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_cdet_0_ctrl_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_cdet_0_ctrl_1`] module"]
#[doc(alias = "se_cdet_0_ctrl_1")]
pub type SeCdet0Ctrl1 = crate::Reg<se_cdet_0_ctrl_1::SeCdet0Ctrl1Spec>;
#[doc = "se_cdet_0_ctrl_1."]
pub mod se_cdet_0_ctrl_1;
#[doc = "se_cdet_0_ctrl_prot (rw) register accessor: se_cdet_0_ctrl_prot.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_cdet_0_ctrl_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_cdet_0_ctrl_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_cdet_0_ctrl_prot`] module"]
#[doc(alias = "se_cdet_0_ctrl_prot")]
pub type SeCdet0CtrlProt = crate::Reg<se_cdet_0_ctrl_prot::SeCdet0CtrlProtSpec>;
#[doc = "se_cdet_0_ctrl_prot."]
pub mod se_cdet_0_ctrl_prot;
#[doc = "se_gmac_0_ctrl_0 (rw) register accessor: se_gmac_0_ctrl_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_gmac_0_ctrl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_gmac_0_ctrl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_gmac_0_ctrl_0`] module"]
#[doc(alias = "se_gmac_0_ctrl_0")]
pub type SeGmac0Ctrl0 = crate::Reg<se_gmac_0_ctrl_0::SeGmac0Ctrl0Spec>;
#[doc = "se_gmac_0_ctrl_0."]
pub mod se_gmac_0_ctrl_0;
#[doc = "se_gmac_0_lca (rw) register accessor: se_gmac_0_lca.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_gmac_0_lca::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_gmac_0_lca::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_gmac_0_lca`] module"]
#[doc(alias = "se_gmac_0_lca")]
pub type SeGmac0Lca = crate::Reg<se_gmac_0_lca::SeGmac0LcaSpec>;
#[doc = "se_gmac_0_lca."]
pub mod se_gmac_0_lca;
#[doc = "se_gmac_0_status (r) register accessor: se_gmac_0_status.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_gmac_0_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_gmac_0_status`] module"]
#[doc(alias = "se_gmac_0_status")]
pub type SeGmac0Status = crate::Reg<se_gmac_0_status::SeGmac0StatusSpec>;
#[doc = "se_gmac_0_status."]
pub mod se_gmac_0_status;
#[doc = "se_gmac_0_ctrl_prot (rw) register accessor: se_gmac_0_ctrl_prot.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_gmac_0_ctrl_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_gmac_0_ctrl_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_gmac_0_ctrl_prot`] module"]
#[doc(alias = "se_gmac_0_ctrl_prot")]
pub type SeGmac0CtrlProt = crate::Reg<se_gmac_0_ctrl_prot::SeGmac0CtrlProtSpec>;
#[doc = "se_gmac_0_ctrl_prot."]
pub mod se_gmac_0_ctrl_prot;
#[doc = "se_ctrl_prot_rd (r) register accessor: se_ctrl_prot_rd.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_ctrl_prot_rd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_ctrl_prot_rd`] module"]
#[doc(alias = "se_ctrl_prot_rd")]
pub type SeCtrlProtRd = crate::Reg<se_ctrl_prot_rd::SeCtrlProtRdSpec>;
#[doc = "se_ctrl_prot_rd."]
pub mod se_ctrl_prot_rd;
#[doc = "se_ctrl_reserved_0 (rw) register accessor: se_ctrl_reserved_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_ctrl_reserved_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_ctrl_reserved_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_ctrl_reserved_0`] module"]
#[doc(alias = "se_ctrl_reserved_0")]
pub type SeCtrlReserved0 = crate::Reg<se_ctrl_reserved_0::SeCtrlReserved0Spec>;
#[doc = "se_ctrl_reserved_0."]
pub mod se_ctrl_reserved_0;
#[doc = "se_ctrl_reserved_1 (rw) register accessor: se_ctrl_reserved_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_ctrl_reserved_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_ctrl_reserved_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_ctrl_reserved_1`] module"]
#[doc(alias = "se_ctrl_reserved_1")]
pub type SeCtrlReserved1 = crate::Reg<se_ctrl_reserved_1::SeCtrlReserved1Spec>;
#[doc = "se_ctrl_reserved_1."]
pub mod se_ctrl_reserved_1;
#[doc = "se_ctrl_reserved_2 (rw) register accessor: se_ctrl_reserved_2.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_ctrl_reserved_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_ctrl_reserved_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_ctrl_reserved_2`] module"]
#[doc(alias = "se_ctrl_reserved_2")]
pub type SeCtrlReserved2 = crate::Reg<se_ctrl_reserved_2::SeCtrlReserved2Spec>;
#[doc = "se_ctrl_reserved_2."]
pub mod se_ctrl_reserved_2;
