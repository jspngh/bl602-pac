#[doc = "Register `rfcal_status` reader"]
pub type R = crate::R<RfcalStatusSpec>;
#[doc = "Register `rfcal_status` writer"]
pub type W = crate::W<RfcalStatusSpec>;
#[doc = "Field `rcal_status` reader - "]
pub type RcalStatusR = crate::FieldReader;
#[doc = "Field `rcal_status` writer - "]
pub type RcalStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc_oscal_status` reader - "]
pub type AdcOscalStatusR = crate::FieldReader;
#[doc = "Field `adc_oscal_status` writer - "]
pub type AdcOscalStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `fcal_status` reader - "]
pub type FcalStatusR = crate::FieldReader;
#[doc = "Field `fcal_status` writer - "]
pub type FcalStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `acal_status` reader - "]
pub type AcalStatusR = crate::FieldReader;
#[doc = "Field `acal_status` writer - "]
pub type AcalStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `inc_fcal_status` reader - "]
pub type IncFcalStatusR = crate::FieldReader;
#[doc = "Field `inc_fcal_status` writer - "]
pub type IncFcalStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `inc_acal_status` reader - "]
pub type IncAcalStatusR = crate::FieldReader;
#[doc = "Field `inc_acal_status` writer - "]
pub type IncAcalStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `clkpll_cal_status` reader - "]
pub type ClkpllCalStatusR = crate::FieldReader;
#[doc = "Field `clkpll_cal_status` writer - "]
pub type ClkpllCalStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ros_status` reader - "]
pub type RosStatusR = crate::FieldReader;
#[doc = "Field `ros_status` writer - "]
pub type RosStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tos_status` reader - "]
pub type TosStatusR = crate::FieldReader;
#[doc = "Field `tos_status` writer - "]
pub type TosStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rccal_status` reader - "]
pub type RccalStatusR = crate::FieldReader;
#[doc = "Field `rccal_status` writer - "]
pub type RccalStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lo_leakcal_status` reader - "]
pub type LoLeakcalStatusR = crate::FieldReader;
#[doc = "Field `lo_leakcal_status` writer - "]
pub type LoLeakcalStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tiqcal_status_resv` reader - "]
pub type TiqcalStatusResvR = crate::FieldReader;
#[doc = "Field `tiqcal_status_resv` writer - "]
pub type TiqcalStatusResvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `riqcal_status_resv` reader - "]
pub type RiqcalStatusResvR = crate::FieldReader;
#[doc = "Field `riqcal_status_resv` writer - "]
pub type RiqcalStatusResvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pwdet_cal_status` reader - "]
pub type PwdetCalStatusR = crate::FieldReader;
#[doc = "Field `pwdet_cal_status` writer - "]
pub type PwdetCalStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tenscal_status` reader - "]
pub type TenscalStatusR = crate::FieldReader;
#[doc = "Field `tenscal_status` writer - "]
pub type TenscalStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dpd_status` reader - "]
pub type DpdStatusR = crate::FieldReader;
#[doc = "Field `dpd_status` writer - "]
pub type DpdStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rcal_status(&self) -> RcalStatusR {
        RcalStatusR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn adc_oscal_status(&self) -> AdcOscalStatusR {
        AdcOscalStatusR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn fcal_status(&self) -> FcalStatusR {
        FcalStatusR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn acal_status(&self) -> AcalStatusR {
        AcalStatusR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn inc_fcal_status(&self) -> IncFcalStatusR {
        IncFcalStatusR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn inc_acal_status(&self) -> IncAcalStatusR {
        IncAcalStatusR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn clkpll_cal_status(&self) -> ClkpllCalStatusR {
        ClkpllCalStatusR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn ros_status(&self) -> RosStatusR {
        RosStatusR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn tos_status(&self) -> TosStatusR {
        TosStatusR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn rccal_status(&self) -> RccalStatusR {
        RccalStatusR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_leakcal_status(&self) -> LoLeakcalStatusR {
        LoLeakcalStatusR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn tiqcal_status_resv(&self) -> TiqcalStatusResvR {
        TiqcalStatusResvR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn riqcal_status_resv(&self) -> RiqcalStatusResvR {
        RiqcalStatusResvR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn pwdet_cal_status(&self) -> PwdetCalStatusR {
        PwdetCalStatusR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn tenscal_status(&self) -> TenscalStatusR {
        TenscalStatusR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn dpd_status(&self) -> DpdStatusR {
        DpdStatusR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rcal_status(&mut self) -> RcalStatusW<'_, RfcalStatusSpec> {
        RcalStatusW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn adc_oscal_status(&mut self) -> AdcOscalStatusW<'_, RfcalStatusSpec> {
        AdcOscalStatusW::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn fcal_status(&mut self) -> FcalStatusW<'_, RfcalStatusSpec> {
        FcalStatusW::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn acal_status(&mut self) -> AcalStatusW<'_, RfcalStatusSpec> {
        AcalStatusW::new(self, 6)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn inc_fcal_status(&mut self) -> IncFcalStatusW<'_, RfcalStatusSpec> {
        IncFcalStatusW::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn inc_acal_status(&mut self) -> IncAcalStatusW<'_, RfcalStatusSpec> {
        IncAcalStatusW::new(self, 10)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn clkpll_cal_status(&mut self) -> ClkpllCalStatusW<'_, RfcalStatusSpec> {
        ClkpllCalStatusW::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn ros_status(&mut self) -> RosStatusW<'_, RfcalStatusSpec> {
        RosStatusW::new(self, 14)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn tos_status(&mut self) -> TosStatusW<'_, RfcalStatusSpec> {
        TosStatusW::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn rccal_status(&mut self) -> RccalStatusW<'_, RfcalStatusSpec> {
        RccalStatusW::new(self, 18)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_leakcal_status(&mut self) -> LoLeakcalStatusW<'_, RfcalStatusSpec> {
        LoLeakcalStatusW::new(self, 20)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn tiqcal_status_resv(&mut self) -> TiqcalStatusResvW<'_, RfcalStatusSpec> {
        TiqcalStatusResvW::new(self, 22)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn riqcal_status_resv(&mut self) -> RiqcalStatusResvW<'_, RfcalStatusSpec> {
        RiqcalStatusResvW::new(self, 24)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn pwdet_cal_status(&mut self) -> PwdetCalStatusW<'_, RfcalStatusSpec> {
        PwdetCalStatusW::new(self, 26)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn tenscal_status(&mut self) -> TenscalStatusW<'_, RfcalStatusSpec> {
        TenscalStatusW::new(self, 28)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn dpd_status(&mut self) -> DpdStatusW<'_, RfcalStatusSpec> {
        DpdStatusW::new(self, 30)
    }
}
#[doc = "rfcal_status.\n\nYou can [`read`](crate::Reg::read) this register and get [`rfcal_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfcal_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfcalStatusSpec;
impl crate::RegisterSpec for RfcalStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfcal_status::R`](R) reader structure"]
impl crate::Readable for RfcalStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`rfcal_status::W`](W) writer structure"]
impl crate::Writable for RfcalStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rfcal_status to value 0"]
impl crate::Resettable for RfcalStatusSpec {}
