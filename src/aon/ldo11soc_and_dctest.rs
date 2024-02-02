#[doc = "Register `ldo11soc_and_dctest` reader"]
pub type R = crate::R<Ldo11socAndDctestSpec>;
#[doc = "Register `ldo11soc_and_dctest` writer"]
pub type W = crate::W<Ldo11socAndDctestSpec>;
#[doc = "Field `pu_ldo11soc_aon` reader - "]
pub type PuLdo11socAonR = crate::BitReader;
#[doc = "Field `pu_ldo11soc_aon` writer - "]
pub type PuLdo11socAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo11soc_sstart_sel_aon` reader - "]
pub type Ldo11socSstartSelAonR = crate::BitReader;
#[doc = "Field `ldo11soc_sstart_sel_aon` writer - "]
pub type Ldo11socSstartSelAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo11soc_sstart_delay_aon` reader - "]
pub type Ldo11socSstartDelayAonR = crate::FieldReader;
#[doc = "Field `ldo11soc_sstart_delay_aon` writer - "]
pub type Ldo11socSstartDelayAonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ldo11soc_pulldown_aon` reader - "]
pub type Ldo11socPulldownAonR = crate::BitReader;
#[doc = "Field `ldo11soc_pulldown_aon` writer - "]
pub type Ldo11socPulldownAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo11soc_pulldown_sel_aon` reader - "]
pub type Ldo11socPulldownSelAonR = crate::BitReader;
#[doc = "Field `ldo11soc_pulldown_sel_aon` writer - "]
pub type Ldo11socPulldownSelAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo11soc_vth_sel_aon` reader - "]
pub type Ldo11socVthSelAonR = crate::FieldReader;
#[doc = "Field `ldo11soc_vth_sel_aon` writer - "]
pub type Ldo11socVthSelAonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ldo11soc_cc_aon` reader - "]
pub type Ldo11socCcAonR = crate::FieldReader;
#[doc = "Field `ldo11soc_cc_aon` writer - "]
pub type Ldo11socCcAonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ldo11soc_rdy_aon` reader - "]
pub type Ldo11socRdyAonR = crate::BitReader;
#[doc = "Field `ldo11soc_power_good_aon` reader - "]
pub type Ldo11socPowerGoodAonR = crate::BitReader;
#[doc = "Field `pu_vddcore_misc_aon` reader - "]
pub type PuVddcoreMiscAonR = crate::BitReader;
#[doc = "Field `pu_vddcore_misc_aon` writer - "]
pub type PuVddcoreMiscAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pmip_dc_tp_out_en_aon` reader - "]
pub type PmipDcTpOutEnAonR = crate::BitReader;
#[doc = "Field `pmip_dc_tp_out_en_aon` writer - "]
pub type PmipDcTpOutEnAonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ldo11soc_aon(&self) -> PuLdo11socAonR {
        PuLdo11socAonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ldo11soc_sstart_sel_aon(&self) -> Ldo11socSstartSelAonR {
        Ldo11socSstartSelAonR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ldo11soc_sstart_delay_aon(&self) -> Ldo11socSstartDelayAonR {
        Ldo11socSstartDelayAonR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ldo11soc_pulldown_aon(&self) -> Ldo11socPulldownAonR {
        Ldo11socPulldownAonR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ldo11soc_pulldown_sel_aon(&self) -> Ldo11socPulldownSelAonR {
        Ldo11socPulldownSelAonR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ldo11soc_vth_sel_aon(&self) -> Ldo11socVthSelAonR {
        Ldo11socVthSelAonR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ldo11soc_cc_aon(&self) -> Ldo11socCcAonR {
        Ldo11socCcAonR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ldo11soc_rdy_aon(&self) -> Ldo11socRdyAonR {
        Ldo11socRdyAonR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn ldo11soc_power_good_aon(&self) -> Ldo11socPowerGoodAonR {
        Ldo11socPowerGoodAonR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pu_vddcore_misc_aon(&self) -> PuVddcoreMiscAonR {
        PuVddcoreMiscAonR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pmip_dc_tp_out_en_aon(&self) -> PmipDcTpOutEnAonR {
        PmipDcTpOutEnAonR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ldo11soc_aon(&mut self) -> PuLdo11socAonW<'_, Ldo11socAndDctestSpec> {
        PuLdo11socAonW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ldo11soc_sstart_sel_aon(&mut self) -> Ldo11socSstartSelAonW<'_, Ldo11socAndDctestSpec> {
        Ldo11socSstartSelAonW::new(self, 4)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ldo11soc_sstart_delay_aon(
        &mut self,
    ) -> Ldo11socSstartDelayAonW<'_, Ldo11socAndDctestSpec> {
        Ldo11socSstartDelayAonW::new(self, 8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ldo11soc_pulldown_aon(&mut self) -> Ldo11socPulldownAonW<'_, Ldo11socAndDctestSpec> {
        Ldo11socPulldownAonW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ldo11soc_pulldown_sel_aon(
        &mut self,
    ) -> Ldo11socPulldownSelAonW<'_, Ldo11socAndDctestSpec> {
        Ldo11socPulldownSelAonW::new(self, 11)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ldo11soc_vth_sel_aon(&mut self) -> Ldo11socVthSelAonW<'_, Ldo11socAndDctestSpec> {
        Ldo11socVthSelAonW::new(self, 12)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ldo11soc_cc_aon(&mut self) -> Ldo11socCcAonW<'_, Ldo11socAndDctestSpec> {
        Ldo11socCcAonW::new(self, 24)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pu_vddcore_misc_aon(&mut self) -> PuVddcoreMiscAonW<'_, Ldo11socAndDctestSpec> {
        PuVddcoreMiscAonW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pmip_dc_tp_out_en_aon(&mut self) -> PmipDcTpOutEnAonW<'_, Ldo11socAndDctestSpec> {
        PmipDcTpOutEnAonW::new(self, 31)
    }
}
#[doc = "ldo11soc_and_dctest.\n\nYou can [`read`](crate::Reg::read) this register and get [`ldo11soc_and_dctest::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo11soc_and_dctest::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ldo11socAndDctestSpec;
impl crate::RegisterSpec for Ldo11socAndDctestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldo11soc_and_dctest::R`](R) reader structure"]
impl crate::Readable for Ldo11socAndDctestSpec {}
#[doc = "`write(|w| ..)` method takes [`ldo11soc_and_dctest::W`](W) writer structure"]
impl crate::Writable for Ldo11socAndDctestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ldo11soc_and_dctest to value 0x7000_1811"]
impl crate::Resettable for Ldo11socAndDctestSpec {
    const RESET_VALUE: u32 = 0x7000_1811;
}
