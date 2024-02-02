#[doc = "Register `rbb_bw_ctrl_hw` reader"]
pub type R = crate::R<RbbBwCtrlHwSpec>;
#[doc = "Register `rbb_bw_ctrl_hw` writer"]
pub type W = crate::W<RbbBwCtrlHwSpec>;
#[doc = "Field `rbb_bt_mode_ble` reader - "]
pub type RbbBtModeBleR = crate::BitReader;
#[doc = "Field `rbb_bt_mode_ble` writer - "]
pub type RbbBtModeBleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rbb_bt_mode_ble(&self) -> RbbBtModeBleR {
        RbbBtModeBleR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rbb_bt_mode_ble(&mut self) -> RbbBtModeBleW<'_, RbbBwCtrlHwSpec> {
        RbbBtModeBleW::new(self, 0)
    }
}
#[doc = "rbb_bw_ctrl_hw.\n\nYou can [`read`](crate::Reg::read) this register and get [`rbb_bw_ctrl_hw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbb_bw_ctrl_hw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbbBwCtrlHwSpec;
impl crate::RegisterSpec for RbbBwCtrlHwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbb_bw_ctrl_hw::R`](R) reader structure"]
impl crate::Readable for RbbBwCtrlHwSpec {}
#[doc = "`write(|w| ..)` method takes [`rbb_bw_ctrl_hw::W`](W) writer structure"]
impl crate::Writable for RbbBwCtrlHwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rbb_bw_ctrl_hw to value 0"]
impl crate::Resettable for RbbBwCtrlHwSpec {}
