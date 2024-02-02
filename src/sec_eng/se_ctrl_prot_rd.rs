#[doc = "Register `se_ctrl_prot_rd` reader"]
pub type R = crate::R<SeCtrlProtRdSpec>;
#[doc = "Field `se_sha_prot_en_rd` reader - "]
pub type SeShaProtEnRdR = crate::BitReader;
#[doc = "Field `se_sha_id0_en_rd` reader - "]
pub type SeShaId0EnRdR = crate::BitReader;
#[doc = "Field `se_sha_id1_en_rd` reader - "]
pub type SeShaId1EnRdR = crate::BitReader;
#[doc = "Field `se_aes_prot_en_rd` reader - "]
pub type SeAesProtEnRdR = crate::BitReader;
#[doc = "Field `se_aes_id0_en_rd` reader - "]
pub type SeAesId0EnRdR = crate::BitReader;
#[doc = "Field `se_aes_id1_en_rd` reader - "]
pub type SeAesId1EnRdR = crate::BitReader;
#[doc = "Field `se_trng_prot_en_rd` reader - "]
pub type SeTrngProtEnRdR = crate::BitReader;
#[doc = "Field `se_trng_id0_en_rd` reader - "]
pub type SeTrngId0EnRdR = crate::BitReader;
#[doc = "Field `se_trng_id1_en_rd` reader - "]
pub type SeTrngId1EnRdR = crate::BitReader;
#[doc = "Field `se_pka_prot_en_rd` reader - "]
pub type SePkaProtEnRdR = crate::BitReader;
#[doc = "Field `se_pka_id0_en_rd` reader - "]
pub type SePkaId0EnRdR = crate::BitReader;
#[doc = "Field `se_pka_id1_en_rd` reader - "]
pub type SePkaId1EnRdR = crate::BitReader;
#[doc = "Field `se_cdet_prot_en_rd` reader - "]
pub type SeCdetProtEnRdR = crate::BitReader;
#[doc = "Field `se_cdet_id0_en_rd` reader - "]
pub type SeCdetId0EnRdR = crate::BitReader;
#[doc = "Field `se_cdet_id1_en_rd` reader - "]
pub type SeCdetId1EnRdR = crate::BitReader;
#[doc = "Field `se_gmac_prot_en_rd` reader - "]
pub type SeGmacProtEnRdR = crate::BitReader;
#[doc = "Field `se_gmac_id0_en_rd` reader - "]
pub type SeGmacId0EnRdR = crate::BitReader;
#[doc = "Field `se_gmac_id1_en_rd` reader - "]
pub type SeGmacId1EnRdR = crate::BitReader;
#[doc = "Field `se_dbg_dis` reader - "]
pub type SeDbgDisR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_sha_prot_en_rd(&self) -> SeShaProtEnRdR {
        SeShaProtEnRdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_sha_id0_en_rd(&self) -> SeShaId0EnRdR {
        SeShaId0EnRdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_sha_id1_en_rd(&self) -> SeShaId1EnRdR {
        SeShaId1EnRdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn se_aes_prot_en_rd(&self) -> SeAesProtEnRdR {
        SeAesProtEnRdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn se_aes_id0_en_rd(&self) -> SeAesId0EnRdR {
        SeAesId0EnRdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn se_aes_id1_en_rd(&self) -> SeAesId1EnRdR {
        SeAesId1EnRdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_trng_prot_en_rd(&self) -> SeTrngProtEnRdR {
        SeTrngProtEnRdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_trng_id0_en_rd(&self) -> SeTrngId0EnRdR {
        SeTrngId0EnRdR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_trng_id1_en_rd(&self) -> SeTrngId1EnRdR {
        SeTrngId1EnRdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn se_pka_prot_en_rd(&self) -> SePkaProtEnRdR {
        SePkaProtEnRdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_pka_id0_en_rd(&self) -> SePkaId0EnRdR {
        SePkaId0EnRdR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn se_pka_id1_en_rd(&self) -> SePkaId1EnRdR {
        SePkaId1EnRdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn se_cdet_prot_en_rd(&self) -> SeCdetProtEnRdR {
        SeCdetProtEnRdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn se_cdet_id0_en_rd(&self) -> SeCdetId0EnRdR {
        SeCdetId0EnRdR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn se_cdet_id1_en_rd(&self) -> SeCdetId1EnRdR {
        SeCdetId1EnRdR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn se_gmac_prot_en_rd(&self) -> SeGmacProtEnRdR {
        SeGmacProtEnRdR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn se_gmac_id0_en_rd(&self) -> SeGmacId0EnRdR {
        SeGmacId0EnRdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn se_gmac_id1_en_rd(&self) -> SeGmacId1EnRdR {
        SeGmacId1EnRdR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn se_dbg_dis(&self) -> SeDbgDisR {
        SeDbgDisR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "se_ctrl_prot_rd.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_ctrl_prot_rd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeCtrlProtRdSpec;
impl crate::RegisterSpec for SeCtrlProtRdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_ctrl_prot_rd::R`](R) reader structure"]
impl crate::Readable for SeCtrlProtRdSpec {}
#[doc = "`reset()` method sets se_ctrl_prot_rd to value 0x0077_7777"]
impl crate::Resettable for SeCtrlProtRdSpec {
    const RESET_VALUE: u32 = 0x0077_7777;
}
