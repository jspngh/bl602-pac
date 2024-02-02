#[doc = "Register `reg_key_slot_6_w3` reader"]
pub type R = crate::R<RegKeySlot6W3Spec>;
#[doc = "Register `reg_key_slot_6_w3` writer"]
pub type W = crate::W<RegKeySlot6W3Spec>;
#[doc = "Field `reg_key_slot_6_w3` reader - "]
pub type RegKeySlot6W3R = crate::FieldReader<u32>;
#[doc = "Field `reg_key_slot_6_w3` writer - "]
pub type RegKeySlot6W3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_key_slot_6_w3(&self) -> RegKeySlot6W3R {
        RegKeySlot6W3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_key_slot_6_w3(&mut self) -> RegKeySlot6W3W<'_, RegKeySlot6W3Spec> {
        RegKeySlot6W3W::new(self, 0)
    }
}
#[doc = "reg_key_slot_6_w3.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_6_w3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_6_w3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeySlot6W3Spec;
impl crate::RegisterSpec for RegKeySlot6W3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_slot_6_w3::R`](R) reader structure"]
impl crate::Readable for RegKeySlot6W3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_key_slot_6_w3::W`](W) writer structure"]
impl crate::Writable for RegKeySlot6W3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets reg_key_slot_6_w3 to value 0"]
impl crate::Resettable for RegKeySlot6W3Spec {}
