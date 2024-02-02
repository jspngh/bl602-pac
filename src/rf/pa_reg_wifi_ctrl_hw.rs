#[doc = "Register `pa_reg_wifi_ctrl_hw` reader"]
pub type R = crate::R<PaRegWifiCtrlHwSpec>;
#[doc = "Register `pa_reg_wifi_ctrl_hw` writer"]
pub type W = crate::W<PaRegWifiCtrlHwSpec>;
#[doc = "Field `pa_half_on_wifi` reader - "]
pub type PaHalfOnWifiR = crate::BitReader;
#[doc = "Field `pa_half_on_wifi` writer - "]
pub type PaHalfOnWifiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pa_etb_en_wifi` reader - "]
pub type PaEtbEnWifiR = crate::BitReader;
#[doc = "Field `pa_etb_en_wifi` writer - "]
pub type PaEtbEnWifiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pa_ib_fix_wifi` reader - "]
pub type PaIbFixWifiR = crate::BitReader;
#[doc = "Field `pa_ib_fix_wifi` writer - "]
pub type PaIbFixWifiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pa_half_on_wifi(&self) -> PaHalfOnWifiR {
        PaHalfOnWifiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pa_etb_en_wifi(&self) -> PaEtbEnWifiR {
        PaEtbEnWifiR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pa_ib_fix_wifi(&self) -> PaIbFixWifiR {
        PaIbFixWifiR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pa_half_on_wifi(&mut self) -> PaHalfOnWifiW<'_, PaRegWifiCtrlHwSpec> {
        PaHalfOnWifiW::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pa_etb_en_wifi(&mut self) -> PaEtbEnWifiW<'_, PaRegWifiCtrlHwSpec> {
        PaEtbEnWifiW::new(self, 8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pa_ib_fix_wifi(&mut self) -> PaIbFixWifiW<'_, PaRegWifiCtrlHwSpec> {
        PaIbFixWifiW::new(self, 16)
    }
}
#[doc = "pa_reg_wifi_ctrl_hw.\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_reg_wifi_ctrl_hw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_reg_wifi_ctrl_hw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaRegWifiCtrlHwSpec;
impl crate::RegisterSpec for PaRegWifiCtrlHwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa_reg_wifi_ctrl_hw::R`](R) reader structure"]
impl crate::Readable for PaRegWifiCtrlHwSpec {}
#[doc = "`write(|w| ..)` method takes [`pa_reg_wifi_ctrl_hw::W`](W) writer structure"]
impl crate::Writable for PaRegWifiCtrlHwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets pa_reg_wifi_ctrl_hw to value 0"]
impl crate::Resettable for PaRegWifiCtrlHwSpec {}
