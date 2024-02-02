#[doc = "Register `adda_reg_ctrl_hw` reader"]
pub type R = crate::R<AddaRegCtrlHwSpec>;
#[doc = "Register `adda_reg_ctrl_hw` writer"]
pub type W = crate::W<AddaRegCtrlHwSpec>;
#[doc = "Field `adda_ldo_dvdd_sel_rx` reader - "]
pub type AddaLdoDvddSelRxR = crate::FieldReader;
#[doc = "Field `adda_ldo_dvdd_sel_rx` writer - "]
pub type AddaLdoDvddSelRxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `adda_ldo_dvdd_sel_tx` reader - "]
pub type AddaLdoDvddSelTxR = crate::FieldReader;
#[doc = "Field `adda_ldo_dvdd_sel_tx` writer - "]
pub type AddaLdoDvddSelTxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel_rx(&self) -> AddaLdoDvddSelRxR {
        AddaLdoDvddSelRxR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel_tx(&self) -> AddaLdoDvddSelTxR {
        AddaLdoDvddSelTxR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel_rx(&mut self) -> AddaLdoDvddSelRxW<'_, AddaRegCtrlHwSpec> {
        AddaLdoDvddSelRxW::new(self, 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn adda_ldo_dvdd_sel_tx(&mut self) -> AddaLdoDvddSelTxW<'_, AddaRegCtrlHwSpec> {
        AddaLdoDvddSelTxW::new(self, 4)
    }
}
#[doc = "adda_reg_ctrl_hw.\n\nYou can [`read`](crate::Reg::read) this register and get [`adda_reg_ctrl_hw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adda_reg_ctrl_hw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddaRegCtrlHwSpec;
impl crate::RegisterSpec for AddaRegCtrlHwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adda_reg_ctrl_hw::R`](R) reader structure"]
impl crate::Readable for AddaRegCtrlHwSpec {}
#[doc = "`write(|w| ..)` method takes [`adda_reg_ctrl_hw::W`](W) writer structure"]
impl crate::Writable for AddaRegCtrlHwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adda_reg_ctrl_hw to value 0"]
impl crate::Resettable for AddaRegCtrlHwSpec {}
