/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	22/03/2019
 */

use acpica_sys::*;

pub unsafe fn init() -> Result<(), &'static str> {
	if AcpiInitializeSubsystem() != AE_OK {
		return Err("Could not initialize subsystem");
	}
	if AcpiInitializeTables(core::ptr::null_mut(), 16, false) != AE_OK {
		return Err("Could not initialize tables");
	}
	if AcpiLoadTables() != AE_OK {
		return Err("Could not load tables");
	}
	if AcpiEnableSubsystem(0) != AE_OK {
		return Err("Could not enable subsystem");
	}
	if AcpiInitializeObjects(0) != AE_OK {
		return Err("Could not initialize objects");
	}
	Ok(())
}