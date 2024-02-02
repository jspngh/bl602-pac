#[doc = "Register `ppu_ctrl_hw` reader"]
pub type R = crate::R<PpuCtrlHwSpec>;
#[doc = "Register `ppu_ctrl_hw` writer"]
pub type W = crate::W<PpuCtrlHwSpec>;
#[doc = "Field `ppu_lna_hw` reader - "]
pub type PpuLnaHwR = crate::BitReader;
#[doc = "Field `ppu_lna_hw` writer - "]
pub type PpuLnaHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ppu_rmxgm_hw` reader - "]
pub type PpuRmxgmHwR = crate::BitReader;
#[doc = "Field `ppu_rmxgm_hw` writer - "]
pub type PpuRmxgmHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ppu_rbb_hw` reader - "]
pub type PpuRbbHwR = crate::BitReader;
#[doc = "Field `ppu_rbb_hw` writer - "]
pub type PpuRbbHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ppu_vco_hw` reader - "]
pub type PpuVcoHwR = crate::BitReader;
#[doc = "Field `ppu_vco_hw` writer - "]
pub type PpuVcoHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ppu_fbdv_hw` reader - "]
pub type PpuFbdvHwR = crate::BitReader;
#[doc = "Field `ppu_fbdv_hw` writer - "]
pub type PpuFbdvHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ppu_pfd_hw` reader - "]
pub type PpuPfdHwR = crate::BitReader;
#[doc = "Field `ppu_pfd_hw` writer - "]
pub type PpuPfdHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ppu_osmx_hw` reader - "]
pub type PpuOsmxHwR = crate::BitReader;
#[doc = "Field `ppu_osmx_hw` writer - "]
pub type PpuOsmxHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ppu_rxbuf_hw` reader - "]
pub type PpuRxbufHwR = crate::BitReader;
#[doc = "Field `ppu_rxbuf_hw` writer - "]
pub type PpuRxbufHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ppu_txbuf_hw` reader - "]
pub type PpuTxbufHwR = crate::BitReader;
#[doc = "Field `ppu_txbuf_hw` writer - "]
pub type PpuTxbufHwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ppu_lna_hw(&self) -> PpuLnaHwR {
        PpuLnaHwR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ppu_rmxgm_hw(&self) -> PpuRmxgmHwR {
        PpuRmxgmHwR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ppu_rbb_hw(&self) -> PpuRbbHwR {
        PpuRbbHwR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ppu_vco_hw(&self) -> PpuVcoHwR {
        PpuVcoHwR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ppu_fbdv_hw(&self) -> PpuFbdvHwR {
        PpuFbdvHwR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ppu_pfd_hw(&self) -> PpuPfdHwR {
        PpuPfdHwR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ppu_osmx_hw(&self) -> PpuOsmxHwR {
        PpuOsmxHwR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ppu_rxbuf_hw(&self) -> PpuRxbufHwR {
        PpuRxbufHwR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ppu_txbuf_hw(&self) -> PpuTxbufHwR {
        PpuTxbufHwR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ppu_lna_hw(&mut self) -> PpuLnaHwW<'_, PpuCtrlHwSpec> {
        PpuLnaHwW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ppu_rmxgm_hw(&mut self) -> PpuRmxgmHwW<'_, PpuCtrlHwSpec> {
        PpuRmxgmHwW::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ppu_rbb_hw(&mut self) -> PpuRbbHwW<'_, PpuCtrlHwSpec> {
        PpuRbbHwW::new(self, 11)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ppu_vco_hw(&mut self) -> PpuVcoHwW<'_, PpuCtrlHwSpec> {
        PpuVcoHwW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ppu_fbdv_hw(&mut self) -> PpuFbdvHwW<'_, PpuCtrlHwSpec> {
        PpuFbdvHwW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ppu_pfd_hw(&mut self) -> PpuPfdHwW<'_, PpuCtrlHwSpec> {
        PpuPfdHwW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ppu_osmx_hw(&mut self) -> PpuOsmxHwW<'_, PpuCtrlHwSpec> {
        PpuOsmxHwW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ppu_rxbuf_hw(&mut self) -> PpuRxbufHwW<'_, PpuCtrlHwSpec> {
        PpuRxbufHwW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ppu_txbuf_hw(&mut self) -> PpuTxbufHwW<'_, PpuCtrlHwSpec> {
        PpuTxbufHwW::new(self, 25)
    }
}
#[doc = "ppu_ctrl_hw.\n\nYou can [`read`](crate::Reg::read) this register and get [`ppu_ctrl_hw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppu_ctrl_hw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpuCtrlHwSpec;
impl crate::RegisterSpec for PpuCtrlHwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppu_ctrl_hw::R`](R) reader structure"]
impl crate::Readable for PpuCtrlHwSpec {}
#[doc = "`write(|w| ..)` method takes [`ppu_ctrl_hw::W`](W) writer structure"]
impl crate::Writable for PpuCtrlHwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ppu_ctrl_hw to value 0"]
impl crate::Resettable for PpuCtrlHwSpec {}
