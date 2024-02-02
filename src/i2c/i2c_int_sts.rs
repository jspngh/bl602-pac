#[doc = "Register `i2c_int_sts` reader"]
pub type R = crate::R<I2cIntStsSpec>;
#[doc = "Register `i2c_int_sts` writer"]
pub type W = crate::W<I2cIntStsSpec>;
#[doc = "Field `i2c_end_int` reader - "]
pub type I2cEndIntR = crate::BitReader;
#[doc = "Field `i2c_txf_int` reader - "]
pub type I2cTxfIntR = crate::BitReader;
#[doc = "Field `i2c_rxf_int` reader - "]
pub type I2cRxfIntR = crate::BitReader;
#[doc = "Field `i2c_nak_int` reader - "]
pub type I2cNakIntR = crate::BitReader;
#[doc = "Field `i2c_arb_int` reader - "]
pub type I2cArbIntR = crate::BitReader;
#[doc = "Field `i2c_fer_int` reader - "]
pub type I2cFerIntR = crate::BitReader;
#[doc = "Field `cr_i2c_end_mask` reader - "]
pub type CrI2cEndMaskR = crate::BitReader;
#[doc = "Field `cr_i2c_end_mask` writer - "]
pub type CrI2cEndMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_txf_mask` reader - "]
pub type CrI2cTxfMaskR = crate::BitReader;
#[doc = "Field `cr_i2c_txf_mask` writer - "]
pub type CrI2cTxfMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_rxf_mask` reader - "]
pub type CrI2cRxfMaskR = crate::BitReader;
#[doc = "Field `cr_i2c_rxf_mask` writer - "]
pub type CrI2cRxfMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_nak_mask` reader - "]
pub type CrI2cNakMaskR = crate::BitReader;
#[doc = "Field `cr_i2c_nak_mask` writer - "]
pub type CrI2cNakMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_arb_mask` reader - "]
pub type CrI2cArbMaskR = crate::BitReader;
#[doc = "Field `cr_i2c_arb_mask` writer - "]
pub type CrI2cArbMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_fer_mask` reader - "]
pub type CrI2cFerMaskR = crate::BitReader;
#[doc = "Field `cr_i2c_fer_mask` writer - "]
pub type CrI2cFerMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_end_clr` reader - "]
pub type CrI2cEndClrR = crate::BitReader;
#[doc = "Field `cr_i2c_end_clr` writer - "]
pub type CrI2cEndClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rsvd_17` reader - "]
pub type Rsvd17R = crate::BitReader;
#[doc = "Field `rsvd_17` writer - "]
pub type Rsvd17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rsvd_18` reader - "]
pub type Rsvd18R = crate::BitReader;
#[doc = "Field `rsvd_18` writer - "]
pub type Rsvd18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_nak_clr` reader - "]
pub type CrI2cNakClrR = crate::BitReader;
#[doc = "Field `cr_i2c_nak_clr` writer - "]
pub type CrI2cNakClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_arb_clr` reader - "]
pub type CrI2cArbClrR = crate::BitReader;
#[doc = "Field `cr_i2c_arb_clr` writer - "]
pub type CrI2cArbClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rsvd_21` reader - "]
pub type Rsvd21R = crate::BitReader;
#[doc = "Field `rsvd_21` writer - "]
pub type Rsvd21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_end_en` reader - "]
pub type CrI2cEndEnR = crate::BitReader;
#[doc = "Field `cr_i2c_end_en` writer - "]
pub type CrI2cEndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_txf_en` reader - "]
pub type CrI2cTxfEnR = crate::BitReader;
#[doc = "Field `cr_i2c_txf_en` writer - "]
pub type CrI2cTxfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_rxf_en` reader - "]
pub type CrI2cRxfEnR = crate::BitReader;
#[doc = "Field `cr_i2c_rxf_en` writer - "]
pub type CrI2cRxfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_nak_en` reader - "]
pub type CrI2cNakEnR = crate::BitReader;
#[doc = "Field `cr_i2c_nak_en` writer - "]
pub type CrI2cNakEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_arb_en` reader - "]
pub type CrI2cArbEnR = crate::BitReader;
#[doc = "Field `cr_i2c_arb_en` writer - "]
pub type CrI2cArbEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_fer_en` reader - "]
pub type CrI2cFerEnR = crate::BitReader;
#[doc = "Field `cr_i2c_fer_en` writer - "]
pub type CrI2cFerEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2c_end_int(&self) -> I2cEndIntR {
        I2cEndIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2c_txf_int(&self) -> I2cTxfIntR {
        I2cTxfIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2c_rxf_int(&self) -> I2cRxfIntR {
        I2cRxfIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2c_nak_int(&self) -> I2cNakIntR {
        I2cNakIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2c_arb_int(&self) -> I2cArbIntR {
        I2cArbIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn i2c_fer_int(&self) -> I2cFerIntR {
        I2cFerIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_i2c_end_mask(&self) -> CrI2cEndMaskR {
        CrI2cEndMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_i2c_txf_mask(&self) -> CrI2cTxfMaskR {
        CrI2cTxfMaskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_i2c_rxf_mask(&self) -> CrI2cRxfMaskR {
        CrI2cRxfMaskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_i2c_nak_mask(&self) -> CrI2cNakMaskR {
        CrI2cNakMaskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_i2c_arb_mask(&self) -> CrI2cArbMaskR {
        CrI2cArbMaskR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_i2c_fer_mask(&self) -> CrI2cFerMaskR {
        CrI2cFerMaskR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_i2c_end_clr(&self) -> CrI2cEndClrR {
        CrI2cEndClrR::new(((self.bits >> 16) & 1) != 0)
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
    pub fn cr_i2c_nak_clr(&self) -> CrI2cNakClrR {
        CrI2cNakClrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cr_i2c_arb_clr(&self) -> CrI2cArbClrR {
        CrI2cArbClrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rsvd_21(&self) -> Rsvd21R {
        Rsvd21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_i2c_end_en(&self) -> CrI2cEndEnR {
        CrI2cEndEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_i2c_txf_en(&self) -> CrI2cTxfEnR {
        CrI2cTxfEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_i2c_rxf_en(&self) -> CrI2cRxfEnR {
        CrI2cRxfEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_i2c_nak_en(&self) -> CrI2cNakEnR {
        CrI2cNakEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cr_i2c_arb_en(&self) -> CrI2cArbEnR {
        CrI2cArbEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cr_i2c_fer_en(&self) -> CrI2cFerEnR {
        CrI2cFerEnR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_i2c_end_mask(&mut self) -> CrI2cEndMaskW<'_, I2cIntStsSpec> {
        CrI2cEndMaskW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_i2c_txf_mask(&mut self) -> CrI2cTxfMaskW<'_, I2cIntStsSpec> {
        CrI2cTxfMaskW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_i2c_rxf_mask(&mut self) -> CrI2cRxfMaskW<'_, I2cIntStsSpec> {
        CrI2cRxfMaskW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_i2c_nak_mask(&mut self) -> CrI2cNakMaskW<'_, I2cIntStsSpec> {
        CrI2cNakMaskW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_i2c_arb_mask(&mut self) -> CrI2cArbMaskW<'_, I2cIntStsSpec> {
        CrI2cArbMaskW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_i2c_fer_mask(&mut self) -> CrI2cFerMaskW<'_, I2cIntStsSpec> {
        CrI2cFerMaskW::new(self, 13)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_i2c_end_clr(&mut self) -> CrI2cEndClrW<'_, I2cIntStsSpec> {
        CrI2cEndClrW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rsvd_17(&mut self) -> Rsvd17W<'_, I2cIntStsSpec> {
        Rsvd17W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rsvd_18(&mut self) -> Rsvd18W<'_, I2cIntStsSpec> {
        Rsvd18W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_i2c_nak_clr(&mut self) -> CrI2cNakClrW<'_, I2cIntStsSpec> {
        CrI2cNakClrW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cr_i2c_arb_clr(&mut self) -> CrI2cArbClrW<'_, I2cIntStsSpec> {
        CrI2cArbClrW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rsvd_21(&mut self) -> Rsvd21W<'_, I2cIntStsSpec> {
        Rsvd21W::new(self, 21)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_i2c_end_en(&mut self) -> CrI2cEndEnW<'_, I2cIntStsSpec> {
        CrI2cEndEnW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_i2c_txf_en(&mut self) -> CrI2cTxfEnW<'_, I2cIntStsSpec> {
        CrI2cTxfEnW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_i2c_rxf_en(&mut self) -> CrI2cRxfEnW<'_, I2cIntStsSpec> {
        CrI2cRxfEnW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_i2c_nak_en(&mut self) -> CrI2cNakEnW<'_, I2cIntStsSpec> {
        CrI2cNakEnW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cr_i2c_arb_en(&mut self) -> CrI2cArbEnW<'_, I2cIntStsSpec> {
        CrI2cArbEnW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cr_i2c_fer_en(&mut self) -> CrI2cFerEnW<'_, I2cIntStsSpec> {
        CrI2cFerEnW::new(self, 29)
    }
}
#[doc = "i2c_int_sts.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_int_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_int_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cIntStsSpec;
impl crate::RegisterSpec for I2cIntStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_int_sts::R`](R) reader structure"]
impl crate::Readable for I2cIntStsSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_int_sts::W`](W) writer structure"]
impl crate::Writable for I2cIntStsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets i2c_int_sts to value 0x3f00_3f00"]
impl crate::Resettable for I2cIntStsSpec {
    const RESET_VALUE: u32 = 0x3f00_3f00;
}
