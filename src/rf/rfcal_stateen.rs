#[doc = "Register `rfcal_stateen` reader"]
pub type R = crate::R<RfcalStateenSpec>;
#[doc = "Register `rfcal_stateen` writer"]
pub type W = crate::W<RfcalStateenSpec>;
#[doc = "Field `rcal_sten_resv` reader - "]
pub type RcalStenResvR = crate::BitReader;
#[doc = "Field `rcal_sten_resv` writer - "]
pub type RcalStenResvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_oscal_sten` reader - "]
pub type AdcOscalStenR = crate::BitReader;
#[doc = "Field `adc_oscal_sten` writer - "]
pub type AdcOscalStenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dl_rfcal_table_sten` reader - "]
pub type DlRfcalTableStenR = crate::BitReader;
#[doc = "Field `dl_rfcal_table_sten` writer - "]
pub type DlRfcalTableStenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fcal_sten` reader - "]
pub type FcalStenR = crate::BitReader;
#[doc = "Field `fcal_sten` writer - "]
pub type FcalStenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `acal_sten` reader - "]
pub type AcalStenR = crate::BitReader;
#[doc = "Field `acal_sten` writer - "]
pub type AcalStenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `inc_fcal_sten` reader - "]
pub type IncFcalStenR = crate::BitReader;
#[doc = "Field `inc_fcal_sten` writer - "]
pub type IncFcalStenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `inc_acal_sten` reader - "]
pub type IncAcalStenR = crate::BitReader;
#[doc = "Field `inc_acal_sten` writer - "]
pub type IncAcalStenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpll_cal_sten` reader - "]
pub type ClkpllCalStenR = crate::BitReader;
#[doc = "Field `clkpll_cal_sten` writer - "]
pub type ClkpllCalStenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `roscal_sten` reader - "]
pub type RoscalStenR = crate::BitReader;
#[doc = "Field `roscal_sten` writer - "]
pub type RoscalStenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `toscal_sten_resv` reader - "]
pub type ToscalStenResvR = crate::BitReader;
#[doc = "Field `toscal_sten_resv` writer - "]
pub type ToscalStenResvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rccal_sten` reader - "]
pub type RccalStenR = crate::BitReader;
#[doc = "Field `rccal_sten` writer - "]
pub type RccalStenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_leakcal_sten` reader - "]
pub type LoLeakcalStenR = crate::BitReader;
#[doc = "Field `lo_leakcal_sten` writer - "]
pub type LoLeakcalStenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tiqcal_sten` reader - "]
pub type TiqcalStenR = crate::BitReader;
#[doc = "Field `tiqcal_sten` writer - "]
pub type TiqcalStenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `riqcal_sten` reader - "]
pub type RiqcalStenR = crate::BitReader;
#[doc = "Field `riqcal_sten` writer - "]
pub type RiqcalStenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwdet_cal_sten` reader - "]
pub type PwdetCalStenR = crate::BitReader;
#[doc = "Field `pwdet_cal_sten` writer - "]
pub type PwdetCalStenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tsencal_sten` reader - "]
pub type TsencalStenR = crate::BitReader;
#[doc = "Field `tsencal_sten` writer - "]
pub type TsencalStenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dpd_sten` reader - "]
pub type DpdStenR = crate::BitReader;
#[doc = "Field `dpd_sten` writer - "]
pub type DpdStenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rfcal_level` reader - "]
pub type RfcalLevelR = crate::FieldReader;
#[doc = "Field `rfcal_level` writer - "]
pub type RfcalLevelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rcal_sten_resv(&self) -> RcalStenResvR {
        RcalStenResvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc_oscal_sten(&self) -> AdcOscalStenR {
        AdcOscalStenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dl_rfcal_table_sten(&self) -> DlRfcalTableStenR {
        DlRfcalTableStenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fcal_sten(&self) -> FcalStenR {
        FcalStenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn acal_sten(&self) -> AcalStenR {
        AcalStenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn inc_fcal_sten(&self) -> IncFcalStenR {
        IncFcalStenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn inc_acal_sten(&self) -> IncAcalStenR {
        IncAcalStenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clkpll_cal_sten(&self) -> ClkpllCalStenR {
        ClkpllCalStenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn roscal_sten(&self) -> RoscalStenR {
        RoscalStenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn toscal_sten_resv(&self) -> ToscalStenResvR {
        ToscalStenResvR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rccal_sten(&self) -> RccalStenR {
        RccalStenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn lo_leakcal_sten(&self) -> LoLeakcalStenR {
        LoLeakcalStenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tiqcal_sten(&self) -> TiqcalStenR {
        TiqcalStenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn riqcal_sten(&self) -> RiqcalStenR {
        RiqcalStenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pwdet_cal_sten(&self) -> PwdetCalStenR {
        PwdetCalStenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tsencal_sten(&self) -> TsencalStenR {
        TsencalStenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dpd_sten(&self) -> DpdStenR {
        DpdStenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn rfcal_level(&self) -> RfcalLevelR {
        RfcalLevelR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rcal_sten_resv(&mut self) -> RcalStenResvW<'_, RfcalStateenSpec> {
        RcalStenResvW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc_oscal_sten(&mut self) -> AdcOscalStenW<'_, RfcalStateenSpec> {
        AdcOscalStenW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dl_rfcal_table_sten(&mut self) -> DlRfcalTableStenW<'_, RfcalStateenSpec> {
        DlRfcalTableStenW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fcal_sten(&mut self) -> FcalStenW<'_, RfcalStateenSpec> {
        FcalStenW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn acal_sten(&mut self) -> AcalStenW<'_, RfcalStateenSpec> {
        AcalStenW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn inc_fcal_sten(&mut self) -> IncFcalStenW<'_, RfcalStateenSpec> {
        IncFcalStenW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn inc_acal_sten(&mut self) -> IncAcalStenW<'_, RfcalStateenSpec> {
        IncAcalStenW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clkpll_cal_sten(&mut self) -> ClkpllCalStenW<'_, RfcalStateenSpec> {
        ClkpllCalStenW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn roscal_sten(&mut self) -> RoscalStenW<'_, RfcalStateenSpec> {
        RoscalStenW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn toscal_sten_resv(&mut self) -> ToscalStenResvW<'_, RfcalStateenSpec> {
        ToscalStenResvW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rccal_sten(&mut self) -> RccalStenW<'_, RfcalStateenSpec> {
        RccalStenW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn lo_leakcal_sten(&mut self) -> LoLeakcalStenW<'_, RfcalStateenSpec> {
        LoLeakcalStenW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tiqcal_sten(&mut self) -> TiqcalStenW<'_, RfcalStateenSpec> {
        TiqcalStenW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn riqcal_sten(&mut self) -> RiqcalStenW<'_, RfcalStateenSpec> {
        RiqcalStenW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pwdet_cal_sten(&mut self) -> PwdetCalStenW<'_, RfcalStateenSpec> {
        PwdetCalStenW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tsencal_sten(&mut self) -> TsencalStenW<'_, RfcalStateenSpec> {
        TsencalStenW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dpd_sten(&mut self) -> DpdStenW<'_, RfcalStateenSpec> {
        DpdStenW::new(self, 16)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn rfcal_level(&mut self) -> RfcalLevelW<'_, RfcalStateenSpec> {
        RfcalLevelW::new(self, 30)
    }
}
#[doc = "rf calibration state enabl in full cal list\n\nYou can [`read`](crate::Reg::read) this register and get [`rfcal_stateen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfcal_stateen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfcalStateenSpec;
impl crate::RegisterSpec for RfcalStateenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfcal_stateen::R`](R) reader structure"]
impl crate::Readable for RfcalStateenSpec {}
#[doc = "`write(|w| ..)` method takes [`rfcal_stateen::W`](W) writer structure"]
impl crate::Writable for RfcalStateenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rfcal_stateen to value 0"]
impl crate::Resettable for RfcalStateenSpec {}
