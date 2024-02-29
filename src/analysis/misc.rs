use std::fmt::Write;
use std::ops::Range;
use pelite;
use pelite::pe64::*;
use pelite::pattern as pat;

pub fn print(o: &mut String, bin: PeFile<'_>) {
	let _ = writeln!(o, "[Miscellaneous]");
	header(o, bin);
	main_camera(o, bin);
	player_network_state(o, bin);
	player_network_state_weapon_gadget_manager(o, bin);
	player_network_state_is_down(o, bin);
	player_network_state_statics(o, bin);
	local_player_network_state(o, bin);
	player_network(o, bin);
	bit_testing(o, bin);
	
	let _ = writeln!(o);
}

fn header(o: &mut String, bin: PeFile<'_>) {
	// Check if offsets are correct
	let time_date_stamp = bin.file_header().TimeDateStamp;
	let check_sum = bin.optional_header().CheckSum;
	let _ = writeln!(o, "TimeDateStamp={:#x}", time_date_stamp);
	let _ = writeln!(o, "CheckSum={:#x}", check_sum);
}


fn main_camera(o: &mut String, bin: PeFile<'_>) {
	let mut save = [0; 5];

	// reo: 0D16080
	// NOTE: this pattern is very reliable. Its derived from the MainCamera::WorldToScreenPoint method
	if bin.scanner().finds_code(pat!("488B05${'} 488B80B8000000 [10-40] 0F10B0 u4 48???????? [310-360] F30F5ECE ( 660F6E80 u4 | 660F6E40 u1 ) F30F5ED6 0F5BC0 F30F58CF F30F58D7"), &mut save) {
		let main_camera = save[1];
		let world_to_screen_matrix = save[2];
		let pixel_width = save[3];
		let _ = writeln!(o, "MainCamera_c={:#x}", main_camera);
		let _ = writeln!(o, "MainCamera_c!static_fields={:#x}", 0xB8);
		let _ = writeln!(o, "MainCamera_c!static_fields!WorldToScreenMatrix={:#x}", world_to_screen_matrix);
		let _ = writeln!(o, "MainCamera_c!static_fields!PixelWidth={:#x}", pixel_width);
		let _ = writeln!(o, "MainCamera_c!static_fields!PixelHeidht={:#x}", pixel_width + 4);
	}
	else {
		crate::print_error("unable to find MainCamera!");
	}
}

fn player_network_state(o: &mut String, bin: PeFile<'_>) {

	let mut save = [0;6];

	// TODO: Divide the pattern into smaller parts to avoid a single point of failure. 
	// Do this for all double space seperated patterns in the pattern
	if bin.scanner().matches_code(pat!("F20F1000 F20F1186u4 8B4008 8986???? 488B83u4 4885C0 0F84  [60-80] F20F1000 F20F1186u4  [530-570]  33D2 ?????? 488BCF F30F11?u4 F30F11????? 488B73u1")).next(&mut save) {
		let server_position = save[1];
		let _ = save[2];
		let server_velocity = save[3];
		let mouse_look = save[4];
		let state = save[5];

		let _ = writeln!(o, "PlayerNetworkState_c!fields.ServerVelocity={:#x}", server_velocity);
		let _ = writeln!(o, "PlayerNetworkState_c!fields.ClientVelocity={:#x}", server_velocity+0xC);
		let _ = writeln!(o, "PlayerNetworkState_c!fields.ServerPosition={:#x}", server_position);
		let _ = writeln!(o, "PlayerNetworkState_c!fields.ClientPosition={:#x}", server_position+0xC);
		let _ = writeln!(o, "PlayerNetworkState_c!fields.HeadPosition={:#x}", server_position+0xC+0xC);	// This offset has historically been inbetween ClientPosition and MouseLook
		let _ = writeln!(o, "PlayerNetworkState_c!fields.MouseLook={:#x}", mouse_look);
		let _ = writeln!(o, "PlayerNetwork_c!fields.State={:#x}", state);

	} 
	else {
		crate::print_error("unable to find player_network_state!");
	}
}

fn local_player_network_state(o: &mut String, bin: PeFile<'_>) {

	let mut save = [0;4];

	// reo: 14FB051
	if bin.scanner().finds_code(pat!("F30F100D${0AD7A3BD} F20F?? [185-200] E8???? 488B05${'} 488B80B8000000 488D????? 488B50u1"), &mut save) {
		let player_network_state_typeinfo = save[1];
		let local = save[2];
		let _ = writeln!(o, "PlayerNetworkState_c={:#x}", player_network_state_typeinfo);
		let _ = writeln!(o, "PlayerNetworkState_c!static_fields={:#x}", 0xB8);
		let _ = writeln!(o, "PlayerNetworkState_c!static_fields!Local={:#x}", local);
	} 
	else {
		crate::print_error("unable to find local_player_network_state!");
	}
}

fn player_network(o: &mut String, bin: PeFile<'_>) {

	let mut save = [0;4];

	// reo: CC9C7D (MainLoop)
	if bin.scanner().matches_code(pat!("488BC8 E8???? 488B05${'} 488B80B8000000 488B40u1 4885C0 0F????? 488B4010 4885C0 0F????? 4863CB 3B5818 0F83???? 488B4CC820")).next(&mut save) {
		let player_network = save[1];
		let instances_list = save[2];
		let _ = writeln!(o, "PlayerNetwork_c={:#x}", player_network);
		let _ = writeln!(o, "PlayerNetwork_c!static_fields={:#x}", 0xB8);
		let _ = writeln!(o, "PlayerNetwork_c!static_fields!FastListA_InstancesList={:#x}", instances_list);
	} 
	else {
		crate::print_error("unable to find player_network!");
	}
}

fn player_network_state_weapon_gadget_manager(o: &mut String, bin: PeFile<'_>) {
	let mut save = [0;8];

	// reo: 0129D010, PlayerNetworkState__get_CurrentToolSafe
	if bin.scanner().finds_code(pat!("0FB687u4 83F805 (77? | 0F87????) 488D15???? 8B8C82A4??? 4803CA FFE1'"), &mut save) {
		let current_loadout_index = save[1];
		let saved_pos = save[2];
		let _ = writeln!(o, "PlayerNetworkState_c!fields.CurrentLoadoutIndex={:#x}", current_loadout_index);

		let range = Range { start: saved_pos, end: saved_pos + 0x60 };
		if bin.scanner().finds(pat!("488B?u4 4883C4? [2-15] 488B?u4 4883C4? [2-15] 488B?u4 4883C4? [2-15] 488B?u4 4883C4? [2-15] 488B?u4 4883C4? [2-15] 488B?u4 4883C4?'"), range, &mut save) {
			let primary = save[1];
			let secondary = save[2];
			let first_aid = save[3];
			let tool_a = save[4];
			let tool_b = save[5];
			let throwable = save[6];
			let saved_pos = save[7];

			let _ = writeln!(o, "PlayerNetworkState_c!fields.Primary={:#x}", primary);
			let _ = writeln!(o, "PlayerNetworkState_c!fields.Secondary={:#x}", secondary);
			let _ = writeln!(o, "PlayerNetworkState_c!fields.FirstAid={:#x}", first_aid);
			let _ = writeln!(o, "PlayerNetworkState_c!fields.ToolA={:#x}", tool_a);
			let _ = writeln!(o, "PlayerNetworkState_c!fields.ToolB={:#x}", tool_b);
			let _ = writeln!(o, "PlayerNetworkState_c!fields.Throwable={:#x}", throwable);

			let range = Range { start: saved_pos, end: saved_pos + 0x200 };
			if bin.scanner().finds(pat!("488B80u4 488B5C"), range, &mut save) {
				let item = save[1];
				let _ = writeln!(o, "WeaponManager_c!fields.Item={:#x}", item);
			}
			else {
				crate::print_error("unable to find AWeapon!Item!");
			}
		}
		else {
			crate::print_error("unable to find weapon_gadget_manager!");
		}
	} 
	else {
		crate::print_error("unable to find CurrentLoadoutIndex!");
	}
}


fn player_network_state_is_down(o: &mut String, bin: PeFile<'_>) {
	let mut save = [0;4];

	// ref: 00B5E0B0, THPController::OnThreadUpdate
	if bin.scanner().finds_code(pat!("4438B0u4 0F85???? 4438B0u4 74? F30F1005${0000A041} EB?"), &mut save) {
		let is_down = save[1];
		let idk_something_that_affects_distance = save[2];
		let _ = writeln!(o, "PlayerNetworkState_c!fields.IsDown={:#x}", is_down);
		let _ = writeln!(o, "PlayerNetworkState_c!fields.IdkSomeBoolThatAffectsDistance={:#x}", idk_something_that_affects_distance);
	} 
	else {
		crate::print_error("unable to find player_network_state_is_down!");
	}
}

fn player_network_state_statics(o: &mut String, bin: PeFile<'_>) {
	let mut save = [0;4];

	// ref: 012976A0, PlayerNetworkState::Register
	if bin.scanner().finds_code(pat!("4088701084DB 0F84???? 488B05???? F6802F01000002 74? 39B0E0000000 75? 488BC8 E8???? 488B05u4 488B80B8000000 488B58u1"), &mut save) {
		let _ = save[1]; // player_network_state
		let connected_states = save[2];
		let _ = writeln!(o, "PlayerNetworkState_c!static_fields!ConnectedStates={:#x}", connected_states);
	} 
	else {
		crate::print_error("unable to find player_network_state_statics!");
	}
}

fn bit_testing(o: &mut String, bin: PeFile<'_>) {
	let mut save = [0;4];

	if bin.scanner().finds_code(pat!("E8${[60-70] 33C9 E8$ [80-90] E8???? 488B?${'} 488B80B8000000 0FB640u1} 84C0 B990D00300 0F45F1"), &mut save) {
		let bit_testing_typeinfo = save[1];
		let training_ground = save[2];
		let _ = writeln!(o, "BitTesting_c={:#x}", bit_testing_typeinfo);
		let _ = writeln!(o, "BitTesting_c!static_fields={:#x}", 0xb8);
		let _ = writeln!(o, "BitTesting_c!static_fields!TrainingGround={:#x}", training_ground);
	} 
	else {
		crate::print_error("unable to find bit_testing!");
	}
}

// reo: FirstPerson_FPSway_TypeInfo!Instance, 1550E38
// god pattern for IDA: 48 8B ?? ?? ?? ?? ??  48 8B ?? B8 00 00 00  48 8B ?8