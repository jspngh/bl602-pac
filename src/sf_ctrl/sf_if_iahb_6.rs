#[doc = "Register `sf_if_iahb_6` reader"]
pub type R = crate::R<SfIfIahb6Spec>;
#[doc = "Register `sf_if_iahb_6` writer"]
pub type W = crate::W<SfIfIahb6Spec>;
#[doc = "Field `sf_if_3_cmd_byte` reader - "]
pub type SfIf3CmdByteR = crate::FieldReader;
#[doc = "Field `sf_if_3_cmd_byte` writer - "]
pub type SfIf3CmdByteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_if_3_spi_mode` reader - "]
pub type SfIf3SpiModeR = crate::FieldReader;
#[doc = "Field `sf_if_3_spi_mode` writer - "]
pub type SfIf3SpiModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_if_3_qpi_mode_en` reader - "]
pub type SfIf3QpiModeEnR = crate::BitReader;
#[doc = "Field `sf_if_3_qpi_mode_en` writer - "]
pub type SfIf3QpiModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_3_cmd_byte(&self) -> SfIf3CmdByteR {
        SfIf3CmdByteR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn sf_if_3_spi_mode(&self) -> SfIf3SpiModeR {
        SfIf3SpiModeR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if_3_qpi_mode_en(&self) -> SfIf3QpiModeEnR {
        SfIf3QpiModeEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_3_cmd_byte(&mut self) -> SfIf3CmdByteW<'_, SfIfIahb6Spec> {
        SfIf3CmdByteW::new(self, 20)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn sf_if_3_spi_mode(&mut self) -> SfIf3SpiModeW<'_, SfIfIahb6Spec> {
        SfIf3SpiModeW::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if_3_qpi_mode_en(&mut self) -> SfIf3QpiModeEnW<'_, SfIfIahb6Spec> {
        SfIf3QpiModeEnW::new(self, 31)
    }
}
#[doc = "sf_if_iahb_6.\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIfIahb6Spec;
impl crate::RegisterSpec for SfIfIahb6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if_iahb_6::R`](R) reader structure"]
impl crate::Readable for SfIfIahb6Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_if_iahb_6::W`](W) writer structure"]
impl crate::Writable for SfIfIahb6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sf_if_iahb_6 to value 0x8000_0000"]
impl crate::Resettable for SfIfIahb6Spec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
