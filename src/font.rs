mod mono_13;
mod mono_16;
mod mono_20;
mod mono_24;
mod sans_13;
mod sans_16;
mod sans_20;
mod sans_24;

pub use mono_13::FONT as MONO_13;
pub use mono_16::FONT as MONO_16;
pub use mono_20::FONT as MONO_20;
pub use mono_24::FONT as MONO_24;
pub use sans_13::FONT as SANS_13;
pub use sans_16::FONT as SANS_16;
pub use sans_20::FONT as SANS_20;
pub use sans_24::FONT as SANS_24;

pub fn char_to_idx(ch: char) -> Option<usize> {
	match ch {
		' '..='~' => Some(ch as u32 as usize - ' ' as u32 as usize),
		'ᴇ' => Some(0x5f),
		'∞' => Some(0x60),
		'×' => Some(0x61),
		'÷' => Some(0x62),
		'±' => Some(0x63),
		'°' => Some(0x64),
		'∀' => Some(0x65),
		'∅' => Some(0x66),
		'∈' => Some(0x67),
		'∉' => Some(0x68),
		'∙' => Some(0x69),
		'∫' => Some(0x6a),
		'≈' => Some(0x6b),
		'≤' => Some(0x6c),
		'≥' => Some(0x6d),
		'⋂' => Some(0x6e),
		'⋃' => Some(0x6f),
		'←' => Some(0x70),
		'↑' => Some(0x71),
		'→' => Some(0x72),
		'↓' => Some(0x73),
		'↵' => Some(0x74),
		'⬏' => Some(0x75),
		'α' => Some(0x76),
		'β' => Some(0x77),
		'Γ' => Some(0x78),
		'γ' => Some(0x79),
		'Δ' => Some(0x7a),
		'δ' => Some(0x7b),
		'ϵ' => Some(0x7c),
		'ϝ' => Some(0x7d),
		'ζ' => Some(0x7e),
		'η' => Some(0x7f),
		'Θ' => Some(0x80),
		'θ' => Some(0x81),
		'ι' => Some(0x82),
		'κ' => Some(0x83),
		'Λ' => Some(0x84),
		'λ' => Some(0x85),
		'μ' => Some(0x86),
		'ν' => Some(0x87),
		'Ξ' => Some(0x88),
		'ξ' => Some(0x89),
		'Π' => Some(0x8a),
		'π' => Some(0x8b),
		'ρ' => Some(0x8c),
		'Σ' => Some(0x8d),
		'σ' => Some(0x8e),
		'τ' => Some(0x8f),
		'υ' => Some(0x90),
		'Φ' => Some(0x91),
		'ϕ' => Some(0x92),
		'χ' => Some(0x93),
		'Ψ' => Some(0x94),
		'ψ' => Some(0x95),
		'Ω' => Some(0x96),
		'ω' => Some(0x97),
		'…' => Some(0x98),
		'▪' => Some(0x99),
		'◂' => Some(0x9a),
		'▴' => Some(0x9b),
		'▸' => Some(0x9c),
		'▾' => Some(0x9d),
		'≠' => Some(0x9e),
		'≷' => Some(0x9f),
		'∡' => Some(0xa0),
		'²' => Some(0xa1),
		'³' => Some(0xa2),
		'ˣ' => Some(0xa3),
		'₂' => Some(0xa4),
		'ℹ' => Some(0xa5),
		'⟪' => Some(0xa6),
		'⟫' => Some(0xa7),
		_ => None,
	}
}
