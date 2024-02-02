#[doc = "Register `se_trng_0_ctrl_3` reader"]
pub type R = crate::R<SeTrng0Ctrl3Spec>;
#[doc = "Register `se_trng_0_ctrl_3` writer"]
pub type W = crate::W<SeTrng0Ctrl3Spec>;
#[doc = "Field `se_trng_0_cp_ratio` reader - "]
pub type SeTrng0CpRatioR = crate::FieldReader;
#[doc = "Field `se_trng_0_cp_ratio` writer - "]
pub type SeTrng0CpRatioW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `se_trng_0_ht_rct_c` reader - "]
pub type SeTrng0HtRctCR = crate::FieldReader;
#[doc = "Field `se_trng_0_ht_rct_c` writer - "]
pub type SeTrng0HtRctCW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `se_trng_0_ht_apt_c` reader - "]
pub type SeTrng0HtAptCR = crate::FieldReader<u16>;
#[doc = "Field `se_trng_0_ht_apt_c` writer - "]
pub type SeTrng0HtAptCW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `se_trng_0_ht_od_en` reader - "]
pub type SeTrng0HtOdEnR = crate::BitReader;
#[doc = "Field `se_trng_0_ht_od_en` writer - "]
pub type SeTrng0HtOdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_trng_0_rosc_dis` reader - Used to be called 'se_trng_0_rosc_en', but the SDK calls it 'se_trng_0_rosc_dis'."]
pub type SeTrng0RoscDisR = crate::BitReader;
#[doc = "Field `se_trng_0_rosc_dis` writer - Used to be called 'se_trng_0_rosc_en', but the SDK calls it 'se_trng_0_rosc_dis'."]
pub type SeTrng0RoscDisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn se_trng_0_cp_ratio(&self) -> SeTrng0CpRatioR {
        SeTrng0CpRatioR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn se_trng_0_ht_rct_c(&self) -> SeTrng0HtRctCR {
        SeTrng0HtRctCR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn se_trng_0_ht_apt_c(&self) -> SeTrng0HtAptCR {
        SeTrng0HtAptCR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn se_trng_0_ht_od_en(&self) -> SeTrng0HtOdEnR {
        SeTrng0HtOdEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 31 - Used to be called 'se_trng_0_rosc_en', but the SDK calls it 'se_trng_0_rosc_dis'."]
    #[inline(always)]
    pub fn se_trng_0_rosc_dis(&self) -> SeTrng0RoscDisR {
        SeTrng0RoscDisR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn se_trng_0_cp_ratio(&mut self) -> SeTrng0CpRatioW<'_, SeTrng0Ctrl3Spec> {
        SeTrng0CpRatioW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn se_trng_0_ht_rct_c(&mut self) -> SeTrng0HtRctCW<'_, SeTrng0Ctrl3Spec> {
        SeTrng0HtRctCW::new(self, 8)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn se_trng_0_ht_apt_c(&mut self) -> SeTrng0HtAptCW<'_, SeTrng0Ctrl3Spec> {
        SeTrng0HtAptCW::new(self, 16)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn se_trng_0_ht_od_en(&mut self) -> SeTrng0HtOdEnW<'_, SeTrng0Ctrl3Spec> {
        SeTrng0HtOdEnW::new(self, 26)
    }
    #[doc = "Bit 31 - Used to be called 'se_trng_0_rosc_en', but the SDK calls it 'se_trng_0_rosc_dis'."]
    #[inline(always)]
    pub fn se_trng_0_rosc_dis(&mut self) -> SeTrng0RoscDisW<'_, SeTrng0Ctrl3Spec> {
        SeTrng0RoscDisW::new(self, 31)
    }
}
#[doc = "se_trng_0_ctrl_3.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_ctrl_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_trng_0_ctrl_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeTrng0Ctrl3Spec;
impl crate::RegisterSpec for SeTrng0Ctrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_trng_0_ctrl_3::R`](R) reader structure"]
impl crate::Readable for SeTrng0Ctrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`se_trng_0_ctrl_3::W`](W) writer structure"]
impl crate::Writable for SeTrng0Ctrl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets se_trng_0_ctrl_3 to value 0x837a_4203"]
impl crate::Resettable for SeTrng0Ctrl3Spec {
    const RESET_VALUE: u32 = 0x837a_4203;
}
