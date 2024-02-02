#[doc = "Register `ef_crc_ctrl_4` reader"]
pub type R = crate::R<EfCrcCtrl4Spec>;
#[doc = "Register `ef_crc_ctrl_4` writer"]
pub type W = crate::W<EfCrcCtrl4Spec>;
#[doc = "Field `ef_crc_golden` reader - "]
pub type EfCrcGoldenR = crate::FieldReader<u32>;
#[doc = "Field `ef_crc_golden` writer - "]
pub type EfCrcGoldenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_crc_golden(&self) -> EfCrcGoldenR {
        EfCrcGoldenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_crc_golden(&mut self) -> EfCrcGoldenW<'_, EfCrcCtrl4Spec> {
        EfCrcGoldenW::new(self, 0)
    }
}
#[doc = "ef_crc_ctrl_4.\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_crc_ctrl_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_crc_ctrl_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfCrcCtrl4Spec;
impl crate::RegisterSpec for EfCrcCtrl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_crc_ctrl_4::R`](R) reader structure"]
impl crate::Readable for EfCrcCtrl4Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_crc_ctrl_4::W`](W) writer structure"]
impl crate::Writable for EfCrcCtrl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ef_crc_ctrl_4 to value 0xc2a8_fa9d"]
impl crate::Resettable for EfCrcCtrl4Spec {
    const RESET_VALUE: u32 = 0xc2a8_fa9d;
}
