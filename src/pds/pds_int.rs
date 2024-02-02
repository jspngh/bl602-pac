#[doc = "Register `PDS_INT` reader"]
pub type R = crate::R<PdsIntSpec>;
#[doc = "Register `PDS_INT` writer"]
pub type W = crate::W<PdsIntSpec>;
#[doc = "Field `ro_pds_wake_int` reader - "]
pub type RoPdsWakeIntR = crate::BitReader;
#[doc = "Field `ro_pds_irq_in` reader - "]
pub type RoPdsIrqInR = crate::BitReader;
#[doc = "Field `ro_pds_rf_done_int` reader - "]
pub type RoPdsRfDoneIntR = crate::BitReader;
#[doc = "Field `ro_pds_pll_done_int` reader - "]
pub type RoPdsPllDoneIntR = crate::BitReader;
#[doc = "Field `cr_pds_wake_int_mask` reader - "]
pub type CrPdsWakeIntMaskR = crate::BitReader;
#[doc = "Field `cr_pds_wake_int_mask` writer - "]
pub type CrPdsWakeIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_irq_in_dis` reader - "]
pub type CrPdsIrqInDisR = crate::BitReader;
#[doc = "Field `cr_pds_irq_in_dis` writer - "]
pub type CrPdsIrqInDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_rf_done_int_mask` reader - "]
pub type CrPdsRfDoneIntMaskR = crate::BitReader;
#[doc = "Field `cr_pds_rf_done_int_mask` writer - "]
pub type CrPdsRfDoneIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_pll_done_int_mask` reader - "]
pub type CrPdsPllDoneIntMaskR = crate::BitReader;
#[doc = "Field `cr_pds_pll_done_int_mask` writer - "]
pub type CrPdsPllDoneIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_int_clr` reader - "]
pub type CrPdsIntClrR = crate::BitReader;
#[doc = "Field `cr_pds_int_clr` writer - "]
pub type CrPdsIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ro_pds_wake_int(&self) -> RoPdsWakeIntR {
        RoPdsWakeIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ro_pds_irq_in(&self) -> RoPdsIrqInR {
        RoPdsIrqInR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ro_pds_rf_done_int(&self) -> RoPdsRfDoneIntR {
        RoPdsRfDoneIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ro_pds_pll_done_int(&self) -> RoPdsPllDoneIntR {
        RoPdsPllDoneIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_wake_int_mask(&self) -> CrPdsWakeIntMaskR {
        CrPdsWakeIntMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_irq_in_dis(&self) -> CrPdsIrqInDisR {
        CrPdsIrqInDisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_rf_done_int_mask(&self) -> CrPdsRfDoneIntMaskR {
        CrPdsRfDoneIntMaskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_pds_pll_done_int_mask(&self) -> CrPdsPllDoneIntMaskR {
        CrPdsPllDoneIntMaskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_int_clr(&self) -> CrPdsIntClrR {
        CrPdsIntClrR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_wake_int_mask(&mut self) -> CrPdsWakeIntMaskW<'_, PdsIntSpec> {
        CrPdsWakeIntMaskW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_irq_in_dis(&mut self) -> CrPdsIrqInDisW<'_, PdsIntSpec> {
        CrPdsIrqInDisW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_rf_done_int_mask(&mut self) -> CrPdsRfDoneIntMaskW<'_, PdsIntSpec> {
        CrPdsRfDoneIntMaskW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_pds_pll_done_int_mask(&mut self) -> CrPdsPllDoneIntMaskW<'_, PdsIntSpec> {
        CrPdsPllDoneIntMaskW::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_int_clr(&mut self) -> CrPdsIntClrW<'_, PdsIntSpec> {
        CrPdsIntClrW::new(self, 16)
    }
}
#[doc = "PDS_INT.\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdsIntSpec;
impl crate::RegisterSpec for PdsIntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_int::R`](R) reader structure"]
impl crate::Readable for PdsIntSpec {}
#[doc = "`write(|w| ..)` method takes [`pds_int::W`](W) writer structure"]
impl crate::Writable for PdsIntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDS_INT to value 0"]
impl crate::Resettable for PdsIntSpec {}
