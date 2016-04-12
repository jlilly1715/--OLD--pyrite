use super::super::core::memory::*;

pub mod tiles;
pub mod obj;

// #TODO remove these allows
pub mod mode0;

#[allow(unused_variables)]
pub mod mode1;

#[allow(unused_variables)]
pub mod mode2;

pub mod mode3;
pub mod mode4;
pub mod mode5;

pub type Pixel = u16;
pub type GbaPixel = (u8, u8, u8);
pub type GbaLcdLine = Vec<GbaPixel>;
pub type GbaBGLine = [Pixel; 240];

pub struct ObjLineInfo {
	/// I stuff all of the object's metadata in here.
	/// bits 0-2 priority
	/// bit  3   window
	/// bit  4   semi-transparent
	pub data: [u8; 240]
}

impl ObjLineInfo {
	#[inline(always)]
	pub fn get_priority(&self, idx: usize) -> u8 {
		self.data[idx] & 0x7
	}

	#[inline(always)]
	pub fn set_priority(&mut self, idx: usize, priority: u8) {
		self.data[idx] &= !0x7; // priorities can be overwritten...and stuff.
		self.data[idx] |= priority & 0x7;
	}

	#[inline(always)]
	pub fn is_window(&self, idx: usize) -> bool {
		(self.data[idx] & 0x8) != 0
	}

	#[inline(always)]
	pub fn set_window(&mut self, idx: usize) {
		self.data[idx] |= 0x8;
	}

	#[inline(always)]
	pub fn is_transparent(&self, idx: usize) -> bool {
		(self.data[idx] & 0x10) != 0
	}

	#[inline(always)]
	pub fn set_transparent(&mut self, idx: usize) {
		self.data[idx] |= 0x10;
	}

	#[inline(always)]
	pub fn clear_transparent(&mut self, idx: usize) {
		self.data[idx] &= !0x10;
	}
}

// No point in having a secondary screen buffer
// since the GBA renders in scan lines anyway.
pub type GbaLcdScreenBuffer = Vec<GbaLcdLine>;

#[derive(Default)]
struct BlendingParams {
	target_drawn: bool,
	source_on_top: bool,
	target_overwritten: bool,
	source_pixel: Pixel,
	target_pixel: Pixel,
	force_obj_blend: bool,
	current_window_prio: u8,
	window_blending_disabled: bool
}

impl BlendingParams {
	fn reset_for_window(&mut self, window_prio: u8, window_blending_disabled: bool) {
		self.target_drawn = false;
		self.source_on_top = false;
		self.target_overwritten = false;
		self.force_obj_blend = false;
		self.current_window_prio = window_prio;
		self.window_blending_disabled = window_blending_disabled;
	}
}

// Add a way for modes themselves to turn these off
// mode 3 for instance only uses bg2, no need for the others.
pub struct GbaDisplayLines {
	pub bg0: GbaBGLine,
	pub bg1: GbaBGLine,
	pub bg2: GbaBGLine,
	pub bg3: GbaBGLine,
	pub obj: GbaBGLine,

	pub bg0_enable: bool,
	pub bg1_enable: bool,
	pub bg2_enable: bool,
	pub bg3_enable: bool,

	pub obj_info: ObjLineInfo
}

pub struct GbaLcd {
	pub screen_buffer: GbaLcdScreenBuffer,
	pub lines: GbaDisplayLines
}

impl GbaLcd {
	pub fn new() -> GbaLcd {
		GbaLcd {
			screen_buffer: vec![vec![(0u8, 0u8, 0u8); 240]; 160],
			lines: GbaDisplayLines {
				bg0: [0; 240],
				bg1: [0; 240],
				bg2: [0; 240],
				bg3: [0; 240],
				obj: [0; 240],
				bg0_enable: false,
				bg1_enable: false,
				bg2_enable: false,
				bg3_enable: false,
				obj_info: ObjLineInfo { data: [0u8; 240] }
			}
		}
	}

	pub fn render_line(&mut self, memory: &mut GbaMemory, line: u16) {
		let dispcnt = memory.get_reg(ioreg::DISPCNT);
		self.clear_obj_line();

		match dispcnt & 0x7 {
			0 => mode0::render_mode_0(dispcnt, memory, line, &mut self.lines),
			1 => mode1::render_mode_1(dispcnt, memory, line, &mut self.lines),
			2 => mode2::render_mode_2(dispcnt, memory, line, &mut self.lines),
			3 => mode3::render_mode_3(dispcnt, memory, line, &mut self.lines),
			4 => mode4::render_mode_4(dispcnt, memory, line, &mut self.lines),
			5 => mode5::render_mode_5(dispcnt, memory, line, &mut self.lines),
			_ => unreachable!()
		}

		self.blend_line(line, memory);
	}

	fn clear_obj_line(&mut self) {
		for i in 0..240 {
			self.lines.obj[i] = 0;
			self.lines.obj_info.data[i] = 0;
		}
	}

	// #TODO Consider making a second version of this function that doesn't use windows
	//       at all. This seems to be how VBA does it but right now I'm not sure if the performance
	//       gain will be worth the added complexity of maintaining both functions.
	fn blend_line(&mut self, line: u16, memory: &GbaMemory) {
		let dispcnt = memory.get_reg(ioreg::DISPCNT);

		let backdrop = convert_rgb5_to_rgb8(memory.read16(0x05000000));
		let output = &mut self.screen_buffer[line as usize][0..240];

		let bg0_enabled = self.lines.bg0_enable && (((dispcnt >> 8) & 1) != 0);
		let bg1_enabled = self.lines.bg1_enable && (((dispcnt >> 9) & 1) != 0);
		let bg2_enabled = self.lines.bg2_enable && (((dispcnt >> 10) & 1) != 0);
		let bg3_enabled = self.lines.bg3_enable && (((dispcnt >> 11) & 1) != 0);

		let obj_enabled = ((dispcnt >> 12) & 1) != 0;

		let bg0_priority = memory.get_reg(ioreg::BG0CNT) & 0x3;
		let bg1_priority = memory.get_reg(ioreg::BG1CNT) & 0x3;
		let bg2_priority = memory.get_reg(ioreg::BG2CNT) & 0x3;
		let bg3_priority = memory.get_reg(ioreg::BG3CNT) & 0x3;

		let bldcnt = memory.get_reg(ioreg::BLDCNT);
		let blend_mode = (bldcnt >> 6) & 0x3;

		// (priority, bg, line)
		let mut rendering_order: [(u8, u16, Option<&GbaBGLine>); 4] = [(0, 0, None), (0, 0, None), (0, 0, None), (0, 0, None)];
		let mut rendering_order_idx = 0;
		for priority in (0..4).rev() {
			let mut temp_priority = rendering_order_idx;

			if bg3_enabled && bg3_priority == priority {
				rendering_order[rendering_order_idx] = (priority as u8, 3, Some(&self.lines.bg3));
				rendering_order_idx += 1;
			}

			if bg2_enabled && bg2_priority == priority {
				rendering_order[rendering_order_idx] = (priority as u8, 2, Some(&self.lines.bg2));
				rendering_order_idx += 1;
			}

			if bg1_enabled && bg1_priority == priority {
				rendering_order[rendering_order_idx] = (priority as u8, 1, Some(&self.lines.bg1));
				rendering_order_idx += 1;
			}

			if bg0_enabled && bg0_priority == priority {
				rendering_order[rendering_order_idx] = (priority as u8, 0, Some(&self.lines.bg0));
				rendering_order_idx += 1;
			}
		}

		// I'll have borrowing issues if I don't define this here like so:
		let obj_info = &self.lines.obj_info;
		let obj_line = &self.lines.obj;

		let blend_sources = bldcnt & 0x3f;
		let blend_targets = (bldcnt >> 8) & 0x3f;

		let blend_alpha = memory.get_reg(ioreg::BLDALPHA);
		let blend_eva = min!(16, blend_alpha & 0x1f);
		let blend_evb = min!(16, (blend_alpha >> 8) & 0x1f);

		let bldy = memory.get_reg(ioreg::BLDY);
		let blend_evy = bldy & 0x1f;

		let pixel_brightness_fn: fn(u16, GbaPixel) -> GbaPixel = match blend_mode {
			2 => brighten_pixel,
			3 => darken_pixel,
			_ => pixel_lum_nop
		};

		let mut blending_params: BlendingParams = Default::default();

		let change_pixel_brightness = |blend_evy: u16, color: GbaPixel, blending_params: &mut BlendingParams| -> GbaPixel {
			if !blending_params.window_blending_disabled {
				return pixel_brightness_fn(blend_evy, color);
			} else {
				return color
			}
		};

		let win0_enabled = ((dispcnt >> 13) & 1) == 1;
		let win1_enabled = ((dispcnt >> 14) & 1) == 1;
		let win_obj_enabled = ((dispcnt >> 15) & 1) == 1;

		let win0h = memory.get_reg(ioreg::WIN0H);
		let win1h = memory.get_reg(ioreg::WIN1H);
		let win0v = memory.get_reg(ioreg::WIN0V);
		let win1v = memory.get_reg(ioreg::WIN1V);
		let winin = memory.get_reg(ioreg::WININ);
		let winout = memory.get_reg(ioreg::WINOUT);

		let win0_left = (win0h >> 8) & 0xff; // inclusive
		let win0_right = min!(240, win0h & 0xff); // exclusive
		let win0_top = (win0v >> 8) & 0xff; // inclusive
		let win0_bottom = min!(160, win0v & 0xff); // exclusive

		let win1_left = (win1h >> 8) & 0xff; // inclusive
		let win1_right = min!(240, win1h & 0xff); // exclusive
		let win1_top = (win1v >> 8) & 0xff; // inclusive
		let win1_bottom = min!(160, win1v & 0xff); // exclusive

		let win0_in = winin & 0x1f;
		let win1_in = (winin >> 8) & 0x1f;
		let winout_in = winout & 0x1f;
		let win_obj_in = (winout >> 8) & 0x1f;

		let mut on_pixel_drawn = |layer_idx: u16, color: Pixel, force_source: bool, blending_params: &mut BlendingParams| {
			if !is_transparent(color) && !blending_params.window_blending_disabled {
				if force_source || ((blend_sources >> layer_idx) & 1) == 1 { // This is a source layer.
					blending_params.source_on_top = true;
					blending_params.source_pixel = color;
				} else if ((blend_targets >> layer_idx) & 1) == 1 { // This is a target layer.
					if blending_params.target_drawn {
						blending_params.target_overwritten = true;
					} else {
						blending_params.target_drawn = true;
						blending_params.target_pixel = color;
					}
					blending_params.source_on_top = false;
				}
			}
		};

		on_pixel_drawn(5, backdrop, false, &mut blending_params);
		let darkened_backdrop = backdrop;

		let mut window_clip_pixel = |line: u16, column: u16, src_pixel: Pixel, layer_idx: u16, dest_pixel: &mut Pixel, blending_params: &mut BlendingParams| -> bool {
			if !win0_enabled && !win1_enabled && !win_obj_enabled { // Windowing is turned off.
				return true;
			}
			let mut pwindow_priority = 0;
			let window_disables_blending;
			if win0_enabled && window_contains(column, line, win0_left, win0_right, win0_top, win0_bottom) {
				pwindow_priority = 4;
				window_disables_blending = ((winin >> 5) & 1) == 0;
				if ((win0_in >> layer_idx) & 1) == 0 {
					if pwindow_priority > blending_params.current_window_prio {
						blending_params.reset_for_window(pwindow_priority, window_disables_blending); // #FIXME do I need to be doing this?
						// we basically act like the backdrop has been drawn again.
						*dest_pixel = darkened_backdrop;
						on_pixel_drawn(5, backdrop, false, blending_params);
					}
					return false;
				}
			} else if win1_enabled && window_contains(column, line, win1_left, win1_right, win1_top, win1_bottom) {
				pwindow_priority = 3;
				window_disables_blending = ((winin >> 13) & 1) == 0;
				if ((win1_in >> layer_idx) & 1) == 0 {
					if pwindow_priority > blending_params.current_window_prio {
						blending_params.reset_for_window(pwindow_priority, window_disables_blending);
						// we basically act like the backdrop has been drawn again.
						*dest_pixel = darkened_backdrop;
						on_pixel_drawn(5, backdrop, false, blending_params);
					}
					return false;
				}
			} else if obj_info.is_window(column as usize) {
				pwindow_priority = 2;
				window_disables_blending = ((winout >> 13) & 1) == 0;
				if ((win_obj_in >> layer_idx) & 1) == 0 {
					if pwindow_priority > blending_params.current_window_prio {
						blending_params.reset_for_window(pwindow_priority, window_disables_blending);
						// we basically act like the backdrop has been drawn again.
						*dest_pixel = darkened_backdrop;
						on_pixel_drawn(5, backdrop, false, blending_params);
					}
					return false;
				}
			} else {
				pwindow_priority = 1;
				window_disables_blending = ((winout >> 5) & 1) == 0;
				if ((winout_in >> layer_idx) & 1) == 0 {
					if pwindow_priority > blending_params.current_window_prio {
						blending_params.reset_for_window(pwindow_priority, window_disables_blending);
						// we basically act like the backdrop has been drawn again.
						*dest_pixel = darkened_backdrop;
						on_pixel_drawn(5, backdrop, false, blending_params);
					}
					return false;
				}
			}

			if pwindow_priority < blending_params.current_window_prio {
				return false
			} else if pwindow_priority > blending_params.current_window_prio {
				blending_params.reset_for_window(pwindow_priority, window_disables_blending);
				// we basically act like the backdrop has been drawn again.
				*dest_pixel = darkened_backdrop;
				on_pixel_drawn(5, backdrop, false, blending_params);
			}
			return true
		};

		let mut process_pixel = |priority: u8, bg: u16, pixel_idx: usize, dest_pixel: &mut Pixel, maybe_line: Option<&GbaBGLine>,
				blending_params: &mut BlendingParams| {

			// First we draw the BG's pixel (if there is one)
			if let Some(bg_line) = maybe_line {
				let mut src_pixel = bg_line[pixel_idx];
				if window_clip_pixel(line, pixel_idx as u16, src_pixel, bg, dest_pixel, blending_params) {
					// #TODO might want to do the alpha = 0 check on the pixels here and remove it from the
					// on pixel drawn and blend_pixels functions in order to remove redundancy.
					on_pixel_drawn(bg, src_pixel, false, blending_params);
					*dest_pixel = Self::blend_pixels(src_pixel, *dest_pixel);
				}
			}

			// Then we draw the OBJ's pixel on top if there is one at this priority.
			let obj_priority = obj_info.get_priority(pixel_idx);
			if obj_enabled && obj_priority > 0 && (obj_priority - 1) == priority {
				let obj_pixel = obj_line[pixel_idx];
				if window_clip_pixel(line, pixel_idx as u16, obj_pixel, 4, dest_pixel, blending_params) {
					// #TODO might want to do the alpha = 0 check on the pixels here and remove it from the
					// on pixel drawn and blend_pixels functions in order to remove redundancy.
					on_pixel_drawn(4, obj_pixel, obj_info.is_transparent(pixel_idx), blending_params);
					*dest_pixel = Self::blend_pixels(obj_pixel, *dest_pixel);	
					blending_params.force_obj_blend |= obj_info.is_transparent(pixel_idx);
				}
			}
		};

		for pix in 0..240 {
			// #TODO I'm drawing the OBJ's pixel multiple times. Is there any way to not do this?
			//change_pixel_brightness(blend_evy, blend_evy, bg, src_pixel, &mut blending_params)

			let mut output_pixel = darkened_backdrop;

			process_pixel(rendering_order[0].0, rendering_order[0].1, pix, &mut output_pixel, rendering_order[0].2, &mut blending_params);
			process_pixel(rendering_order[1].0, rendering_order[1].1, pix, &mut output_pixel, rendering_order[1].2, &mut blending_params);
			process_pixel(rendering_order[2].0, rendering_order[2].1, pix, &mut output_pixel, rendering_order[2].2, &mut blending_params);
			process_pixel(rendering_order[3].0, rendering_order[3].1, pix, &mut output_pixel, rendering_order[3].2, &mut blending_params);

			if !blending_params.window_blending_disabled && (blend_mode == 1 || (blending_params.force_obj_blend && blend_mode > 0)) && (blending_params.target_drawn && !blending_params.target_overwritten && blending_params.source_on_top) {
				let (t_r, t_g, t_b) = expand_color(blending_params.target_pixel);
				let (s_r, s_g, s_b) = expand_color(blending_params.source_pixel);

				output[pix] = (
					min!(255, (((t_r as u16) * blend_evb) >> 4) + (((s_r as u16) * blend_eva) >> 4)) as u8,
					min!(255, (((t_g as u16) * blend_evb) >> 4) + (((s_g as u16) * blend_eva) >> 4)) as u8,
					min!(255, (((t_b as u16) * blend_evb) >> 4) + (((s_b as u16) * blend_eva) >> 4)) as u8,
				);
			} else {
				let expanded = expand_color(output_pixel); 
				if blending_params.source_on_top {
					output[pix] = change_pixel_brightness(blend_evy, expanded, &mut blending_params);
				} else {
					output[pix] = expanded;
				}
			}

			blending_params.target_drawn = false;
			blending_params.source_on_top = false;
			blending_params.target_overwritten = false;
			blending_params.force_obj_blend = false;
			blending_params.window_blending_disabled = false;
			blending_params.current_window_prio = 0; // reset it to winout.
		}
	}

	#[inline(always)]
	fn blend_pixels(a: Pixel, b: Pixel) -> Pixel {
		if is_transparent(a) { b }
		else { a }
	}
}

pub fn expand_color(rgb5: u16) -> (u8, u8, u8) {
	(
		(((rgb5 & 0x1f) * 527 + 23 ) >> 6) as u8,
		((((rgb5 >> 5) & 0x1f) * 527 + 23 ) >> 6) as u8,
		((((rgb5 >> 10) & 0x1f) * 527 + 23 ) >> 6) as u8
	)
}

/// Bit   Expl.
/// 0-4   Red Intensity   (0-31)
/// 5-9   Green Intensity (0-31)
/// 10-14 Blue Intensity  (0-31)
/// 15    Not used in GBA Mode (in NDS Mode: Alpha=0=Transparent, Alpha=1=Normal)
#[inline(always)]
pub fn convert_rgb5_to_rgb8(rgb5: u16) -> Pixel {
	// We use the NDS's system because it uses less memory than passing RGB8 everywhere.
	rgb5 | 0x8000 // Setting the alpha bit to 1
}

#[inline(always)]
pub fn is_transparent(pixel: Pixel) -> bool {
	(pixel & 0x8000) == 0
}

/// Bit   Expl.
/// 0-4   Red Intensity   (0-31)
/// 5-9   Green Intensity (0-31)
/// 10-14 Blue Intensity  (0-31)
/// 15    Not used in GBA Mode (in NDS Mode: Alpha=0=Transparent, Alpha=1=Normal)
#[inline(always)]
pub fn convert_rgb5_to_rgba8(rgb5: u16) -> u16 {
	// We use the NDS's system because it uses less memory than passing RGB8 everywhere.
	rgb5 | 0x8000 // Setting the alpha bit to 1
}

#[inline(always)]
fn window_contains(x: u16, y: u16, w_left: u16, w_right: u16, w_top: u16, w_bottom: u16) -> bool {
	// #TODO make this handle the cross pattern that occurs when w_right is less than w_left
	(x >= w_left) && (x < w_right) &&
	(y >= w_top) && (y < w_bottom)
}


// PIXEL BRIGHTNESS FUNCTIONS:

/// I1st + (31-I1st)*EVY
fn brighten_pixel(blend_evy: u16, color: GbaPixel) -> GbaPixel {
	(
		((color.0 as u16) + (((255 - (color.0 as u16)) * blend_evy) >> 4)) as u8,
		((color.1 as u16) + (((255 - (color.1 as u16)) * blend_evy) >> 4)) as u8,
		((color.2 as u16) + (((255 - (color.2 as u16)) * blend_evy) >> 4)) as u8
	)
}

/// I1st - (I1st)*EVY
fn darken_pixel(blend_evy: u16, color: GbaPixel) -> GbaPixel {
	(
		((color.0 as u16) - (((color.0 as u16) * blend_evy) >> 4)) as u8,
		((color.1 as u16) - (((color.1 as u16) * blend_evy) >> 4)) as u8,
		((color.2 as u16) - (((color.2 as u16) * blend_evy) >> 4)) as u8
	)
}

/// Just does nothing to the pixel
fn pixel_lum_nop(_: u16, color: GbaPixel) -> GbaPixel {
	color
}
