#[doc = "Register `ten_dc` reader"]
pub type R = crate::R<TenDcSpec>;
#[doc = "Register `ten_dc` writer"]
pub type W = crate::W<TenDcSpec>;
#[doc = "Field `tmux` reader - "]
pub type TmuxR = crate::FieldReader;
#[doc = "Field `tmux` writer - "]
pub type TmuxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dc_tp_en` reader - "]
pub type DcTpEnR = crate::BitReader;
#[doc = "Field `dc_tp_en` writer - "]
pub type DcTpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dc_tp_clkpll_en` reader - "]
pub type DcTpClkpllEnR = crate::BitReader;
#[doc = "Field `dc_tp_clkpll_en` writer - "]
pub type DcTpClkpllEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_clkpll` reader - "]
pub type TenClkpllR = crate::BitReader;
#[doc = "Field `ten_clkpll` writer - "]
pub type TenClkpllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_clkpll_sfreg` reader - "]
pub type TenClkpllSfregR = crate::BitReader;
#[doc = "Field `ten_clkpll_sfreg` writer - "]
pub type TenClkpllSfregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_rrf_0` reader - "]
pub type TenRrf0R = crate::BitReader;
#[doc = "Field `ten_rrf_0` writer - "]
pub type TenRrf0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_rrf_1` reader - "]
pub type TenRrf1R = crate::BitReader;
#[doc = "Field `ten_rrf_1` writer - "]
pub type TenRrf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_pa` reader - "]
pub type TenPaR = crate::BitReader;
#[doc = "Field `ten_pa` writer - "]
pub type TenPaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_tmx` reader - "]
pub type TenTmxR = crate::BitReader;
#[doc = "Field `ten_tmx` writer - "]
pub type TenTmxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_tia` reader - "]
pub type TenTiaR = crate::BitReader;
#[doc = "Field `ten_tia` writer - "]
pub type TenTiaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_bq` reader - "]
pub type TenBqR = crate::BitReader;
#[doc = "Field `ten_bq` writer - "]
pub type TenBqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_atest` reader - "]
pub type TenAtestR = crate::BitReader;
#[doc = "Field `ten_atest` writer - "]
pub type TenAtestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_tbb` reader - "]
pub type TenTbbR = crate::BitReader;
#[doc = "Field `ten_tbb` writer - "]
pub type TenTbbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_adc` reader - "]
pub type TenAdcR = crate::BitReader;
#[doc = "Field `ten_adc` writer - "]
pub type TenAdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_dac_i` reader - "]
pub type TenDacIR = crate::BitReader;
#[doc = "Field `ten_dac_i` writer - "]
pub type TenDacIW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_dac_q` reader - "]
pub type TenDacQR = crate::BitReader;
#[doc = "Field `ten_dac_q` writer - "]
pub type TenDacQW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_vco` reader - "]
pub type TenVcoR = crate::BitReader;
#[doc = "Field `ten_vco` writer - "]
pub type TenVcoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_pfdcp` reader - "]
pub type TenPfdcpR = crate::BitReader;
#[doc = "Field `ten_pfdcp` writer - "]
pub type TenPfdcpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_lf` reader - "]
pub type TenLfR = crate::BitReader;
#[doc = "Field `ten_lf` writer - "]
pub type TenLfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_lodist` reader - "]
pub type TenLodistR = crate::BitReader;
#[doc = "Field `ten_lodist` writer - "]
pub type TenLodistW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmux(&self) -> TmuxR {
        TmuxR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dc_tp_en(&self) -> DcTpEnR {
        DcTpEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dc_tp_clkpll_en(&self) -> DcTpClkpllEnR {
        DcTpClkpllEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ten_clkpll(&self) -> TenClkpllR {
        TenClkpllR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ten_clkpll_sfreg(&self) -> TenClkpllSfregR {
        TenClkpllSfregR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ten_rrf_0(&self) -> TenRrf0R {
        TenRrf0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ten_rrf_1(&self) -> TenRrf1R {
        TenRrf1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ten_pa(&self) -> TenPaR {
        TenPaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ten_tmx(&self) -> TenTmxR {
        TenTmxR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ten_tia(&self) -> TenTiaR {
        TenTiaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ten_bq(&self) -> TenBqR {
        TenBqR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ten_atest(&self) -> TenAtestR {
        TenAtestR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ten_tbb(&self) -> TenTbbR {
        TenTbbR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ten_adc(&self) -> TenAdcR {
        TenAdcR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ten_dac_i(&self) -> TenDacIR {
        TenDacIR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ten_dac_q(&self) -> TenDacQR {
        TenDacQR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ten_vco(&self) -> TenVcoR {
        TenVcoR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ten_pfdcp(&self) -> TenPfdcpR {
        TenPfdcpR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ten_lf(&self) -> TenLfR {
        TenLfR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ten_lodist(&self) -> TenLodistR {
        TenLodistR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmux(&mut self) -> TmuxW<'_, TenDcSpec> {
        TmuxW::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dc_tp_en(&mut self) -> DcTpEnW<'_, TenDcSpec> {
        DcTpEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dc_tp_clkpll_en(&mut self) -> DcTpClkpllEnW<'_, TenDcSpec> {
        DcTpClkpllEnW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ten_clkpll(&mut self) -> TenClkpllW<'_, TenDcSpec> {
        TenClkpllW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ten_clkpll_sfreg(&mut self) -> TenClkpllSfregW<'_, TenDcSpec> {
        TenClkpllSfregW::new(self, 9)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ten_rrf_0(&mut self) -> TenRrf0W<'_, TenDcSpec> {
        TenRrf0W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ten_rrf_1(&mut self) -> TenRrf1W<'_, TenDcSpec> {
        TenRrf1W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ten_pa(&mut self) -> TenPaW<'_, TenDcSpec> {
        TenPaW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ten_tmx(&mut self) -> TenTmxW<'_, TenDcSpec> {
        TenTmxW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ten_tia(&mut self) -> TenTiaW<'_, TenDcSpec> {
        TenTiaW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ten_bq(&mut self) -> TenBqW<'_, TenDcSpec> {
        TenBqW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ten_atest(&mut self) -> TenAtestW<'_, TenDcSpec> {
        TenAtestW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ten_tbb(&mut self) -> TenTbbW<'_, TenDcSpec> {
        TenTbbW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ten_adc(&mut self) -> TenAdcW<'_, TenDcSpec> {
        TenAdcW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ten_dac_i(&mut self) -> TenDacIW<'_, TenDcSpec> {
        TenDacIW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ten_dac_q(&mut self) -> TenDacQW<'_, TenDcSpec> {
        TenDacQW::new(self, 22)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ten_vco(&mut self) -> TenVcoW<'_, TenDcSpec> {
        TenVcoW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ten_pfdcp(&mut self) -> TenPfdcpW<'_, TenDcSpec> {
        TenPfdcpW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ten_lf(&mut self) -> TenLfW<'_, TenDcSpec> {
        TenLfW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ten_lodist(&mut self) -> TenLodistW<'_, TenDcSpec> {
        TenLodistW::new(self, 27)
    }
}
#[doc = "dc test register\n\nYou can [`read`](crate::Reg::read) this register and get [`ten_dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ten_dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TenDcSpec;
impl crate::RegisterSpec for TenDcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ten_dc::R`](R) reader structure"]
impl crate::Readable for TenDcSpec {}
#[doc = "`write(|w| ..)` method takes [`ten_dc::W`](W) writer structure"]
impl crate::Writable for TenDcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ten_dc to value 0"]
impl crate::Resettable for TenDcSpec {}
