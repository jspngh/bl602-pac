#[doc = "Register `i2c_config` reader"]
pub type R = crate::R<I2cConfigSpec>;
#[doc = "Register `i2c_config` writer"]
pub type W = crate::W<I2cConfigSpec>;
#[doc = "Field `cr_i2c_m_en` reader - "]
pub type CrI2cMEnR = crate::BitReader;
#[doc = "Field `cr_i2c_m_en` writer - "]
pub type CrI2cMEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_pkt_dir` reader - "]
pub type CrI2cPktDirR = crate::BitReader;
#[doc = "Field `cr_i2c_pkt_dir` writer - "]
pub type CrI2cPktDirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_deg_en` reader - "]
pub type CrI2cDegEnR = crate::BitReader;
#[doc = "Field `cr_i2c_deg_en` writer - "]
pub type CrI2cDegEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_scl_sync_en` reader - "]
pub type CrI2cSclSyncEnR = crate::BitReader;
#[doc = "Field `cr_i2c_scl_sync_en` writer - "]
pub type CrI2cSclSyncEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_sub_addr_en` reader - "]
pub type CrI2cSubAddrEnR = crate::BitReader;
#[doc = "Field `cr_i2c_sub_addr_en` writer - "]
pub type CrI2cSubAddrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_sub_addr_bc` reader - "]
pub type CrI2cSubAddrBcR = crate::FieldReader;
#[doc = "Field `cr_i2c_sub_addr_bc` writer - "]
pub type CrI2cSubAddrBcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_i2c_slv_addr` reader - "]
pub type CrI2cSlvAddrR = crate::FieldReader;
#[doc = "Field `cr_i2c_slv_addr` writer - "]
pub type CrI2cSlvAddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `cr_i2c_pkt_len` reader - "]
pub type CrI2cPktLenR = crate::FieldReader;
#[doc = "Field `cr_i2c_pkt_len` writer - "]
pub type CrI2cPktLenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_i2c_deg_cnt` reader - "]
pub type CrI2cDegCntR = crate::FieldReader;
#[doc = "Field `cr_i2c_deg_cnt` writer - "]
pub type CrI2cDegCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_i2c_m_en(&self) -> CrI2cMEnR {
        CrI2cMEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_i2c_pkt_dir(&self) -> CrI2cPktDirR {
        CrI2cPktDirR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_i2c_deg_en(&self) -> CrI2cDegEnR {
        CrI2cDegEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_i2c_scl_sync_en(&self) -> CrI2cSclSyncEnR {
        CrI2cSclSyncEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_en(&self) -> CrI2cSubAddrEnR {
        CrI2cSubAddrEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_bc(&self) -> CrI2cSubAddrBcR {
        CrI2cSubAddrBcR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn cr_i2c_slv_addr(&self) -> CrI2cSlvAddrR {
        CrI2cSlvAddrR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_i2c_pkt_len(&self) -> CrI2cPktLenR {
        CrI2cPktLenR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn cr_i2c_deg_cnt(&self) -> CrI2cDegCntR {
        CrI2cDegCntR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_i2c_m_en(&mut self) -> CrI2cMEnW<'_, I2cConfigSpec> {
        CrI2cMEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_i2c_pkt_dir(&mut self) -> CrI2cPktDirW<'_, I2cConfigSpec> {
        CrI2cPktDirW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_i2c_deg_en(&mut self) -> CrI2cDegEnW<'_, I2cConfigSpec> {
        CrI2cDegEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_i2c_scl_sync_en(&mut self) -> CrI2cSclSyncEnW<'_, I2cConfigSpec> {
        CrI2cSclSyncEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_en(&mut self) -> CrI2cSubAddrEnW<'_, I2cConfigSpec> {
        CrI2cSubAddrEnW::new(self, 4)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_bc(&mut self) -> CrI2cSubAddrBcW<'_, I2cConfigSpec> {
        CrI2cSubAddrBcW::new(self, 5)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn cr_i2c_slv_addr(&mut self) -> CrI2cSlvAddrW<'_, I2cConfigSpec> {
        CrI2cSlvAddrW::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_i2c_pkt_len(&mut self) -> CrI2cPktLenW<'_, I2cConfigSpec> {
        CrI2cPktLenW::new(self, 16)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn cr_i2c_deg_cnt(&mut self) -> CrI2cDegCntW<'_, I2cConfigSpec> {
        CrI2cDegCntW::new(self, 28)
    }
}
#[doc = "i2c_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cConfigSpec;
impl crate::RegisterSpec for I2cConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_config::R`](R) reader structure"]
impl crate::Readable for I2cConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_config::W`](W) writer structure"]
impl crate::Writable for I2cConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets i2c_config to value 0x0a"]
impl crate::Resettable for I2cConfigSpec {
    const RESET_VALUE: u32 = 0x0a;
}
