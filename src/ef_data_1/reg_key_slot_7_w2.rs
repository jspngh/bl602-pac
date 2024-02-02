#[doc = "Register `reg_key_slot_7_w2` reader"]
pub type R = crate::R<RegKeySlot7W2Spec>;
#[doc = "Register `reg_key_slot_7_w2` writer"]
pub type W = crate::W<RegKeySlot7W2Spec>;
#[doc = "Field `reg_key_slot_7_w2` reader - "]
pub type RegKeySlot7W2R = crate::FieldReader<u32>;
#[doc = "Field `reg_key_slot_7_w2` writer - "]
pub type RegKeySlot7W2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_key_slot_7_w2(&self) -> RegKeySlot7W2R {
        RegKeySlot7W2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_key_slot_7_w2(&mut self) -> RegKeySlot7W2W<'_, RegKeySlot7W2Spec> {
        RegKeySlot7W2W::new(self, 0)
    }
}
#[doc = "reg_key_slot_7_w2.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_7_w2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_7_w2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeySlot7W2Spec;
impl crate::RegisterSpec for RegKeySlot7W2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_slot_7_w2::R`](R) reader structure"]
impl crate::Readable for RegKeySlot7W2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_key_slot_7_w2::W`](W) writer structure"]
impl crate::Writable for RegKeySlot7W2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets reg_key_slot_7_w2 to value 0"]
impl crate::Resettable for RegKeySlot7W2Spec {}
