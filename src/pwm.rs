#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pwm_int_config: PwmIntConfig,
    _reserved1: [u8; 0x1c],
    pwm0_clkdiv: Pwm0Clkdiv,
    pwm0_thre1: Pwm0Thre1,
    pwm0_thre2: Pwm0Thre2,
    pwm0_period: Pwm0Period,
    pwm0_config: Pwm0Config,
    pwm0_interrupt: Pwm0Interrupt,
    _reserved7: [u8; 0x08],
    pwm1_clkdiv: Pwm1Clkdiv,
    pwm1_thre1: Pwm1Thre1,
    pwm1_thre2: Pwm1Thre2,
    pwm1_period: Pwm1Period,
    pwm1_config: Pwm1Config,
    pwm1_interrupt: Pwm1Interrupt,
    _reserved13: [u8; 0x08],
    pwm2_clkdiv: Pwm2Clkdiv,
    pwm2_thre1: Pwm2Thre1,
    pwm2_thre2: Pwm2Thre2,
    pwm2_period: Pwm2Period,
    pwm2_config: Pwm2Config,
    pwm2_interrupt: Pwm2Interrupt,
    _reserved19: [u8; 0x08],
    pwm3_clkdiv: Pwm3Clkdiv,
    pwm3_thre1: Pwm3Thre1,
    pwm3_thre2: Pwm3Thre2,
    pwm3_period: Pwm3Period,
    pwm3_config: Pwm3Config,
    pwm3_interrupt: Pwm3Interrupt,
    _reserved25: [u8; 0x08],
    pwm4_clkdiv: Pwm4Clkdiv,
    pwm4_thre1: Pwm4Thre1,
    pwm4_thre2: Pwm4Thre2,
    pwm4_period: Pwm4Period,
    pwm4_config: Pwm4Config,
    pwm4_interrupt: Pwm4Interrupt,
}
impl RegisterBlock {
    #[doc = "0x00 - pwm_int_config."]
    #[inline(always)]
    pub const fn pwm_int_config(&self) -> &PwmIntConfig {
        &self.pwm_int_config
    }
    #[doc = "0x20 - pwm0_clkdiv."]
    #[inline(always)]
    pub const fn pwm0_clkdiv(&self) -> &Pwm0Clkdiv {
        &self.pwm0_clkdiv
    }
    #[doc = "0x24 - pwm0_thre1."]
    #[inline(always)]
    pub const fn pwm0_thre1(&self) -> &Pwm0Thre1 {
        &self.pwm0_thre1
    }
    #[doc = "0x28 - pwm0_thre2."]
    #[inline(always)]
    pub const fn pwm0_thre2(&self) -> &Pwm0Thre2 {
        &self.pwm0_thre2
    }
    #[doc = "0x2c - pwm0_period."]
    #[inline(always)]
    pub const fn pwm0_period(&self) -> &Pwm0Period {
        &self.pwm0_period
    }
    #[doc = "0x30 - pwm0_config."]
    #[inline(always)]
    pub const fn pwm0_config(&self) -> &Pwm0Config {
        &self.pwm0_config
    }
    #[doc = "0x34 - pwm0_interrupt."]
    #[inline(always)]
    pub const fn pwm0_interrupt(&self) -> &Pwm0Interrupt {
        &self.pwm0_interrupt
    }
    #[doc = "0x40 - pwm1_clkdiv."]
    #[inline(always)]
    pub const fn pwm1_clkdiv(&self) -> &Pwm1Clkdiv {
        &self.pwm1_clkdiv
    }
    #[doc = "0x44 - pwm1_thre1."]
    #[inline(always)]
    pub const fn pwm1_thre1(&self) -> &Pwm1Thre1 {
        &self.pwm1_thre1
    }
    #[doc = "0x48 - pwm1_thre2."]
    #[inline(always)]
    pub const fn pwm1_thre2(&self) -> &Pwm1Thre2 {
        &self.pwm1_thre2
    }
    #[doc = "0x4c - pwm1_period."]
    #[inline(always)]
    pub const fn pwm1_period(&self) -> &Pwm1Period {
        &self.pwm1_period
    }
    #[doc = "0x50 - pwm1_config."]
    #[inline(always)]
    pub const fn pwm1_config(&self) -> &Pwm1Config {
        &self.pwm1_config
    }
    #[doc = "0x54 - pwm1_interrupt."]
    #[inline(always)]
    pub const fn pwm1_interrupt(&self) -> &Pwm1Interrupt {
        &self.pwm1_interrupt
    }
    #[doc = "0x60 - pwm2_clkdiv."]
    #[inline(always)]
    pub const fn pwm2_clkdiv(&self) -> &Pwm2Clkdiv {
        &self.pwm2_clkdiv
    }
    #[doc = "0x64 - pwm2_thre1."]
    #[inline(always)]
    pub const fn pwm2_thre1(&self) -> &Pwm2Thre1 {
        &self.pwm2_thre1
    }
    #[doc = "0x68 - pwm2_thre2."]
    #[inline(always)]
    pub const fn pwm2_thre2(&self) -> &Pwm2Thre2 {
        &self.pwm2_thre2
    }
    #[doc = "0x6c - pwm2_period."]
    #[inline(always)]
    pub const fn pwm2_period(&self) -> &Pwm2Period {
        &self.pwm2_period
    }
    #[doc = "0x70 - pwm2_config."]
    #[inline(always)]
    pub const fn pwm2_config(&self) -> &Pwm2Config {
        &self.pwm2_config
    }
    #[doc = "0x74 - pwm2_interrupt."]
    #[inline(always)]
    pub const fn pwm2_interrupt(&self) -> &Pwm2Interrupt {
        &self.pwm2_interrupt
    }
    #[doc = "0x80 - pwm3_clkdiv."]
    #[inline(always)]
    pub const fn pwm3_clkdiv(&self) -> &Pwm3Clkdiv {
        &self.pwm3_clkdiv
    }
    #[doc = "0x84 - pwm3_thre1."]
    #[inline(always)]
    pub const fn pwm3_thre1(&self) -> &Pwm3Thre1 {
        &self.pwm3_thre1
    }
    #[doc = "0x88 - pwm3_thre2."]
    #[inline(always)]
    pub const fn pwm3_thre2(&self) -> &Pwm3Thre2 {
        &self.pwm3_thre2
    }
    #[doc = "0x8c - pwm3_period."]
    #[inline(always)]
    pub const fn pwm3_period(&self) -> &Pwm3Period {
        &self.pwm3_period
    }
    #[doc = "0x90 - pwm3_config."]
    #[inline(always)]
    pub const fn pwm3_config(&self) -> &Pwm3Config {
        &self.pwm3_config
    }
    #[doc = "0x94 - pwm3_interrupt."]
    #[inline(always)]
    pub const fn pwm3_interrupt(&self) -> &Pwm3Interrupt {
        &self.pwm3_interrupt
    }
    #[doc = "0xa0 - pwm4_clkdiv."]
    #[inline(always)]
    pub const fn pwm4_clkdiv(&self) -> &Pwm4Clkdiv {
        &self.pwm4_clkdiv
    }
    #[doc = "0xa4 - pwm4_thre1."]
    #[inline(always)]
    pub const fn pwm4_thre1(&self) -> &Pwm4Thre1 {
        &self.pwm4_thre1
    }
    #[doc = "0xa8 - pwm4_thre2."]
    #[inline(always)]
    pub const fn pwm4_thre2(&self) -> &Pwm4Thre2 {
        &self.pwm4_thre2
    }
    #[doc = "0xac - pwm4_period."]
    #[inline(always)]
    pub const fn pwm4_period(&self) -> &Pwm4Period {
        &self.pwm4_period
    }
    #[doc = "0xb0 - pwm4_config."]
    #[inline(always)]
    pub const fn pwm4_config(&self) -> &Pwm4Config {
        &self.pwm4_config
    }
    #[doc = "0xb4 - pwm4_interrupt."]
    #[inline(always)]
    pub const fn pwm4_interrupt(&self) -> &Pwm4Interrupt {
        &self.pwm4_interrupt
    }
}
#[doc = "pwm_int_config (rw) register accessor: pwm_int_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm_int_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm_int_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_int_config`] module"]
#[doc(alias = "pwm_int_config")]
pub type PwmIntConfig = crate::Reg<pwm_int_config::PwmIntConfigSpec>;
#[doc = "pwm_int_config."]
pub mod pwm_int_config;
#[doc = "pwm0_clkdiv (rw) register accessor: pwm0_clkdiv.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm0_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm0_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm0_clkdiv`] module"]
#[doc(alias = "pwm0_clkdiv")]
pub type Pwm0Clkdiv = crate::Reg<pwm0_clkdiv::Pwm0ClkdivSpec>;
#[doc = "pwm0_clkdiv."]
pub mod pwm0_clkdiv;
#[doc = "pwm0_thre1 (rw) register accessor: pwm0_thre1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm0_thre1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm0_thre1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm0_thre1`] module"]
#[doc(alias = "pwm0_thre1")]
pub type Pwm0Thre1 = crate::Reg<pwm0_thre1::Pwm0Thre1Spec>;
#[doc = "pwm0_thre1."]
pub mod pwm0_thre1;
#[doc = "pwm0_thre2 (rw) register accessor: pwm0_thre2.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm0_thre2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm0_thre2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm0_thre2`] module"]
#[doc(alias = "pwm0_thre2")]
pub type Pwm0Thre2 = crate::Reg<pwm0_thre2::Pwm0Thre2Spec>;
#[doc = "pwm0_thre2."]
pub mod pwm0_thre2;
#[doc = "pwm0_period (rw) register accessor: pwm0_period.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm0_period::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm0_period::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm0_period`] module"]
#[doc(alias = "pwm0_period")]
pub type Pwm0Period = crate::Reg<pwm0_period::Pwm0PeriodSpec>;
#[doc = "pwm0_period."]
pub mod pwm0_period;
#[doc = "pwm0_config (rw) register accessor: pwm0_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm0_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm0_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm0_config`] module"]
#[doc(alias = "pwm0_config")]
pub type Pwm0Config = crate::Reg<pwm0_config::Pwm0ConfigSpec>;
#[doc = "pwm0_config."]
pub mod pwm0_config;
#[doc = "pwm0_interrupt (rw) register accessor: pwm0_interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm0_interrupt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm0_interrupt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm0_interrupt`] module"]
#[doc(alias = "pwm0_interrupt")]
pub type Pwm0Interrupt = crate::Reg<pwm0_interrupt::Pwm0InterruptSpec>;
#[doc = "pwm0_interrupt."]
pub mod pwm0_interrupt;
#[doc = "pwm1_clkdiv (rw) register accessor: pwm1_clkdiv.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_clkdiv`] module"]
#[doc(alias = "pwm1_clkdiv")]
pub type Pwm1Clkdiv = crate::Reg<pwm1_clkdiv::Pwm1ClkdivSpec>;
#[doc = "pwm1_clkdiv."]
pub mod pwm1_clkdiv;
#[doc = "pwm1_thre1 (rw) register accessor: pwm1_thre1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_thre1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_thre1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_thre1`] module"]
#[doc(alias = "pwm1_thre1")]
pub type Pwm1Thre1 = crate::Reg<pwm1_thre1::Pwm1Thre1Spec>;
#[doc = "pwm1_thre1."]
pub mod pwm1_thre1;
#[doc = "pwm1_thre2 (rw) register accessor: pwm1_thre2.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_thre2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_thre2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_thre2`] module"]
#[doc(alias = "pwm1_thre2")]
pub type Pwm1Thre2 = crate::Reg<pwm1_thre2::Pwm1Thre2Spec>;
#[doc = "pwm1_thre2."]
pub mod pwm1_thre2;
#[doc = "pwm1_period (rw) register accessor: pwm1_period.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_period::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_period::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_period`] module"]
#[doc(alias = "pwm1_period")]
pub type Pwm1Period = crate::Reg<pwm1_period::Pwm1PeriodSpec>;
#[doc = "pwm1_period."]
pub mod pwm1_period;
#[doc = "pwm1_config (rw) register accessor: pwm1_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_config`] module"]
#[doc(alias = "pwm1_config")]
pub type Pwm1Config = crate::Reg<pwm1_config::Pwm1ConfigSpec>;
#[doc = "pwm1_config."]
pub mod pwm1_config;
#[doc = "pwm1_interrupt (rw) register accessor: pwm1_interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_interrupt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_interrupt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm1_interrupt`] module"]
#[doc(alias = "pwm1_interrupt")]
pub type Pwm1Interrupt = crate::Reg<pwm1_interrupt::Pwm1InterruptSpec>;
#[doc = "pwm1_interrupt."]
pub mod pwm1_interrupt;
#[doc = "pwm2_clkdiv (rw) register accessor: pwm2_clkdiv.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm2_clkdiv`] module"]
#[doc(alias = "pwm2_clkdiv")]
pub type Pwm2Clkdiv = crate::Reg<pwm2_clkdiv::Pwm2ClkdivSpec>;
#[doc = "pwm2_clkdiv."]
pub mod pwm2_clkdiv;
#[doc = "pwm2_thre1 (rw) register accessor: pwm2_thre1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2_thre1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2_thre1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm2_thre1`] module"]
#[doc(alias = "pwm2_thre1")]
pub type Pwm2Thre1 = crate::Reg<pwm2_thre1::Pwm2Thre1Spec>;
#[doc = "pwm2_thre1."]
pub mod pwm2_thre1;
#[doc = "pwm2_thre2 (rw) register accessor: pwm2_thre2.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2_thre2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2_thre2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm2_thre2`] module"]
#[doc(alias = "pwm2_thre2")]
pub type Pwm2Thre2 = crate::Reg<pwm2_thre2::Pwm2Thre2Spec>;
#[doc = "pwm2_thre2."]
pub mod pwm2_thre2;
#[doc = "pwm2_period (rw) register accessor: pwm2_period.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2_period::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2_period::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm2_period`] module"]
#[doc(alias = "pwm2_period")]
pub type Pwm2Period = crate::Reg<pwm2_period::Pwm2PeriodSpec>;
#[doc = "pwm2_period."]
pub mod pwm2_period;
#[doc = "pwm2_config (rw) register accessor: pwm2_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm2_config`] module"]
#[doc(alias = "pwm2_config")]
pub type Pwm2Config = crate::Reg<pwm2_config::Pwm2ConfigSpec>;
#[doc = "pwm2_config."]
pub mod pwm2_config;
#[doc = "pwm2_interrupt (rw) register accessor: pwm2_interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2_interrupt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2_interrupt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm2_interrupt`] module"]
#[doc(alias = "pwm2_interrupt")]
pub type Pwm2Interrupt = crate::Reg<pwm2_interrupt::Pwm2InterruptSpec>;
#[doc = "pwm2_interrupt."]
pub mod pwm2_interrupt;
#[doc = "pwm3_clkdiv (rw) register accessor: pwm3_clkdiv.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm3_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm3_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm3_clkdiv`] module"]
#[doc(alias = "pwm3_clkdiv")]
pub type Pwm3Clkdiv = crate::Reg<pwm3_clkdiv::Pwm3ClkdivSpec>;
#[doc = "pwm3_clkdiv."]
pub mod pwm3_clkdiv;
#[doc = "pwm3_thre1 (rw) register accessor: pwm3_thre1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm3_thre1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm3_thre1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm3_thre1`] module"]
#[doc(alias = "pwm3_thre1")]
pub type Pwm3Thre1 = crate::Reg<pwm3_thre1::Pwm3Thre1Spec>;
#[doc = "pwm3_thre1."]
pub mod pwm3_thre1;
#[doc = "pwm3_thre2 (rw) register accessor: pwm3_thre2.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm3_thre2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm3_thre2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm3_thre2`] module"]
#[doc(alias = "pwm3_thre2")]
pub type Pwm3Thre2 = crate::Reg<pwm3_thre2::Pwm3Thre2Spec>;
#[doc = "pwm3_thre2."]
pub mod pwm3_thre2;
#[doc = "pwm3_period (rw) register accessor: pwm3_period.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm3_period::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm3_period::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm3_period`] module"]
#[doc(alias = "pwm3_period")]
pub type Pwm3Period = crate::Reg<pwm3_period::Pwm3PeriodSpec>;
#[doc = "pwm3_period."]
pub mod pwm3_period;
#[doc = "pwm3_config (rw) register accessor: pwm3_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm3_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm3_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm3_config`] module"]
#[doc(alias = "pwm3_config")]
pub type Pwm3Config = crate::Reg<pwm3_config::Pwm3ConfigSpec>;
#[doc = "pwm3_config."]
pub mod pwm3_config;
#[doc = "pwm3_interrupt (rw) register accessor: pwm3_interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm3_interrupt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm3_interrupt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm3_interrupt`] module"]
#[doc(alias = "pwm3_interrupt")]
pub type Pwm3Interrupt = crate::Reg<pwm3_interrupt::Pwm3InterruptSpec>;
#[doc = "pwm3_interrupt."]
pub mod pwm3_interrupt;
#[doc = "pwm4_clkdiv (rw) register accessor: pwm4_clkdiv.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm4_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm4_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm4_clkdiv`] module"]
#[doc(alias = "pwm4_clkdiv")]
pub type Pwm4Clkdiv = crate::Reg<pwm4_clkdiv::Pwm4ClkdivSpec>;
#[doc = "pwm4_clkdiv."]
pub mod pwm4_clkdiv;
#[doc = "pwm4_thre1 (rw) register accessor: pwm4_thre1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm4_thre1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm4_thre1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm4_thre1`] module"]
#[doc(alias = "pwm4_thre1")]
pub type Pwm4Thre1 = crate::Reg<pwm4_thre1::Pwm4Thre1Spec>;
#[doc = "pwm4_thre1."]
pub mod pwm4_thre1;
#[doc = "pwm4_thre2 (rw) register accessor: pwm4_thre2.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm4_thre2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm4_thre2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm4_thre2`] module"]
#[doc(alias = "pwm4_thre2")]
pub type Pwm4Thre2 = crate::Reg<pwm4_thre2::Pwm4Thre2Spec>;
#[doc = "pwm4_thre2."]
pub mod pwm4_thre2;
#[doc = "pwm4_period (rw) register accessor: pwm4_period.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm4_period::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm4_period::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm4_period`] module"]
#[doc(alias = "pwm4_period")]
pub type Pwm4Period = crate::Reg<pwm4_period::Pwm4PeriodSpec>;
#[doc = "pwm4_period."]
pub mod pwm4_period;
#[doc = "pwm4_config (rw) register accessor: pwm4_config.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm4_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm4_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm4_config`] module"]
#[doc(alias = "pwm4_config")]
pub type Pwm4Config = crate::Reg<pwm4_config::Pwm4ConfigSpec>;
#[doc = "pwm4_config."]
pub mod pwm4_config;
#[doc = "pwm4_interrupt (rw) register accessor: pwm4_interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm4_interrupt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm4_interrupt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm4_interrupt`] module"]
#[doc(alias = "pwm4_interrupt")]
pub type Pwm4Interrupt = crate::Reg<pwm4_interrupt::Pwm4InterruptSpec>;
#[doc = "pwm4_interrupt."]
pub mod pwm4_interrupt;
