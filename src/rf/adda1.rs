#[doc = "Register `adda1` reader"]
pub type R = crate::R<Adda1Spec>;
#[doc = "Register `adda1` writer"]
pub type W = crate::W<Adda1Spec>;
#[doc = "Field `dac_dvdd_sel` reader - "]
pub type DacDvddSelR = crate::FieldReader;
#[doc = "Field `dac_dvdd_sel` writer - "]
pub type DacDvddSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dac_bias_sel` reader - "]
pub type DacBiasSelR = crate::FieldReader;
#[doc = "Field `dac_bias_sel` writer - "]
pub type DacBiasSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dac_clk_sel` reader - "]
pub type DacClkSelR = crate::FieldReader;
#[doc = "Field `dac_clk_sel` writer - "]
pub type DacClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dac_rccalsel` reader - "]
pub type DacRccalselR = crate::BitReader;
#[doc = "Field `dac_rccalsel` writer - "]
pub type DacRccalselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_clk_sync_inv` reader - "]
pub type DacClkSyncInvR = crate::BitReader;
#[doc = "Field `dac_clk_sync_inv` writer - "]
pub type DacClkSyncInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adda_ldo_byps` reader - "]
pub type AddaLdoBypsR = crate::BitReader;
#[doc = "Field `adda_ldo_byps` writer - "]
pub type AddaLdoBypsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adda_ldo_dvdd_sel` reader - "]
pub type AddaLdoDvddSelR = crate::FieldReader;
#[doc = "Field `adda_ldo_dvdd_sel` writer - "]
pub type AddaLdoDvddSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `adda_ldo_dvdd_sel_hw` reader - "]
pub type AddaLdoDvddSelHwR = crate::FieldReader;
#[doc = "Field `adda_ldo_dvdd_sel_hw` writer - "]
pub type AddaLdoDvddSelHwW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dac_dvdd_sel(&self) -> DacDvddSelR {
        DacDvddSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dac_bias_sel(&self) -> DacBiasSelR {
        DacBiasSelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn dac_clk_sel(&self) -> DacClkSelR {
        DacClkSelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dac_rccalsel(&self) -> DacRccalselR {
        DacRccalselR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dac_clk_sync_inv(&self) -> DacClkSyncInvR {
        DacClkSyncInvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adda_ldo_byps(&self) -> AddaLdoBypsR {
        AddaLdoBypsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel(&self) -> AddaLdoDvddSelR {
        AddaLdoDvddSelR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel_hw(&self) -> AddaLdoDvddSelHwR {
        AddaLdoDvddSelHwR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dac_dvdd_sel(&mut self) -> DacDvddSelW<'_, Adda1Spec> {
        DacDvddSelW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dac_bias_sel(&mut self) -> DacBiasSelW<'_, Adda1Spec> {
        DacBiasSelW::new(self, 4)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn dac_clk_sel(&mut self) -> DacClkSelW<'_, Adda1Spec> {
        DacClkSelW::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dac_rccalsel(&mut self) -> DacRccalselW<'_, Adda1Spec> {
        DacRccalselW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dac_clk_sync_inv(&mut self) -> DacClkSyncInvW<'_, Adda1Spec> {
        DacClkSyncInvW::new(self, 13)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adda_ldo_byps(&mut self) -> AddaLdoBypsW<'_, Adda1Spec> {
        AddaLdoBypsW::new(self, 16)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel(&mut self) -> AddaLdoDvddSelW<'_, Adda1Spec> {
        AddaLdoDvddSelW::new(self, 20)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel_hw(&mut self) -> AddaLdoDvddSelHwW<'_, Adda1Spec> {
        AddaLdoDvddSelHwW::new(self, 24)
    }
}
#[doc = "adda1.\n\nYou can [`read`](crate::Reg::read) this register and get [`adda1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adda1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adda1Spec;
impl crate::RegisterSpec for Adda1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adda1::R`](R) reader structure"]
impl crate::Readable for Adda1Spec {}
#[doc = "`write(|w| ..)` method takes [`adda1::W`](W) writer structure"]
impl crate::Writable for Adda1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adda1 to value 0"]
impl crate::Resettable for Adda1Spec {}
