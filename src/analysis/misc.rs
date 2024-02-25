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
	let image_base = bin.optional_header().ImageBase;
	let _ = writeln!(f.human, "TimeDateStamp = {:#x}", time_date_stamp);
	let _ = writeln!(f.human, "CheckSum = {:#x}", check_sum);
	let _ = writeln!(f.human, "ImageBase = {:#x}", image_base);
	let _ = writeln!(f.human, "");

	let _ = writeln!(f.ini, "TimeDateStamp={:#x}", time_date_stamp);
	let _ = writeln!(f.ini, "CheckSum={:#x}", check_sum);
	let _ = writeln!(f.ini, "ImageBase={:#x}", image_base);
	let _ = writeln!(f.ini, "");
}


fn main_camera(f: &mut super::Output, bin: PeFile<'_>) {
	let mut save = [0; 5];
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

	// TODO: Divide the pattern into smaller parts to increase the chance of finding offsets
	if bin.scanner().matches_code(pat!("F20F1000 F20F1186u4 8B4008 8986???? 488B83u4 4885C0 0F84 [60-80] F20F1000 F20F1186u4 [530-570] 33D2 ?????? 488BCF F30F11?u4 F30F11????? 488B73u1")).next(&mut save) {
		let server_position = save[1];
		let _ = save[2];
		let server_velocity = save[3];
		let mouse_look = save[4];
		let state = save[5];

		let _ = writeln!(f.ini, "PlayerNetworkState!ServerPosition={:#x}", server_position);
		let _ = writeln!(f.ini, "PlayerNetworkState!ClientPosition={:#x}", server_position+0xC);
		let _ = writeln!(f.ini, "PlayerNetworkState!ServerVelocity={:#x}", server_velocity);
		let _ = writeln!(f.ini, "PlayerNetworkState!ClientVelocity={:#x}", server_velocity+0xC);
		let _ = writeln!(f.ini, "PlayerNetworkState!MouseLook={:#x}", mouse_look);
		let _ = writeln!(f.ini, "PlayerNetwork!State={:#x}", state);

	} 
	else {
		crate::print_error("unable to find player_network_state_positions!");
	}
}


/*
mov     rax, cs:MainCamera_TypeInfo																				..<--
MainCamera$$WorldToScreenPoint+4E                   test    byte ptr [rax+12Fh], 2								..
MainCamera$$WorldToScreenPoint+55                   jz      short loc_D160EF									..
MainCamera$$WorldToScreenPoint+57                   cmp     dword ptr [rax+0E0h], 0								skip
MainCamera$$WorldToScreenPoint+5E                   jnz     short loc_D160EF
MainCamera$$WorldToScreenPoint+60                   mov     rcx, rax
MainCamera$$WorldToScreenPoint+63                   call    il2cpp_runtime_class_init
MainCamera$$WorldToScreenPoint+68                   mov     rax, cs:MainCamera_TypeInfo
MainCamera$$WorldToScreenPoint+6F
MainCamera$$WorldToScreenPoint+6F   loc_D160EF:                             ; CODE XREF: MainCamera$$WorldToScreenPoint+55↑j
MainCamera$$WorldToScreenPoint+6F                                           ; MainCamera$$WorldToScreenPoint+5E↑j
MainCamera$$WorldToScreenPoint+6F                   mov     rax, [rax+0B8h]										<--
MainCamera$$WorldToScreenPoint+76                   lea     rcx, [rsp+0F8h+var_B8]								skip
MainCamera$$WorldToScreenPoint+7B                   movsd   xmm1, qword ptr [rdi]
MainCamera$$WorldToScreenPoint+7F                   xorps   xmm0, xmm0
MainCamera$$WorldToScreenPoint+82                   movss   xmm7, cs:dword_34C0AEC
MainCamera$$WorldToScreenPoint+8A                   movsd   qword ptr [rsp+0F8h+var_C8], xmm1
MainCamera$$WorldToScreenPoint+90                   movups  xmm6, xmmword ptr [rax+0A8h]						<--
MainCamera$$WorldToScreenPoint+97                   mov     [rsp+0F8h+var_D0], 0								..
MainCamera$$WorldToScreenPoint+A0                   movups  xmm8, xmmword ptr [rax+0B8h]						skip
MainCamera$$WorldToScreenPoint+A8                   movups  xmm9, xmmword ptr [rax+0C8h]
MainCamera$$WorldToScreenPoint+B0                   movups  xmm10, xmmword ptr [rax+0D8h]
MainCamera$$WorldToScreenPoint+B8                   mov     eax, [rdi+8]
MainCamera$$WorldToScreenPoint+BB                   movups  [rsp+0F8h+var_B8], xmm0
MainCamera$$WorldToScreenPoint+C0                   mov     dword ptr [rsp+0F8h+var_C8+8], eax
MainCamera$$WorldToScreenPoint+C4                   movaps  xmm0, xmm1
MainCamera$$WorldToScreenPoint+C7                   mov     dword ptr [rsp+0F8h+var_C8+8], eax
MainCamera$$WorldToScreenPoint+CB                   movss   xmm3, dword ptr [rsp+0F8h+var_C8+8]
MainCamera$$WorldToScreenPoint+D1                   shufps  xmm0, xmm0, 55h ; 'U'
MainCamera$$WorldToScreenPoint+D5                   movsd   qword ptr [rsp+0F8h+var_C8], xmm1
MainCamera$$WorldToScreenPoint+DB                   movaps  xmm2, xmm0
MainCamera$$WorldToScreenPoint+DE                   movss   xmm1, dword ptr [rdi]
MainCamera$$WorldToScreenPoint+E2                   movss   dword ptr [rsp+0F8h+var_D8], xmm7
MainCamera$$WorldToScreenPoint+E8                   call    sub_893200
MainCamera$$WorldToScreenPoint+ED                   mov     rcx, cs:qword_42A4D20
MainCamera$$WorldToScreenPoint+F4                   test    byte ptr [rcx+12Fh], 2
MainCamera$$WorldToScreenPoint+FB                   jz      short loc_D1618B
MainCamera$$WorldToScreenPoint+FD                   cmp     dword ptr [rcx+0E0h], 0
MainCamera$$WorldToScreenPoint+104                  jnz     short loc_D1618B
MainCamera$$WorldToScreenPoint+106                  call    il2cpp_runtime_class_init
MainCamera$$WorldToScreenPoint+10B
MainCamera$$WorldToScreenPoint+10B  loc_D1618B:                             ; CODE XREF: MainCamera$$WorldToScreenPoint+FB↑j
MainCamera$$WorldToScreenPoint+10B                                          ; MainCamera$$WorldToScreenPoint+104↑j
MainCamera$$WorldToScreenPoint+10B                  movups  xmm0, [rsp+0F8h+var_B8]
MainCamera$$WorldToScreenPoint+110                  xor     r9d, r9d
MainCamera$$WorldToScreenPoint+113                  lea     r8, [rsp+0F8h+var_C8]
MainCamera$$WorldToScreenPoint+118                  lea     rdx, [rsp+0F8h+var_98]
MainCamera$$WorldToScreenPoint+11D                  movaps  [rsp+0F8h+var_98], xmm6
MainCamera$$WorldToScreenPoint+122                  lea     rcx, [rsp+0F8h+var_A8]
MainCamera$$WorldToScreenPoint+127                  movdqa  [rsp+0F8h+var_C8], xmm0
MainCamera$$WorldToScreenPoint+12D                  movaps  [rsp+0F8h+var_88], xmm8
MainCamera$$WorldToScreenPoint+133                  movaps  [rsp+0F8h+var_78], xmm9
MainCamera$$WorldToScreenPoint+13C                  movaps  [rsp+0F8h+var_68], xmm10
MainCamera$$WorldToScreenPoint+145                  call    sub_2E764B0
MainCamera$$WorldToScreenPoint+14A                  xorps   xmm0, xmm0
MainCamera$$WorldToScreenPoint+14D                  movups  xmm6, xmmword ptr [rax]
MainCamera$$WorldToScreenPoint+150                  movups  [rsp+0F8h+var_B8], xmm6
MainCamera$$WorldToScreenPoint+155                  shufps  xmm6, xmm6, 0FFh
MainCamera$$WorldToScreenPoint+159                  ucomiss xmm6, xmm0
MainCamera$$WorldToScreenPoint+15C                  jp      short loc_D16228
MainCamera$$WorldToScreenPoint+15E                  jnz     short loc_D16228
MainCamera$$WorldToScreenPoint+160                  mov     rcx, cs:qword_42A53E8
MainCamera$$WorldToScreenPoint+167                  test    byte ptr [rcx+12Fh], 2
MainCamera$$WorldToScreenPoint+16E                  jz      short loc_D161FE
MainCamera$$WorldToScreenPoint+170                  cmp     dword ptr [rcx+0E0h], 0
MainCamera$$WorldToScreenPoint+177                  jnz     short loc_D161FE
MainCamera$$WorldToScreenPoint+179                  call    il2cpp_runtime_class_init
MainCamera$$WorldToScreenPoint+17E
MainCamera$$WorldToScreenPoint+17E  loc_D161FE:                             ; CODE XREF: MainCamera$$WorldToScreenPoint+16E↑j
MainCamera$$WorldToScreenPoint+17E                                          ; MainCamera$$WorldToScreenPoint+177↑j
MainCamera$$WorldToScreenPoint+17E                  xor     ecx, ecx
MainCamera$$WorldToScreenPoint+180                  call    sub_3472580
MainCamera$$WorldToScreenPoint+185                  xor     r8d, r8d
MainCamera$$WorldToScreenPoint+188                  lea     rcx, [rsp+0F8h+var_C8]
MainCamera$$WorldToScreenPoint+18D                  mov     rdx, rax
MainCamera$$WorldToScreenPoint+190                  call    sub_34726F0
MainCamera$$WorldToScreenPoint+195                  movsd   xmm0, qword ptr [rax]
MainCamera$$WorldToScreenPoint+199                  movsd   qword ptr [rbx], xmm0
MainCamera$$WorldToScreenPoint+19D                  mov     eax, [rax+8]
MainCamera$$WorldToScreenPoint+1A0                  mov     [rbx+8], eax
MainCamera$$WorldToScreenPoint+1A3                  jmp     loc_D162B9
MainCamera$$WorldToScreenPoint+1A8  ; ---------------------------------------------------------------------------
MainCamera$$WorldToScreenPoint+1A8
MainCamera$$WorldToScreenPoint+1A8  loc_D16228:                             ; CODE XREF: MainCamera$$WorldToScreenPoint+15C↑j
MainCamera$$WorldToScreenPoint+1A8                                          ; MainCamera$$WorldToScreenPoint+15E↑j
MainCamera$$WorldToScreenPoint+1A8                  mov     rax, cs:MainCamera_TypeInfo
MainCamera$$WorldToScreenPoint+1AF                  test    byte ptr [rax+12Fh], 2
MainCamera$$WorldToScreenPoint+1B6                  jz      short loc_D16250
MainCamera$$WorldToScreenPoint+1B8                  cmp     dword ptr [rax+0E0h], 0
MainCamera$$WorldToScreenPoint+1BF                  jnz     short loc_D16250
MainCamera$$WorldToScreenPoint+1C1                  mov     rcx, rax
MainCamera$$WorldToScreenPoint+1C4                  call    il2cpp_runtime_class_init
MainCamera$$WorldToScreenPoint+1C9                  mov     rax, cs:MainCamera_TypeInfo
MainCamera$$WorldToScreenPoint+1D0
MainCamera$$WorldToScreenPoint+1D0  loc_D16250:                             ; CODE XREF: MainCamera$$WorldToScreenPoint+1B6↑j
MainCamera$$WorldToScreenPoint+1D0                                          ; MainCamera$$WorldToScreenPoint+1BF↑j
MainCamera$$WorldToScreenPoint+1D0                  mov     rax, [rax+0B8h]
MainCamera$$WorldToScreenPoint+1D7                  movaps  xmm3, xmm6
MainCamera$$WorldToScreenPoint+1DA                  movss   xmm1, dword ptr [rsp+0F8h+var_B8]
MainCamera$$WorldToScreenPoint+1E0                  mov     rcx, rbx
MainCamera$$WorldToScreenPoint+1E3                  movss   xmm2, dword ptr [rsp+0F8h+var_B8+4]
MainCamera$$WorldToScreenPoint+1E9                  divss   xmm1, xmm6											..
MainCamera$$WorldToScreenPoint+1ED                  movd    xmm0, dword ptr [rax+8Ch] ; PixelWidth				<--
MainCamera$$WorldToScreenPoint+1F5                  divss   xmm2, xmm6											..
MainCamera$$WorldToScreenPoint+1F9                  cvtdq2ps xmm0, xmm0											..
MainCamera$$WorldToScreenPoint+1FC                  addss   xmm1, xmm7											..
MainCamera$$WorldToScreenPoint+200                  addss   xmm2, xmm7											..
MainCamera$$WorldToScreenPoint+204                  mulss   xmm1, cs:dword_34C0AE4
MainCamera$$WorldToScreenPoint+20C                  mulss   xmm2, cs:dword_34C0AE4
MainCamera$$WorldToScreenPoint+214                  mulss   xmm1, xmm0
MainCamera$$WorldToScreenPoint+218                  movd    xmm0, dword ptr [rax+90h] ; PixelHeight
MainCamera$$WorldToScreenPoint+220                  xor     eax, eax
MainCamera$$WorldToScreenPoint+222                  cvtdq2ps xmm0, xmm0
MainCamera$$WorldToScreenPoint+225                  mov     [rbx], rax
MainCamera$$WorldToScreenPoint+228                  mov     [rbx+8], eax
MainCamera$$WorldToScreenPoint+22B                  mov     [rsp+0F8h+var_D8], rax
MainCamera$$WorldToScreenPoint+230                  mulss   xmm2, xmm0
MainCamera$$WorldToScreenPoint+234                  call    sub_2CC0E50


*/