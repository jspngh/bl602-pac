#[doc = "Register `pa_reg_ctrl_hw2` reader"]
pub type R = crate::R<PaRegCtrlHw2Spec>;
#[doc = "Register `pa_reg_ctrl_hw2` writer"]
pub type W = crate::W<PaRegCtrlHw2Spec>;
#[doc = "Field `pa_iet_11g` reader - "]
pub type PaIet11gR = crate::FieldReader;
#[doc = "Field `pa_iet_11g` writer - "]
pub type PaIet11gW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pa_vbcore_11g` reader - "]
pub type PaVbcore11gR = crate::FieldReader;
#[doc = "Field `pa_vbcore_11g` writer - "]
pub type PaVbcore11gW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pa_vbcas_11g` reader - "]
pub type PaVbcas11gR = crate::FieldReader;
#[doc = "Field `pa_vbcas_11g` writer - "]
pub type PaVbcas11gW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pa_iet_11b` reader - "]
pub type PaIet11bR = crate::FieldReader;
#[doc = "Field `pa_iet_11b` writer - "]
pub type PaIet11bW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pa_vbcore_11b` reader - "]
pub type PaVbcore11bR = crate::FieldReader;
#[doc = "Field `pa_vbcore_11b` writer - "]
pub type PaVbcore11bW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pa_vbcas_11b` reader - "]
pub type PaVbcas11bR = crate::FieldReader;
#[doc = "Field `pa_vbcas_11b` writer - "]
pub type PaVbcas11bW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pa_iet_11g(&self) -> PaIet11gR {
        PaIet11gR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_vbcore_11g(&self) -> PaVbcore11gR {
        PaVbcore11gR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pa_vbcas_11g(&self) -> PaVbcas11gR {
        PaVbcas11gR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn pa_iet_11b(&self) -> PaIet11bR {
        PaIet11bR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn pa_vbcore_11b(&self) -> PaVbcore11bR {
        PaVbcore11bR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn pa_vbcas_11b(&self) -> PaVbcas11bR {
        PaVbcas11bR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pa_iet_11g(&mut self) -> PaIet11gW<'_, PaRegCtrlHw2Spec> {
        PaIet11gW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_vbcore_11g(&mut self) -> PaVbcore11gW<'_, PaRegCtrlHw2Spec> {
        PaVbcore11gW::new(self, 4)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pa_vbcas_11g(&mut self) -> PaVbcas11gW<'_, PaRegCtrlHw2Spec> {
        PaVbcas11gW::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn pa_iet_11b(&mut self) -> PaIet11bW<'_, PaRegCtrlHw2Spec> {
        PaIet11bW::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn pa_vbcore_11b(&mut self) -> PaVbcore11bW<'_, PaRegCtrlHw2Spec> {
        PaVbcore11bW::new(self, 16)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn pa_vbcas_11b(&mut self) -> PaVbcas11bW<'_, PaRegCtrlHw2Spec> {
        PaVbcas11bW::new(self, 20)
    }
}
#[doc = "pa_reg_ctrl_hw2.\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_reg_ctrl_hw2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_reg_ctrl_hw2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaRegCtrlHw2Spec;
impl crate::RegisterSpec for PaRegCtrlHw2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa_reg_ctrl_hw2::R`](R) reader structure"]
impl crate::Readable for PaRegCtrlHw2Spec {}
#[doc = "`write(|w| ..)` method takes [`pa_reg_ctrl_hw2::W`](W) writer structure"]
impl crate::Writable for PaRegCtrlHw2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets pa_reg_ctrl_hw2 to value 0"]
impl crate::Resettable for PaRegCtrlHw2Spec {}
