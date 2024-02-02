#[doc = "Register `rfcal_ctrlen` reader"]
pub type R = crate::R<RfcalCtrlenSpec>;
#[doc = "Register `rfcal_ctrlen` writer"]
pub type W = crate::W<RfcalCtrlenSpec>;
#[doc = "Field `rcal_en_resv` reader - "]
pub type RcalEnResvR = crate::BitReader;
#[doc = "Field `rcal_en_resv` writer - "]
pub type RcalEnResvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_oscal_en` reader - "]
pub type AdcOscalEnR = crate::BitReader;
#[doc = "Field `adc_oscal_en` writer - "]
pub type AdcOscalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dl_rfcal_table_en` reader - "]
pub type DlRfcalTableEnR = crate::BitReader;
#[doc = "Field `dl_rfcal_table_en` writer - "]
pub type DlRfcalTableEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fcal_en` reader - "]
pub type FcalEnR = crate::BitReader;
#[doc = "Field `fcal_en` writer - "]
pub type FcalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `acal_en` reader - "]
pub type AcalEnR = crate::BitReader;
#[doc = "Field `acal_en` writer - "]
pub type AcalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fcal_inc_en` reader - "]
pub type FcalIncEnR = crate::BitReader;
#[doc = "Field `fcal_inc_en` writer - "]
pub type FcalIncEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `acal_inc_en` reader - "]
pub type AcalIncEnR = crate::BitReader;
#[doc = "Field `acal_inc_en` writer - "]
pub type AcalIncEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `roscal_inc_en` reader - "]
pub type RoscalIncEnR = crate::BitReader;
#[doc = "Field `roscal_inc_en` writer - "]
pub type RoscalIncEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_cal_en` reader - "]
pub type ClkpllCalEnR = crate::BitReader;
#[doc = "Field `clkpll_cal_en` writer - "]
pub type ClkpllCalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `roscal_en` reader - "]
pub type RoscalEnR = crate::BitReader;
#[doc = "Field `roscal_en` writer - "]
pub type RoscalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `toscal_en` reader - "]
pub type ToscalEnR = crate::BitReader;
#[doc = "Field `toscal_en` writer - "]
pub type ToscalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rccal_en` reader - "]
pub type RccalEnR = crate::BitReader;
#[doc = "Field `rccal_en` writer - "]
pub type RccalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_leakcal_en` reader - "]
pub type LoLeakcalEnR = crate::BitReader;
#[doc = "Field `lo_leakcal_en` writer - "]
pub type LoLeakcalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tiqcal_en` reader - "]
pub type TiqcalEnR = crate::BitReader;
#[doc = "Field `tiqcal_en` writer - "]
pub type TiqcalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `riqcal_en` reader - "]
pub type RiqcalEnR = crate::BitReader;
#[doc = "Field `riqcal_en` writer - "]
pub type RiqcalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwdet_cal_en` reader - "]
pub type PwdetCalEnR = crate::BitReader;
#[doc = "Field `pwdet_cal_en` writer - "]
pub type PwdetCalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tsencal_en` reader - "]
pub type TsencalEnR = crate::BitReader;
#[doc = "Field `tsencal_en` writer - "]
pub type TsencalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dpd_en` reader - "]
pub type DpdEnR = crate::BitReader;
#[doc = "Field `dpd_en` writer - "]
pub type DpdEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rcal_en_resv(&self) -> RcalEnResvR {
        RcalEnResvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc_oscal_en(&self) -> AdcOscalEnR {
        AdcOscalEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dl_rfcal_table_en(&self) -> DlRfcalTableEnR {
        DlRfcalTableEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fcal_en(&self) -> FcalEnR {
        FcalEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn acal_en(&self) -> AcalEnR {
        AcalEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fcal_inc_en(&self) -> FcalIncEnR {
        FcalIncEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn acal_inc_en(&self) -> AcalIncEnR {
        AcalIncEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn roscal_inc_en(&self) -> RoscalIncEnR {
        RoscalIncEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_cal_en(&self) -> ClkpllCalEnR {
        ClkpllCalEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn roscal_en(&self) -> RoscalEnR {
        RoscalEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn toscal_en(&self) -> ToscalEnR {
        ToscalEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rccal_en(&self) -> RccalEnR {
        RccalEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_leakcal_en(&self) -> LoLeakcalEnR {
        LoLeakcalEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tiqcal_en(&self) -> TiqcalEnR {
        TiqcalEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn riqcal_en(&self) -> RiqcalEnR {
        RiqcalEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pwdet_cal_en(&self) -> PwdetCalEnR {
        PwdetCalEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tsencal_en(&self) -> TsencalEnR {
        TsencalEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dpd_en(&self) -> DpdEnR {
        DpdEnR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rcal_en_resv(&mut self) -> RcalEnResvW<'_, RfcalCtrlenSpec> {
        RcalEnResvW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc_oscal_en(&mut self) -> AdcOscalEnW<'_, RfcalCtrlenSpec> {
        AdcOscalEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dl_rfcal_table_en(&mut self) -> DlRfcalTableEnW<'_, RfcalCtrlenSpec> {
        DlRfcalTableEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fcal_en(&mut self) -> FcalEnW<'_, RfcalCtrlenSpec> {
        FcalEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn acal_en(&mut self) -> AcalEnW<'_, RfcalCtrlenSpec> {
        AcalEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fcal_inc_en(&mut self) -> FcalIncEnW<'_, RfcalCtrlenSpec> {
        FcalIncEnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn acal_inc_en(&mut self) -> AcalIncEnW<'_, RfcalCtrlenSpec> {
        AcalIncEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn roscal_inc_en(&mut self) -> RoscalIncEnW<'_, RfcalCtrlenSpec> {
        RoscalIncEnW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_cal_en(&mut self) -> ClkpllCalEnW<'_, RfcalCtrlenSpec> {
        ClkpllCalEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn roscal_en(&mut self) -> RoscalEnW<'_, RfcalCtrlenSpec> {
        RoscalEnW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn toscal_en(&mut self) -> ToscalEnW<'_, RfcalCtrlenSpec> {
        ToscalEnW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rccal_en(&mut self) -> RccalEnW<'_, RfcalCtrlenSpec> {
        RccalEnW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_leakcal_en(&mut self) -> LoLeakcalEnW<'_, RfcalCtrlenSpec> {
        LoLeakcalEnW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tiqcal_en(&mut self) -> TiqcalEnW<'_, RfcalCtrlenSpec> {
        TiqcalEnW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn riqcal_en(&mut self) -> RiqcalEnW<'_, RfcalCtrlenSpec> {
        RiqcalEnW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pwdet_cal_en(&mut self) -> PwdetCalEnW<'_, RfcalCtrlenSpec> {
        PwdetCalEnW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tsencal_en(&mut self) -> TsencalEnW<'_, RfcalCtrlenSpec> {
        TsencalEnW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dpd_en(&mut self) -> DpdEnW<'_, RfcalCtrlenSpec> {
        DpdEnW::new(self, 17)
    }
}
#[doc = "Calibration mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfcal_ctrlen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfcal_ctrlen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfcalCtrlenSpec;
impl crate::RegisterSpec for RfcalCtrlenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfcal_ctrlen::R`](R) reader structure"]
impl crate::Readable for RfcalCtrlenSpec {}
#[doc = "`write(|w| ..)` method takes [`rfcal_ctrlen::W`](W) writer structure"]
impl crate::Writable for RfcalCtrlenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rfcal_ctrlen to value 0"]
impl crate::Resettable for RfcalCtrlenSpec {}
