#[doc = "Register `pucr1` reader"]
pub type R = crate::R<Pucr1Spec>;
#[doc = "Register `pucr1` writer"]
pub type W = crate::W<Pucr1Spec>;
#[doc = "Field `pu_sfreg` reader - "]
pub type PuSfregR = crate::BitReader;
#[doc = "Field `pu_sfreg` writer - "]
pub type PuSfregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_lna` reader - "]
pub type PuLnaR = crate::BitReader;
#[doc = "Field `pu_lna` writer - "]
pub type PuLnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_rmxgm` reader - "]
pub type PuRmxgmR = crate::BitReader;
#[doc = "Field `pu_rmxgm` writer - "]
pub type PuRmxgmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_rmx` reader - "]
pub type PuRmxR = crate::BitReader;
#[doc = "Field `pu_rmx` writer - "]
pub type PuRmxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_rbb` reader - "]
pub type PuRbbR = crate::BitReader;
#[doc = "Field `pu_rbb` writer - "]
pub type PuRbbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_adda_ldo` reader - "]
pub type PuAddaLdoR = crate::BitReader;
#[doc = "Field `pu_adda_ldo` writer - "]
pub type PuAddaLdoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_clk_en` reader - "]
pub type AdcClkEnR = crate::BitReader;
#[doc = "Field `adc_clk_en` writer - "]
pub type AdcClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_adc` reader - "]
pub type PuAdcR = crate::BitReader;
#[doc = "Field `pu_adc` writer - "]
pub type PuAdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_op_atest` reader - "]
pub type PuOpAtestR = crate::BitReader;
#[doc = "Field `pu_op_atest` writer - "]
pub type PuOpAtestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_pa` reader - "]
pub type PuPaR = crate::BitReader;
#[doc = "Field `pu_pa` writer - "]
pub type PuPaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_tmx` reader - "]
pub type PuTmxR = crate::BitReader;
#[doc = "Field `pu_tmx` writer - "]
pub type PuTmxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_tbb` reader - "]
pub type PuTbbR = crate::BitReader;
#[doc = "Field `pu_tbb` writer - "]
pub type PuTbbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_dac` reader - "]
pub type PuDacR = crate::BitReader;
#[doc = "Field `pu_dac` writer - "]
pub type PuDacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_vco` reader - "]
pub type PuVcoR = crate::BitReader;
#[doc = "Field `pu_vco` writer - "]
pub type PuVcoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_fbdv` reader - "]
pub type PuFbdvR = crate::BitReader;
#[doc = "Field `pu_fbdv` writer - "]
pub type PuFbdvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_pfd` reader - "]
pub type PuPfdR = crate::BitReader;
#[doc = "Field `pu_pfd` writer - "]
pub type PuPfdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_osmx` reader - "]
pub type PuOsmxR = crate::BitReader;
#[doc = "Field `pu_osmx` writer - "]
pub type PuOsmxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_rxbuf` reader - "]
pub type PuRxbufR = crate::BitReader;
#[doc = "Field `pu_rxbuf` writer - "]
pub type PuRxbufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_txbuf` reader - "]
pub type PuTxbufR = crate::BitReader;
#[doc = "Field `pu_txbuf` writer - "]
pub type PuTxbufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `trsw_en` reader - "]
pub type TrswEnR = crate::BitReader;
#[doc = "Field `trsw_en` writer - "]
pub type TrswEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_pkdet` reader - "]
pub type PuPkdetR = crate::BitReader;
#[doc = "Field `pu_pkdet` writer - "]
pub type PuPkdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_rosdac` reader - "]
pub type PuRosdacR = crate::BitReader;
#[doc = "Field `pu_rosdac` writer - "]
pub type PuRosdacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_pwrmx` reader - "]
pub type PuPwrmxR = crate::BitReader;
#[doc = "Field `pu_pwrmx` writer - "]
pub type PuPwrmxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_tosdac` reader - "]
pub type PuTosdacR = crate::BitReader;
#[doc = "Field `pu_tosdac` writer - "]
pub type PuTosdacW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_sfreg(&self) -> PuSfregR {
        PuSfregR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_lna(&self) -> PuLnaR {
        PuLnaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_rmxgm(&self) -> PuRmxgmR {
        PuRmxgmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_rmx(&self) -> PuRmxR {
        PuRmxR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_rbb(&self) -> PuRbbR {
        PuRbbR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_adda_ldo(&self) -> PuAddaLdoR {
        PuAddaLdoR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adc_clk_en(&self) -> AdcClkEnR {
        AdcClkEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pu_adc(&self) -> PuAdcR {
        PuAdcR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pu_op_atest(&self) -> PuOpAtestR {
        PuOpAtestR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pu_pa(&self) -> PuPaR {
        PuPaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pu_tmx(&self) -> PuTmxR {
        PuTmxR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_tbb(&self) -> PuTbbR {
        PuTbbR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_dac(&self) -> PuDacR {
        PuDacR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pu_vco(&self) -> PuVcoR {
        PuVcoR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pu_fbdv(&self) -> PuFbdvR {
        PuFbdvR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pu_pfd(&self) -> PuPfdR {
        PuPfdR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pu_osmx(&self) -> PuOsmxR {
        PuOsmxR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pu_rxbuf(&self) -> PuRxbufR {
        PuRxbufR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pu_txbuf(&self) -> PuTxbufR {
        PuTxbufR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn trsw_en(&self) -> TrswEnR {
        TrswEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pu_pkdet(&self) -> PuPkdetR {
        PuPkdetR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pu_rosdac(&self) -> PuRosdacR {
        PuRosdacR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pu_pwrmx(&self) -> PuPwrmxR {
        PuPwrmxR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pu_tosdac(&self) -> PuTosdacR {
        PuTosdacR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_sfreg(&mut self) -> PuSfregW<'_, Pucr1Spec> {
        PuSfregW::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_lna(&mut self) -> PuLnaW<'_, Pucr1Spec> {
        PuLnaW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_rmxgm(&mut self) -> PuRmxgmW<'_, Pucr1Spec> {
        PuRmxgmW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_rmx(&mut self) -> PuRmxW<'_, Pucr1Spec> {
        PuRmxW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_rbb(&mut self) -> PuRbbW<'_, Pucr1Spec> {
        PuRbbW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_adda_ldo(&mut self) -> PuAddaLdoW<'_, Pucr1Spec> {
        PuAddaLdoW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adc_clk_en(&mut self) -> AdcClkEnW<'_, Pucr1Spec> {
        AdcClkEnW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pu_adc(&mut self) -> PuAdcW<'_, Pucr1Spec> {
        PuAdcW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pu_op_atest(&mut self) -> PuOpAtestW<'_, Pucr1Spec> {
        PuOpAtestW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pu_pa(&mut self) -> PuPaW<'_, Pucr1Spec> {
        PuPaW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pu_tmx(&mut self) -> PuTmxW<'_, Pucr1Spec> {
        PuTmxW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_tbb(&mut self) -> PuTbbW<'_, Pucr1Spec> {
        PuTbbW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_dac(&mut self) -> PuDacW<'_, Pucr1Spec> {
        PuDacW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pu_vco(&mut self) -> PuVcoW<'_, Pucr1Spec> {
        PuVcoW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pu_fbdv(&mut self) -> PuFbdvW<'_, Pucr1Spec> {
        PuFbdvW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pu_pfd(&mut self) -> PuPfdW<'_, Pucr1Spec> {
        PuPfdW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pu_osmx(&mut self) -> PuOsmxW<'_, Pucr1Spec> {
        PuOsmxW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pu_rxbuf(&mut self) -> PuRxbufW<'_, Pucr1Spec> {
        PuRxbufW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pu_txbuf(&mut self) -> PuTxbufW<'_, Pucr1Spec> {
        PuTxbufW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn trsw_en(&mut self) -> TrswEnW<'_, Pucr1Spec> {
        TrswEnW::new(self, 26)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pu_pkdet(&mut self) -> PuPkdetW<'_, Pucr1Spec> {
        PuPkdetW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pu_rosdac(&mut self) -> PuRosdacW<'_, Pucr1Spec> {
        PuRosdacW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pu_pwrmx(&mut self) -> PuPwrmxW<'_, Pucr1Spec> {
        PuPwrmxW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pu_tosdac(&mut self) -> PuTosdacW<'_, Pucr1Spec> {
        PuTosdacW::new(self, 31)
    }
}
#[doc = "pucr1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pucr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pucr1Spec;
impl crate::RegisterSpec for Pucr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pucr1::R`](R) reader structure"]
impl crate::Readable for Pucr1Spec {}
#[doc = "`write(|w| ..)` method takes [`pucr1::W`](W) writer structure"]
impl crate::Writable for Pucr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets pucr1 to value 0"]
impl crate::Resettable for Pucr1Spec {}
