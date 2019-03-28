/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	22/03/2019
 */

// https://acpica.org/sites/acpica/files/acpica-reference_2.pdf

use acpica_sys::*;

pub unsafe fn init() -> Result<(), &'static str> {
	/*
	if AcpiInitializeSubsystem() != 0 {
		return Err("Could not initialize subsystem");
	}
	if AcpiInitializeTables(core::ptr::null_mut(), 16, false) != 0 {
		return Err("Could not initialize tables");
	}
	if AcpiLoadTables() != 0 {
		return Err("Could not load tables");
	}
	if AcpiEnableSubsystem(0) != 0 {
		return Err("Could not enable subsystem");
	}
	if AcpiInitializeObjects(0) != 0 {
		return Err("Could not initialize objects");
	}
	*/
	Ok(())
}