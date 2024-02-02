#[doc = "Register `pa_reg_ctrl_hw1` reader"]
pub type R = crate::R<PaRegCtrlHw1Spec>;
#[doc = "Register `pa_reg_ctrl_hw1` writer"]
pub type W = crate::W<PaRegCtrlHw1Spec>;
#[doc = "Field `pa_iet_11n` reader - "]
pub type PaIet11nR = crate::FieldReader;
#[doc = "Field `pa_iet_11n` writer - "]
pub type PaIet11nW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pa_vbcore_11n` reader - "]
pub type PaVbcore11nR = crate::FieldReader;
#[doc = "Field `pa_vbcore_11n` writer - "]
pub type PaVbcore11nW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pa_vbcas_11n` reader - "]
pub type PaVbcas11nR = crate::FieldReader;
#[doc = "Field `pa_vbcas_11n` writer - "]
pub type PaVbcas11nW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn pa_iet_11n(&self) -> PaIet11nR {
        PaIet11nR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn pa_vbcore_11n(&self) -> PaVbcore11nR {
        PaVbcore11nR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn pa_vbcas_11n(&self) -> PaVbcas11nR {
        PaVbcas11nR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn pa_iet_11n(&mut self) -> PaIet11nW<'_, PaRegCtrlHw1Spec> {
        PaIet11nW::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn pa_vbcore_11n(&mut self) -> PaVbcore11nW<'_, PaRegCtrlHw1Spec> {
        PaVbcore11nW::new(self, 16)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn pa_vbcas_11n(&mut self) -> PaVbcas11nW<'_, PaRegCtrlHw1Spec> {
        PaVbcas11nW::new(self, 20)
    }
}
#[doc = "pa_reg_ctrl_hw1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_reg_ctrl_hw1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_reg_ctrl_hw1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaRegCtrlHw1Spec;
impl crate::RegisterSpec for PaRegCtrlHw1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa_reg_ctrl_hw1::R`](R) reader structure"]
impl crate::Readable for PaRegCtrlHw1Spec {}
#[doc = "`write(|w| ..)` method takes [`pa_reg_ctrl_hw1::W`](W) writer structure"]
impl crate::Writable for PaRegCtrlHw1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets pa_reg_ctrl_hw1 to value 0"]
impl crate::Resettable for PaRegCtrlHw1Spec {}
