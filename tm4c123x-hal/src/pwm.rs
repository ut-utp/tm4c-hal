
use hal::Pwm;
//#[doc(inline)]
pub struct Tm4cPwm{

}

impl Pwm for Tm4cPwm{

	type Channel = ();
	type Time = u32;
	type Duty = u32;


	fn disable(&mut self, _channel: Self::Channel){}


fn enable(&mut self, _channel: Self::Channel){}


fn get_period(&self) -> Self::Time{
	4
}


fn get_duty(&self, _channel: Self::Channel) -> Self::Duty{
	4
}


fn get_max_duty(&self) -> Self::Duty{
	4
}


fn set_duty(&mut self, _channel: Self::Channel, _duty: Self::Duty){}


fn set_period<P>(&mut self, _period: P){}

}