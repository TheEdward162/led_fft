use crate::DataType;

pub struct TopColorCounter {
	slot: [u8; 3]
}
impl TopColorCounter {
	pub const fn new() -> Self {
		TopColorCounter { slot: [0; 3] }
	}

	pub const fn slots(&self) -> &[u8; 3] {
		&self.slot
	}

	pub fn mixes(&self) -> [DataType; 3] {
		let sum = self.slot[0] as DataType + self.slot[1] as DataType + self.slot[2] as DataType;
		[
			self.slot[0] as DataType / sum,
			self.slot[1] as DataType / sum,
			self.slot[2] as DataType / sum
		]
	}

	pub fn update(&mut self, color: [u8; 3]) -> [crate::DataType; 3] {
		macro_rules! update_color {
			(
				$updated: expr,
				$other_a: expr, $other_b: expr
			) => {
				if color[$updated] > color[$other_a] && color[$updated] > color[$other_b] {
					self.slot[$updated] = self.slot[$updated].saturating_add(1);
				} else if color[$updated] > color[$other_a] || color[$updated] > color[$other_b] {
					// pass
				} else {
					self.slot[$updated] = self.slot[$updated].saturating_sub(1);
					}
			};
		}

		update_color!(0, 1, 2);
		update_color!(1, 0, 2);
		update_color!(2, 1, 0);

		self.mixes()
	}
}
