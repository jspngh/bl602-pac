#[doc = "Register `spi_int_sts` reader"]
pub type R = crate::R<SpiIntStsSpec>;
#[doc = "Register `spi_int_sts` writer"]
pub type W = crate::W<SpiIntStsSpec>;
#[doc = "Field `spi_end_int` reader - "]
pub type SpiEndIntR = crate::BitReader;
#[doc = "Field `spi_txf_int` reader - "]
pub type SpiTxfIntR = crate::BitReader;
#[doc = "Field `spi_rxf_int` reader - "]
pub type SpiRxfIntR = crate::BitReader;
#[doc = "Field `spi_sto_int` reader - "]
pub type SpiStoIntR = crate::BitReader;
#[doc = "Field `spi_txu_int` reader - "]
pub type SpiTxuIntR = crate::BitReader;
#[doc = "Field `spi_fer_int` reader - "]
pub type SpiFerIntR = crate::BitReader;
#[doc = "Field `cr_spi_end_mask` reader - "]
pub type CrSpiEndMaskR = crate::BitReader;
#[doc = "Field `cr_spi_end_mask` writer - "]
pub type CrSpiEndMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_txf_mask` reader - "]
pub type CrSpiTxfMaskR = crate::BitReader;
#[doc = "Field `cr_spi_txf_mask` writer - "]
pub type CrSpiTxfMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_rxf_mask` reader - "]
pub type CrSpiRxfMaskR = crate::BitReader;
#[doc = "Field `cr_spi_rxf_mask` writer - "]
pub type CrSpiRxfMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_sto_mask` reader - "]
pub type CrSpiStoMaskR = crate::BitReader;
#[doc = "Field `cr_spi_sto_mask` writer - "]
pub type CrSpiStoMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_txu_mask` reader - "]
pub type CrSpiTxuMaskR = crate::BitReader;
#[doc = "Field `cr_spi_txu_mask` writer - "]
pub type CrSpiTxuMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_fer_mask` reader - "]
pub type CrSpiFerMaskR = crate::BitReader;
#[doc = "Field `cr_spi_fer_mask` writer - "]
pub type CrSpiFerMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_end_clr` reader - "]
pub type CrSpiEndClrR = crate::BitReader;
#[doc = "Field `cr_spi_end_clr` writer - "]
pub type CrSpiEndClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rsvd_17` reader - "]
pub type Rsvd17R = crate::BitReader;
#[doc = "Field `rsvd_17` writer - "]
pub type Rsvd17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rsvd_18` reader - "]
pub type Rsvd18R = crate::BitReader;
#[doc = "Field `rsvd_18` writer - "]
pub type Rsvd18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_sto_clr` reader - "]
pub type CrSpiStoClrR = crate::BitReader;
#[doc = "Field `cr_spi_sto_clr` writer - "]
pub type CrSpiStoClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_txu_clr` reader - "]
pub type CrSpiTxuClrR = crate::BitReader;
#[doc = "Field `cr_spi_txu_clr` writer - "]
pub type CrSpiTxuClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rsvd_21` reader - "]
pub type Rsvd21R = crate::BitReader;
#[doc = "Field `rsvd_21` writer - "]
pub type Rsvd21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_end_en` reader - "]
pub type CrSpiEndEnR = crate::BitReader;
#[doc = "Field `cr_spi_end_en` writer - "]
pub type CrSpiEndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_txf_en` reader - "]
pub type CrSpiTxfEnR = crate::BitReader;
#[doc = "Field `cr_spi_txf_en` writer - "]
pub type CrSpiTxfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_rxf_en` reader - "]
pub type CrSpiRxfEnR = crate::BitReader;
#[doc = "Field `cr_spi_rxf_en` writer - "]
pub type CrSpiRxfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_sto_en` reader - "]
pub type CrSpiStoEnR = crate::BitReader;
#[doc = "Field `cr_spi_sto_en` writer - "]
pub type CrSpiStoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_txu_en` reader - "]
pub type CrSpiTxuEnR = crate::BitReader;
#[doc = "Field `cr_spi_txu_en` writer - "]
pub type CrSpiTxuEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_spi_fer_en` reader - "]
pub type CrSpiFerEnR = crate::BitReader;
#[doc = "Field `cr_spi_fer_en` writer - "]
pub type CrSpiFerEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_end_int(&self) -> SpiEndIntR {
        SpiEndIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_txf_int(&self) -> SpiTxfIntR {
        SpiTxfIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_rxf_int(&self) -> SpiRxfIntR {
        SpiRxfIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_sto_int(&self) -> SpiStoIntR {
        SpiStoIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi_txu_int(&self) -> SpiTxuIntR {
        SpiTxuIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi_fer_int(&self) -> SpiFerIntR {
        SpiFerIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_spi_end_mask(&self) -> CrSpiEndMaskR {
        CrSpiEndMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_spi_txf_mask(&self) -> CrSpiTxfMaskR {
        CrSpiTxfMaskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_spi_rxf_mask(&self) -> CrSpiRxfMaskR {
        CrSpiRxfMaskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_spi_sto_mask(&self) -> CrSpiStoMaskR {
        CrSpiStoMaskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_spi_txu_mask(&self) -> CrSpiTxuMaskR {
        CrSpiTxuMaskR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_spi_fer_mask(&self) -> CrSpiFerMaskR {
        CrSpiFerMaskR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_spi_end_clr(&self) -> CrSpiEndClrR {
        CrSpiEndClrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rsvd_17(&self) -> Rsvd17R {
        Rsvd17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rsvd_18(&self) -> Rsvd18R {
        Rsvd18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_spi_sto_clr(&self) -> CrSpiStoClrR {
        CrSpiStoClrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cr_spi_txu_clr(&self) -> CrSpiTxuClrR {
        CrSpiTxuClrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rsvd_21(&self) -> Rsvd21R {
        Rsvd21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_spi_end_en(&self) -> CrSpiEndEnR {
        CrSpiEndEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_spi_txf_en(&self) -> CrSpiTxfEnR {
        CrSpiTxfEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_spi_rxf_en(&self) -> CrSpiRxfEnR {
        CrSpiRxfEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_spi_sto_en(&self) -> CrSpiStoEnR {
        CrSpiStoEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cr_spi_txu_en(&self) -> CrSpiTxuEnR {
        CrSpiTxuEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cr_spi_fer_en(&self) -> CrSpiFerEnR {
        CrSpiFerEnR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_spi_end_mask(&mut self) -> CrSpiEndMaskW<'_, SpiIntStsSpec> {
        CrSpiEndMaskW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_spi_txf_mask(&mut self) -> CrSpiTxfMaskW<'_, SpiIntStsSpec> {
        CrSpiTxfMaskW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_spi_rxf_mask(&mut self) -> CrSpiRxfMaskW<'_, SpiIntStsSpec> {
        CrSpiRxfMaskW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_spi_sto_mask(&mut self) -> CrSpiStoMaskW<'_, SpiIntStsSpec> {
        CrSpiStoMaskW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_spi_txu_mask(&mut self) -> CrSpiTxuMaskW<'_, SpiIntStsSpec> {
        CrSpiTxuMaskW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_spi_fer_mask(&mut self) -> CrSpiFerMaskW<'_, SpiIntStsSpec> {
        CrSpiFerMaskW::new(self, 13)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_spi_end_clr(&mut self) -> CrSpiEndClrW<'_, SpiIntStsSpec> {
        CrSpiEndClrW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rsvd_17(&mut self) -> Rsvd17W<'_, SpiIntStsSpec> {
        Rsvd17W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rsvd_18(&mut self) -> Rsvd18W<'_, SpiIntStsSpec> {
        Rsvd18W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_spi_sto_clr(&mut self) -> CrSpiStoClrW<'_, SpiIntStsSpec> {
        CrSpiStoClrW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cr_spi_txu_clr(&mut self) -> CrSpiTxuClrW<'_, SpiIntStsSpec> {
        CrSpiTxuClrW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rsvd_21(&mut self) -> Rsvd21W<'_, SpiIntStsSpec> {
        Rsvd21W::new(self, 21)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_spi_end_en(&mut self) -> CrSpiEndEnW<'_, SpiIntStsSpec> {
        CrSpiEndEnW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_spi_txf_en(&mut self) -> CrSpiTxfEnW<'_, SpiIntStsSpec> {
        CrSpiTxfEnW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_spi_rxf_en(&mut self) -> CrSpiRxfEnW<'_, SpiIntStsSpec> {
        CrSpiRxfEnW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_spi_sto_en(&mut self) -> CrSpiStoEnW<'_, SpiIntStsSpec> {
        CrSpiStoEnW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cr_spi_txu_en(&mut self) -> CrSpiTxuEnW<'_, SpiIntStsSpec> {
        CrSpiTxuEnW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cr_spi_fer_en(&mut self) -> CrSpiFerEnW<'_, SpiIntStsSpec> {
        CrSpiFerEnW::new(self, 29)
    }
}
#[doc = "spi_int_sts.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_int_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_int_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiIntStsSpec;
impl crate::RegisterSpec for SpiIntStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_int_sts::R`](R) reader structure"]
impl crate::Readable for SpiIntStsSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_int_sts::W`](W) writer structure"]
impl crate::Writable for SpiIntStsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets spi_int_sts to value 0x3f00_3f00"]
impl crate::Resettable for SpiIntStsSpec {
    const RESET_VALUE: u32 = 0x3f00_3f00;
}
