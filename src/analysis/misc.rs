use std::fmt::Write;
use std::ops::Range;
use pelite;
use pelite::pe64::*;
use pelite::pattern as pat;

pub fn print(f: &mut super::Output, bin: PeFile<'_>) {
	let _ = writeln!(f.human, "## Miscellaneous\n\n```");
	let _ = writeln!(f.ini, "[Miscellaneous]");
	header(f, bin);
	main_camera(f, bin);
	player_network_state(f, bin);
	local_player_network_state(f, bin);
	player_network(f, bin);
	// entity_list(f, bin);
	// local_entity_handle(f, bin);
	// local_player(f, bin);
	// name_list(f, bin);
	// view_render(f, bin);
	// local_camera(f, bin);
	let _ = writeln!(f.human, "```\n");
	let _ = writeln!(f.ini);
}

fn header(f: &mut super::Output, bin: PeFile<'_>) {
	// Check if offsets are correct
	let time_date_stamp = bin.file_header().TimeDateStamp;
	let check_sum = bin.optional_header().CheckSum;
	let _ = writeln!(f.human, "TimeDateStamp = {:#x}", time_date_stamp);
	let _ = writeln!(f.human, "CheckSum = {:#x}", check_sum);
	let _ = writeln!(f.human, "");

	let _ = writeln!(f.ini, "TimeDateStamp={:#x}", time_date_stamp);
	let _ = writeln!(f.ini, "CheckSum={:#x}", check_sum);
	let _ = writeln!(f.ini, "");
}


fn main_camera(f: &mut super::Output, bin: PeFile<'_>) {
	let mut save = [0; 5];

	// ref: 0D16080
	// NOTE: this pattern is very reliable. Its derived from the MainCamera::WorldToScreenPoint method
	if bin.scanner().finds_code(pat!("488B05${'} 488B80 u4 [10-40] 0F10B0 u4 48???????? [310-360] F30F5ECE ( 660F6E80 u4 | 660F6E40 u1 ) F30F5ED6 0F5BC0 F30F58CF F30F58D7"), &mut save) {
		let main_camera = save[1];
		let static_fields = save[2];
		let world_to_screen_matrix = save[3];
		let pixel_width = save[4];
		let _ = writeln!(f.ini, "MainCamera_c={:#x}", main_camera);
		let _ = writeln!(f.ini, "MainCamera_c!static_fields={:#x}", static_fields);
		let _ = writeln!(f.ini, "MainCamera_c!x={:#x}", 0);
		let _ = writeln!(f.ini, "MainCamera_c!y={:#x}", 4);
		let _ = writeln!(f.ini, "MainCamera_c!WorldToScreenMatrix={:#x}", world_to_screen_matrix);
		let _ = writeln!(f.ini, "MainCamera_c!PixelWidth={:#x}", pixel_width);
		let _ = writeln!(f.ini, "MainCamera_c!PixelHeidht={:#x}", pixel_width + 4);
	}
	else {
		crate::print_error("unable to find MainCamera!");
	}
}

fn player_network_state(f: &mut super::Output, bin: PeFile<'_>) {

	let mut save = [0;6];

	// TODO: Divide the pattern into smaller parts to avoid a single point of failure. 
	// Do this for all double space seperated patterns in the pattern
	if bin.scanner().matches_code(pat!("F20F1000 F20F1186u4 8B4008 8986???? 488B83u4 4885C0 0F84  [60-80] F20F1000 F20F1186u4  [530-570]  33D2 ?????? 488BCF F30F11?u4 F30F11????? 488B73u1")).next(&mut save) {
		let server_position = save[1];
		let _ = save[2];
		let server_velocity = save[3];
		let mouse_look = save[4];
		let state = save[5];

		let _ = writeln!(f.ini, "PlayerNetworkState_c!ServerVelocity={:#x}", server_velocity);
		let _ = writeln!(f.ini, "PlayerNetworkState_c!ClientVelocity={:#x}", server_velocity+0xC);
		let _ = writeln!(f.ini, "PlayerNetworkState_c!ServerPosition={:#x}", server_position);
		let _ = writeln!(f.ini, "PlayerNetworkState_c!ClientPosition={:#x}", server_position+0xC);
		let _ = writeln!(f.ini, "PlayerNetworkState_c!HeadPosition={:#x}", server_position+0xC+0xC);	// This offset has historically been inbetween ClientPosition and MouseLook
		let _ = writeln!(f.ini, "PlayerNetworkState_c!MouseLook={:#x}", mouse_look);
		let _ = writeln!(f.ini, "PlayerNetwork_c!State={:#x}", state);

	} 
	else {
		crate::print_error("unable to find player_network_state!");
	}
}

fn local_player_network_state(f: &mut super::Output, bin: PeFile<'_>) {

	let mut save = [0;4];

	// ref: 14FB051
	if bin.scanner().finds_code(pat!("F30F100D${0AD7A3BD} F20F?? [185-200] E8???? 488B05${'} 488B80B8000000 488D????? 488B50u1"), &mut save) {
		let player_network_state_typeinfo = save[1];
		let local = save[2];
		let _ = writeln!(f.ini, "PlayerNetworkState_c={:#x}", player_network_state_typeinfo);
		let _ = writeln!(f.ini, "PlayerNetworkState_c!Local={:#x}", local);
	} 
	else {
		crate::print_error("unable to find local_player_network_state!");
	}
}

fn player_network(f: &mut super::Output, bin: PeFile<'_>) {

	let mut save = [0;4];

	// ref: CC9C7D (MainLoop)
	if bin.scanner().matches_code(pat!("488BC8 E8???? 488B05${'} 488B80B8000000 488B40u1 4885C0 0F????? 488B4010 4885C0 0F????? 4863CB 3B5818 0F83???? 488B4CC820")).next(&mut save) {
		let player_network = save[1];
		let instances_list = save[2];
		let _ = writeln!(f.ini, "PlayerNetwork_c={:#x}", player_network);
		let _ = writeln!(f.ini, "PlayerNetwork_c!FastListA_InstancesList={:#x}", instances_list);
	} 
	else {
		crate::print_error("unable to find player_network!");
	}
}

// ref: FirstPerson_FPSway_TypeInfo!Instance, 1550E38