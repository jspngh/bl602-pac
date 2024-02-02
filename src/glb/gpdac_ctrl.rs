#[doc = "Register `gpdac_ctrl` reader"]
pub type R = crate::R<GpdacCtrlSpec>;
#[doc = "Register `gpdac_ctrl` writer"]
pub type W = crate::W<GpdacCtrlSpec>;
#[doc = "Field `gpdaca_rstn_ana` reader - "]
pub type GpdacaRstnAnaR = crate::BitReader;
#[doc = "Field `gpdaca_rstn_ana` writer - "]
pub type GpdacaRstnAnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdacb_rstn_ana` reader - "]
pub type GpdacbRstnAnaR = crate::BitReader;
#[doc = "Field `gpdacb_rstn_ana` writer - "]
pub type GpdacbRstnAnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_test_en` reader - "]
pub type GpdacTestEnR = crate::BitReader;
#[doc = "Field `gpdac_test_en` writer - "]
pub type GpdacTestEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_ref_sel` reader - "]
pub type GpdacRefSelR = crate::BitReader;
#[doc = "Field `gpdac_ref_sel` writer - "]
pub type GpdacRefSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_test_sel` reader - "]
pub type GpdacTestSelR = crate::FieldReader;
#[doc = "Field `gpdac_test_sel` writer - "]
pub type GpdacTestSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpdac_reserved` reader - "]
pub type GpdacReservedR = crate::FieldReader;
#[doc = "Field `gpdac_reserved` writer - "]
pub type GpdacReservedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdaca_rstn_ana(&self) -> GpdacaRstnAnaR {
        GpdacaRstnAnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdacb_rstn_ana(&self) -> GpdacbRstnAnaR {
        GpdacbRstnAnaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpdac_test_en(&self) -> GpdacTestEnR {
        GpdacTestEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpdac_ref_sel(&self) -> GpdacRefSelR {
        GpdacRefSelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn gpdac_test_sel(&self) -> GpdacTestSelR {
        GpdacTestSelR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn gpdac_reserved(&self) -> GpdacReservedR {
        GpdacReservedR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdaca_rstn_ana(&mut self) -> GpdacaRstnAnaW<'_, GpdacCtrlSpec> {
        GpdacaRstnAnaW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdacb_rstn_ana(&mut self) -> GpdacbRstnAnaW<'_, GpdacCtrlSpec> {
        GpdacbRstnAnaW::new(self, 1)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpdac_test_en(&mut self) -> GpdacTestEnW<'_, GpdacCtrlSpec> {
        GpdacTestEnW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpdac_ref_sel(&mut self) -> GpdacRefSelW<'_, GpdacCtrlSpec> {
        GpdacRefSelW::new(self, 8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn gpdac_test_sel(&mut self) -> GpdacTestSelW<'_, GpdacCtrlSpec> {
        GpdacTestSelW::new(self, 9)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn gpdac_reserved(&mut self) -> GpdacReservedW<'_, GpdacCtrlSpec> {
        GpdacReservedW::new(self, 24)
    }
}
#[doc = "gpdac_ctrl.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpdac_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdac_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpdacCtrlSpec;
impl crate::RegisterSpec for GpdacCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdac_ctrl::R`](R) reader structure"]
impl crate::Readable for GpdacCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`gpdac_ctrl::W`](W) writer structure"]
impl crate::Writable for GpdacCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets gpdac_ctrl to value 0x03"]
impl crate::Resettable for GpdacCtrlSpec {
    const RESET_VALUE: u32 = 0x03;
}
