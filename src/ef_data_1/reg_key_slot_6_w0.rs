#[doc = "Register `reg_key_slot_6_w0` reader"]
pub type R = crate::R<RegKeySlot6W0Spec>;
#[doc = "Register `reg_key_slot_6_w0` writer"]
pub type W = crate::W<RegKeySlot6W0Spec>;
#[doc = "Field `reg_key_slot_6_w0` reader - "]
pub type RegKeySlot6W0R = crate::FieldReader<u32>;
#[doc = "Field `reg_key_slot_6_w0` writer - "]
pub type RegKeySlot6W0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_key_slot_6_w0(&self) -> RegKeySlot6W0R {
        RegKeySlot6W0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_key_slot_6_w0(&mut self) -> RegKeySlot6W0W<'_, RegKeySlot6W0Spec> {
        RegKeySlot6W0W::new(self, 0)
    }
}
#[doc = "reg_key_slot_6_w0.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_6_w0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_6_w0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeySlot6W0Spec;
impl crate::RegisterSpec for RegKeySlot6W0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_slot_6_w0::R`](R) reader structure"]
impl crate::Readable for RegKeySlot6W0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_key_slot_6_w0::W`](W) writer structure"]
impl crate::Writable for RegKeySlot6W0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets reg_key_slot_6_w0 to value 0"]
impl crate::Resettable for RegKeySlot6W0Spec {}
