#[doc = "Register `reg_key_slot_11_w2` reader"]
pub type R = crate::R<RegKeySlot11W2Spec>;
#[doc = "Register `reg_key_slot_11_w2` writer"]
pub type W = crate::W<RegKeySlot11W2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "reg_key_slot_11_w2.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_key_slot_11_w2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_key_slot_11_w2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeySlot11W2Spec;
impl crate::RegisterSpec for RegKeySlot11W2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_key_slot_11_w2::R`](R) reader structure"]
impl crate::Readable for RegKeySlot11W2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_key_slot_11_w2::W`](W) writer structure"]
impl crate::Writable for RegKeySlot11W2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets reg_key_slot_11_w2 to value 0"]
impl crate::Resettable for RegKeySlot11W2Spec {}
