
use hal::Pwm;
use hal::PwmPin;
extern crate tm4c123x;
use tm4c123x::{PWM0};
use tm4c123x::{GPIO_PORTB};
use crate::gpio::gpiob::{PB6, PB7};
//use crate::gpio;
//#[doc(inline)]
use crate::gpio::{AlternateFunction, OutputMode, AF4};
use crate::sysctl;
use crate::sysctl::Clocks;
pub unsafe trait Pwm_trait<PWM> {}



unsafe impl<T> Pwm_trait<PWM0> for PB6<AlternateFunction<AF4, T>> where T: OutputMode {}
unsafe impl<T> Pwm_trait<PWM0> for PB7<AlternateFunction<AF4, T>> where T: OutputMode {}


//Will soon macro this after testing

pub struct Tm4cPwm<PINS>{
	//pwm1: PWM0,
	pins: PINS,
	pwm: PWM0,
	period: u32,
	duty: u32,
	// pin1: PB6,
	// pin2: PB7,

}
pub enum Channels{
	PWM0,
	PWM1,
}

impl <PWM_PINS > Tm4cPwm<PWM_PINS >
	where
	PWM_PINS: Pwm_trait<PWM0>,
{
    pub fn new(pwm: PWM0, pins: PWM_PINS, pc: &sysctl::PowerControl) -> Self {


        sysctl::control_power(
            pc, sysctl::Domain::Pwm0,
            sysctl::RunMode::Run, sysctl::PowerState::On);
        sysctl::reset(pc, sysctl::Domain::Pwm0);
        pwm.ctl.write(|w| unsafe { w.bits(0) });
            pwm._0_gena
            .write(|w| unsafe { w.bits(0xC8) });
        Self{
            //states: PinArr([State::Disabled; Pin::NUM_PINS]),
            //components: Some(peripheral_set),
            //pwm1: ad0,
            pins: pins,
            pwm: pwm,
            period: 40000,
            duty: 4000,
            // pin1: pb6,
            // pin2: pb7,

        }
    }

}

impl <PWM_PINS > Pwm for Tm4cPwm<PWM_PINS >
	where
	PWM_PINS: Pwm_trait<PWM0>,
{

	type Channel = Channels;
	type Time = u32;
	type Duty = u32;


	fn disable(&mut self, _channel: Self::Channel){
		match _channel{
			Channels::PWM0 => {
               self.pwm.enable
               .write(|w| unsafe { w.bits(self.pwm.enable.read().bits() & !1) });				

			},
			Channels::PWM1 => {

			},

		};
	}


fn enable(&mut self, _channel: Self::Channel){
		match _channel{
			Channels::PWM0 => {
	        self.pwm._0_load.write(|w| unsafe { w.bits(self.period) });
	         self.pwm._0_cmpa.write(|w| unsafe { w.bits(self.duty) });
	         self.pwm._0_ctl
	            .write(|w| unsafe { w.bits(self.pwm._0_ctl.read().bits() | 1) });
	         self.pwm.enable
	            .write(|w| unsafe { w.bits(self.pwm.enable.read().bits() | 1) }); 				

			},
			Channels::PWM1 => {

			},

		};


}


fn get_period(&self) -> Self::Time{
	self.period
}


fn get_duty(&self, _channel: Self::Channel) -> Self::Duty{
	self.duty
}


fn get_max_duty(&self) -> Self::Duty{
	40000
}


fn set_duty(&mut self, _channel: Self::Channel, duty: Self::Duty){
		match _channel{
			Channels::PWM0 => {
				self.disable(Channels::PWM0);
	        	//self.pwm._0_load.write(|w| unsafe { w.bits(40000) });
	         	self.pwm._0_cmpa.write(|w| unsafe { w.bits(duty) });				 				
	         	self.pwm.enable
	            .write(|w| unsafe { w.bits(self.pwm.enable.read().bits() | 1) }); 
	            self.duty = duty;
			},
			Channels::PWM1 => {

			},

		};

}


fn set_period <P>(&mut self, period: P)
where P: Into<Self::Time>

{
	let curr_period: u32 = period.into();
	self.pwm._0_load.write(|w| unsafe { w.bits(curr_period) });
	self.period=curr_period;
}




}








