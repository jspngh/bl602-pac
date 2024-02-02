#[doc = "Register `dfe_ctrl_10` reader"]
pub type R = crate::R<DfeCtrl10Spec>;
#[doc = "Register `dfe_ctrl_10` writer"]
pub type W = crate::W<DfeCtrl10Spec>;
#[doc = "Field `dfe_dac_raw_i` reader - "]
pub type DfeDacRawIR = crate::FieldReader<u16>;
#[doc = "Field `dfe_dac_raw_i` writer - "]
pub type DfeDacRawIW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `dfe_dac_raw_q` reader - "]
pub type DfeDacRawQR = crate::FieldReader<u16>;
#[doc = "Field `dfe_dac_raw_q` writer - "]
pub type DfeDacRawQW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn dfe_dac_raw_i(&self) -> DfeDacRawIR {
        DfeDacRawIR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn dfe_dac_raw_q(&self) -> DfeDacRawQR {
        DfeDacRawQR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn dfe_dac_raw_i(&mut self) -> DfeDacRawIW<'_, DfeCtrl10Spec> {
        DfeDacRawIW::new(self, 0)
    }
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn dfe_dac_raw_q(&mut self) -> DfeDacRawQW<'_, DfeCtrl10Spec> {
        DfeDacRawQW::new(self, 16)
    }
}
#[doc = "dfe_ctrl_10.\n\nYou can [`read`](crate::Reg::read) this register and get [`dfe_ctrl_10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfe_ctrl_10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfeCtrl10Spec;
impl crate::RegisterSpec for DfeCtrl10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfe_ctrl_10::R`](R) reader structure"]
impl crate::Readable for DfeCtrl10Spec {}
#[doc = "`write(|w| ..)` method takes [`dfe_ctrl_10::W`](W) writer structure"]
impl crate::Writable for DfeCtrl10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dfe_ctrl_10 to value 0"]
impl crate::Resettable for DfeCtrl10Spec {}
