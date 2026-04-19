//! Foundational types used throughout the engine.
//!
//! Addendum Section 2.1, Phase 1 Task #1. No dependencies on other engine
//! modules — pure data types with accompanying unit tests.

use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::ops::{BitAnd, BitOr, BitXor, Not};

// =============================================================================
// Numeric ID types
// =============================================================================

pub type PlayerId = u8;
pub type CardId = u32;
pub type AbilityId = u32;
pub type TriggerId = u32;
pub type ConditionId = u32;

// =============================================================================
// String interning
// =============================================================================
//
// `SmallString` is a `u32` handle into a [`StringInterner`]. The interner is
// an ordinary value type — typically owned by `CardRegistry`, threaded into
// registration APIs, and borrowed wherever names need to be resolved.
//
// We intentionally do *not* use a process-global interner: parallel
// self-play simulations share a single process, and any global mutable
// state becomes a coordination bottleneck (lock) or a source of subtle
// non-determinism (interning order leaking between threads). See
// `arcana_design_decisions.md` for the full rationale.
//
// Clone of a `SmallString` is a u32 copy; equality is integer comparison.
// Two `SmallString`s produced by *different* interners are meaningless to
// compare.

/// Interned string handle. Only meaningful relative to the [`StringInterner`]
/// that produced it.
pub type SmallString = u32;

#[derive(Clone, Debug, Default)]
pub struct StringInterner {
    strings: Vec<String>,
    index: HashMap<String, SmallString>,
}

impl StringInterner {
    pub fn new() -> Self { Self::default() }

    /// Intern a string, returning its handle. Idempotent: the same input
    /// returns the same handle within a single interner.
    pub fn intern(&mut self, s: &str) -> SmallString {
        if let Some(&id) = self.index.get(s) {
            return id;
        }
        let id = self.strings.len() as SmallString;
        self.strings.push(s.to_owned());
        self.index.insert(s.to_owned(), id);
        id
    }

    /// Look up a string's handle without interning it. Returns `None` if
    /// `s` has never been interned.
    pub fn lookup(&self, s: &str) -> Option<SmallString> {
        self.index.get(s).copied()
    }

    /// Resolve a handle back to its original text.
    pub fn resolve(&self, id: SmallString) -> Option<&str> {
        self.strings.get(id as usize).map(String::as_str)
    }

    pub fn len(&self) -> usize { self.strings.len() }
    pub fn is_empty(&self) -> bool { self.strings.is_empty() }
}

// =============================================================================
// PtValue — power / toughness
// =============================================================================

/// Power/toughness value. Supports numeric, `*` (CDA-defined), and `*+N`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PtValue {
    Fixed(i32),
    /// `*` — defined by a characteristic-defining ability or other effect.
    Star,
    /// `*+N` — e.g. `*+1`.
    StarPlus(i32),
}

impl PtValue {
    /// Resolve to a concrete integer given the value of `*` (from the CDA).
    /// Returns `None` if this is `Star` or `StarPlus` and `star_value` is
    /// `None`.
    pub fn resolve(&self, star_value: Option<i32>) -> Option<i32> {
        match self {
            PtValue::Fixed(n) => Some(*n),
            PtValue::Star => star_value,
            PtValue::StarPlus(n) => star_value.map(|s| s + n),
        }
    }
}

// =============================================================================
// Color / ManaColor — card-color-identity vs mana-unit-color
// =============================================================================
//
// These are structurally parallel enums, intentionally kept distinct to
// avoid conflating card color identity (which cannot be "colorless" — a
// colorless card has empty identity, not a sixth color) with mana units
// (which *can* be colorless, as in `{C}` produced by Eldrazi Temple).
//
// Conversion:
//   Color → ManaColor  via `From<Color>` / `Color::to_mana()` (always succeeds)
//   ManaColor → Color  via `ManaColor::as_color() -> Option<Color>`
//
// Note: this is a flat-parallel-enum layout rather than the nested
// `ManaColor::Color(Color)` form — same semantic guarantees, but lets
// pattern matching read `ManaColor::Red` instead of
// `ManaColor::Color(Color::Red)`.

/// One of the five MTG colors (WUBRG). Used for card color identity, spell
/// color requirements that can't be paid with colorless mana, filters like
/// "of the chosen color", etc. `ColorSet` is the set-of-`Color` bitset.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl Color {
    pub const fn letter(self) -> char {
        match self {
            Color::White => 'W',
            Color::Blue  => 'U',
            Color::Black => 'B',
            Color::Red   => 'R',
            Color::Green => 'G',
        }
    }

    /// All five colors in WUBRG order.
    pub const fn all() -> [Color; 5] {
        [Color::White, Color::Blue, Color::Black, Color::Red, Color::Green]
    }

    pub const fn to_mana(self) -> ManaColor {
        match self {
            Color::White => ManaColor::White,
            Color::Blue  => ManaColor::Blue,
            Color::Black => ManaColor::Black,
            Color::Red   => ManaColor::Red,
            Color::Green => ManaColor::Green,
        }
    }
}

impl From<Color> for ManaColor {
    fn from(c: Color) -> Self { c.to_mana() }
}

// =============================================================================
// ColorSet — 5-bit bitset over the five colors
// =============================================================================

/// Bitset of the 5 colors (WUBRG). 5 bits, packed into a u8.
///
/// By construction a `ColorSet` can never contain "colorless" — colorless
/// is simply the empty set (`is_colorless()` returns true when no bits are
/// set). Pass `Color` (not `ManaColor`) to `contains` / `with` / `without`
/// — using `ManaColor::Colorless` here would be a type error, which is the
/// point.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ColorSet(pub u8);

impl ColorSet {
    pub const WHITE: u8 = 0b00001;
    pub const BLUE:  u8 = 0b00010;
    pub const BLACK: u8 = 0b00100;
    pub const RED:   u8 = 0b01000;
    pub const GREEN: u8 = 0b10000;

    pub const fn new() -> Self { Self(0) }
    pub const fn white()     -> Self { Self(Self::WHITE) }
    pub const fn blue()      -> Self { Self(Self::BLUE) }
    pub const fn black()     -> Self { Self(Self::BLACK) }
    pub const fn red()       -> Self { Self(Self::RED) }
    pub const fn green()     -> Self { Self(Self::GREEN) }
    pub const fn colorless() -> Self { Self(0) }

    /// Bit mask for a given color.
    pub const fn mask(c: Color) -> u8 {
        match c {
            Color::White => Self::WHITE,
            Color::Blue  => Self::BLUE,
            Color::Black => Self::BLACK,
            Color::Red   => Self::RED,
            Color::Green => Self::GREEN,
        }
    }

    /// Does this set contain the given color?
    pub const fn contains(&self, c: Color) -> bool {
        self.0 & Self::mask(c) != 0
    }

    pub const fn with(self, c: Color) -> Self { Self(self.0 | Self::mask(c)) }
    pub const fn without(self, c: Color) -> Self { Self(self.0 & !Self::mask(c)) }

    /// Number of colors in this set (0..=5).
    pub const fn len(&self) -> u32 { self.0.count_ones() }

    /// Set with no colors.
    pub const fn is_colorless(&self) -> bool { self.0 == 0 }
    /// Exactly one color.
    pub const fn is_monocolor(&self) -> bool { self.0.count_ones() == 1 }
    /// Two or more colors.
    pub const fn is_multicolor(&self) -> bool { self.0.count_ones() > 1 }

    /// Iterate the colors present in this set in WUBRG order.
    pub fn iter(&self) -> impl Iterator<Item = Color> + '_ {
        Color::all().into_iter().filter(move |c| self.contains(*c))
    }
}

impl From<Color> for ColorSet {
    fn from(c: Color) -> Self { Self(Self::mask(c)) }
}

impl BitOr for ColorSet {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}
impl BitAnd for ColorSet {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}
impl BitXor for ColorSet {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}
impl Not for ColorSet {
    type Output = Self;
    fn not(self) -> Self { Self(!self.0 & 0b11111) }
}

// =============================================================================
// TypeLine — card type bitset
// =============================================================================

/// Card types as a bitset. An object can be multiple types at once (e.g. an
/// artifact creature, a legendary enchantment creature).
///
/// The public `const`s match the addendum so card definitions read like
/// `types: TypeLine::CREATURE.into()`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeLine(pub u16);

impl TypeLine {
    pub const CREATURE:     u16 = 1 << 0;
    pub const INSTANT:      u16 = 1 << 1;
    pub const SORCERY:      u16 = 1 << 2;
    pub const ENCHANTMENT:  u16 = 1 << 3;
    pub const ARTIFACT:     u16 = 1 << 4;
    pub const LAND:         u16 = 1 << 5;
    pub const PLANESWALKER: u16 = 1 << 6;
    pub const KINDRED:      u16 = 1 << 7; // formerly "Tribal"
    pub const BATTLE:       u16 = 1 << 8;

    const PERMANENT_MASK: u16 = Self::CREATURE | Self::ENCHANTMENT
        | Self::ARTIFACT | Self::LAND | Self::PLANESWALKER | Self::BATTLE;
    const SPELL_MASK: u16 = Self::INSTANT | Self::SORCERY;

    pub const fn new() -> Self { Self(0) }
    pub const fn with(self, t: u16) -> Self { Self(self.0 | t) }
    pub const fn without(self, t: u16) -> Self { Self(self.0 & !t) }

    /// True if any of the bits in `t` are set.
    pub const fn has(&self, t: u16) -> bool { self.0 & t != 0 }

    pub const fn is_creature(&self)     -> bool { self.has(Self::CREATURE) }
    pub const fn is_land(&self)         -> bool { self.has(Self::LAND) }
    pub const fn is_artifact(&self)     -> bool { self.has(Self::ARTIFACT) }
    pub const fn is_enchantment(&self)  -> bool { self.has(Self::ENCHANTMENT) }
    pub const fn is_planeswalker(&self) -> bool { self.has(Self::PLANESWALKER) }
    pub const fn is_battle(&self)       -> bool { self.has(Self::BATTLE) }
    pub const fn is_instant(&self)      -> bool { self.has(Self::INSTANT) }
    pub const fn is_sorcery(&self)      -> bool { self.has(Self::SORCERY) }

    /// True if this card can exist on the battlefield (CR 110.4).
    pub const fn is_permanent(&self) -> bool {
        self.0 & Self::PERMANENT_MASK != 0
    }
    /// True if this card is an instant or sorcery.
    pub const fn is_spell(&self) -> bool {
        self.0 & Self::SPELL_MASK != 0
    }
}

impl From<u16> for TypeLine {
    fn from(bits: u16) -> Self { Self(bits) }
}

impl BitOr for TypeLine {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}
impl BitAnd for TypeLine {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

// =============================================================================
// SupertypeSet — basic/legendary/snow/world
// =============================================================================

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SupertypeSet(pub u8);

impl SupertypeSet {
    pub const BASIC:     u8 = 1 << 0;
    pub const LEGENDARY: u8 = 1 << 1;
    pub const SNOW:      u8 = 1 << 2;
    pub const WORLD:     u8 = 1 << 3;

    pub const fn new() -> Self { Self(0) }
    pub const fn with(self, s: u8) -> Self { Self(self.0 | s) }
    pub const fn has(&self, s: u8) -> bool { self.0 & s != 0 }

    pub const fn is_basic(&self)     -> bool { self.has(Self::BASIC) }
    pub const fn is_legendary(&self) -> bool { self.has(Self::LEGENDARY) }
    pub const fn is_snow(&self)      -> bool { self.has(Self::SNOW) }
    pub const fn is_world(&self)     -> bool { self.has(Self::WORLD) }
}

impl From<u8> for SupertypeSet {
    fn from(bits: u8) -> Self { Self(bits) }
}

// =============================================================================
// SubtypeSet — open-ended set of interned subtype names
// =============================================================================
//
// Subtypes grow with every set (new creature types, land types, etc.), so we
// can't use a bitset. A `HashSet<SmallString>` is a good tradeoff: small per
// object, clone-friendly (SmallString is a u32 copy), and membership tests are
// O(1).

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubtypeSet(pub HashSet<SmallString>);

impl SubtypeSet {
    pub fn new() -> Self { Self(HashSet::default()) }

    /// Build a set from a list of subtype names, interning each into
    /// `interner`. Typically called at card-registration time.
    pub fn from_names<I, S>(interner: &mut StringInterner, names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut set = HashSet::default();
        for n in names {
            set.insert(interner.intern(n.as_ref()));
        }
        Self(set)
    }

    pub fn insert(&mut self, s: SmallString) -> bool { self.0.insert(s) }

    /// Intern `name` into `interner` and insert it. Returns whether the
    /// set actually changed (i.e. the name was not already present).
    pub fn insert_name(&mut self, interner: &mut StringInterner, name: &str) -> bool {
        self.0.insert(interner.intern(name))
    }

    pub fn remove(&mut self, s: SmallString) -> bool { self.0.remove(&s) }

    pub fn contains(&self, s: SmallString) -> bool { self.0.contains(&s) }

    /// Test membership by name without interning. A name that has never
    /// been interned can never be in the set, so we return `false` for
    /// unknown names rather than mutating the interner.
    pub fn contains_name(&self, interner: &StringInterner, name: &str) -> bool {
        interner.lookup(name).is_some_and(|id| self.0.contains(&id))
    }

    pub fn len(&self) -> usize { self.0.len() }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    pub fn iter(&self) -> impl Iterator<Item = SmallString> + '_ { self.0.iter().copied() }
}

// =============================================================================
// PermanentStatus — per-permanent boolean flags
// =============================================================================

/// Per-permanent boolean flags. These reset when a permanent enters a new
/// zone (handled in zones.rs / zone transitions).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PermanentStatus {
    pub tapped: bool,
    pub flipped: bool,
    pub face_down: bool,
    pub phased_out: bool,
    pub transformed: bool,
    pub monstrous: bool,
    pub renowned: bool,
    /// Entered this turn under current controller (CR 302.1).
    pub summoning_sick: bool,
}

// =============================================================================
// CounterKind + CounterMap
// =============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CounterKind {
    PlusOnePlusOne,
    MinusOneMinusOne,
    Loyalty,
    Charge,
    Time,
    Fade,
    Quest,
    Study,
    Poison,
    Energy, // technically on players, but tokens use this too
    Shield,
    Stun,
    Lore,
    Defense,
    /// Extensible for set-specific counters (e.g. "verse", "wish").
    Named(SmallString),
}

/// Map from `CounterKind` to count. Small and clone-friendly.
pub type CounterMap = HashMap<CounterKind, u32>;

// =============================================================================
// ManaColor
// =============================================================================

/// A unit of mana's color. Includes `Colorless` for mana produced by
/// lands like Wastes or spells with `{C}` in their mana-generation clause.
///
/// Use [`ManaColor::as_color`] to downcast when you need a [`Color`] and
/// want to handle the `Colorless` case explicitly.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ManaColor {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
}

impl ManaColor {
    /// Single-character WUBRG code. Colorless → 'C'.
    pub const fn letter(self) -> char {
        match self {
            ManaColor::White     => 'W',
            ManaColor::Blue      => 'U',
            ManaColor::Black     => 'B',
            ManaColor::Red       => 'R',
            ManaColor::Green     => 'G',
            ManaColor::Colorless => 'C',
        }
    }

    /// Downcast to `Color`. Returns `None` for `Colorless`.
    pub const fn as_color(self) -> Option<Color> {
        match self {
            ManaColor::White     => Some(Color::White),
            ManaColor::Blue      => Some(Color::Blue),
            ManaColor::Black     => Some(Color::Black),
            ManaColor::Red       => Some(Color::Red),
            ManaColor::Green     => Some(Color::Green),
            ManaColor::Colorless => None,
        }
    }

    pub const fn is_color(self) -> bool {
        !matches!(self, ManaColor::Colorless)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- String interner -----------------------------------------------------

    #[test]
    fn intern_is_idempotent() {
        let mut i = StringInterner::new();
        let a = i.intern("Goblin");
        let b = i.intern("Goblin");
        assert_eq!(a, b);
        assert_eq!(i.len(), 1);
    }

    #[test]
    fn intern_distinguishes_strings() {
        let mut i = StringInterner::new();
        let a = i.intern("Goblin");
        let b = i.intern("Bear");
        assert_ne!(a, b);
        assert_eq!(i.len(), 2);
    }

    #[test]
    fn resolve_roundtrip() {
        let mut i = StringInterner::new();
        let id = i.intern("Warrior");
        assert_eq!(i.resolve(id), Some("Warrior"));
    }

    #[test]
    fn resolve_unknown_returns_none() {
        let i = StringInterner::new();
        assert!(i.resolve(u32::MAX).is_none());
        assert!(i.resolve(0).is_none());
    }

    #[test]
    fn lookup_without_intern_does_not_add() {
        let mut i = StringInterner::new();
        assert!(i.lookup("Elf").is_none());
        assert_eq!(i.len(), 0);
        i.intern("Elf");
        assert_eq!(i.lookup("Elf"), Some(0));
        assert!(i.lookup("Goblin").is_none());
    }

    #[test]
    fn separate_interners_are_independent() {
        // Two interners produce their own id-spaces; same string in each
        // is guaranteed to map to some id, but not necessarily the same.
        let mut a = StringInterner::new();
        let mut b = StringInterner::new();
        let ia = a.intern("Zombie");
        let ib = b.intern("Zombie");
        // Both happen to be 0 here, but the point is we never rely on the
        // cross-interner equality — we resolve through the owning interner.
        assert_eq!(a.resolve(ia), Some("Zombie"));
        assert_eq!(b.resolve(ib), Some("Zombie"));
    }

    // --- PtValue -------------------------------------------------------------

    #[test]
    fn pt_value_resolves_fixed() {
        assert_eq!(PtValue::Fixed(3).resolve(None), Some(3));
        assert_eq!(PtValue::Fixed(3).resolve(Some(7)), Some(3));
    }

    #[test]
    fn pt_value_resolves_star() {
        assert_eq!(PtValue::Star.resolve(None), None);
        assert_eq!(PtValue::Star.resolve(Some(4)), Some(4));
        assert_eq!(PtValue::StarPlus(2).resolve(Some(4)), Some(6));
    }

    // --- ColorSet ------------------------------------------------------------

    #[test]
    fn colorset_colorless_default() {
        let cs = ColorSet::new();
        assert!(cs.is_colorless());
        assert!(!cs.is_monocolor());
        assert!(!cs.is_multicolor());
        assert_eq!(cs.len(), 0);
    }

    // (No `colorset_contains_colorless_is_false` test: `ColorSet::contains`
    //  takes `Color`, not `ManaColor`, so passing `ManaColor::Colorless` is
    //  now a type error at the call site — enforced by the compiler.)

    #[test]
    fn colorset_monocolor_detection() {
        let red = ColorSet::red();
        assert!(red.is_monocolor());
        assert!(!red.is_multicolor());
        assert!(!red.is_colorless());
        assert!(red.contains(Color::Red));
        assert!(!red.contains(Color::Blue));
    }

    #[test]
    fn colorset_multicolor() {
        let boros = ColorSet::red() | ColorSet::white();
        assert!(boros.is_multicolor());
        assert_eq!(boros.len(), 2);
        assert!(boros.contains(Color::Red));
        assert!(boros.contains(Color::White));
        assert!(!boros.contains(Color::Blue));
    }

    #[test]
    fn colorset_with_and_without() {
        let cs = ColorSet::new().with(Color::Blue).with(Color::Black);
        assert!(cs.contains(Color::Blue));
        assert!(cs.contains(Color::Black));
        let cs = cs.without(Color::Blue);
        assert!(!cs.contains(Color::Blue));
        assert!(cs.contains(Color::Black));
    }

    #[test]
    fn colorset_iter_yields_color_in_wubrg_order() {
        let gruul_azorius = ColorSet::green() | ColorSet::white() | ColorSet::blue();
        let colors: Vec<_> = gruul_azorius.iter().collect();
        // `iter` yields `Color`, not `ManaColor` — by design, a ColorSet
        // cannot yield `Colorless` because it can't contain it.
        assert_eq!(colors, vec![Color::White, Color::Blue, Color::Green]);
    }

    #[test]
    fn colorset_bitwise_ops() {
        let wu = ColorSet::white() | ColorSet::blue();
        let ub = ColorSet::blue() | ColorSet::black();
        let intersection = wu & ub;
        assert_eq!(intersection.len(), 1);
        assert!(intersection.contains(Color::Blue));

        let xor = wu ^ ub;
        assert_eq!(xor.len(), 2);
        assert!(xor.contains(Color::White));
        assert!(xor.contains(Color::Black));
        assert!(!xor.contains(Color::Blue));

        // Not masks to 5 bits so we never set phantom bits.
        let not_red = !ColorSet::red();
        assert!(!not_red.contains(Color::Red));
        assert_eq!(not_red.len(), 4);
        assert_eq!(not_red.0 & !0b11111, 0);
    }

    #[test]
    fn colorset_from_color() {
        let red: ColorSet = Color::Red.into();
        assert_eq!(red, ColorSet::red());
        assert!(red.contains(Color::Red));
        assert!(!red.contains(Color::Blue));
    }

    #[test]
    fn color_to_mana_roundtrip() {
        for c in Color::all() {
            assert_eq!(ManaColor::from(c).as_color(), Some(c));
        }
        assert_eq!(ManaColor::Colorless.as_color(), None);
    }

    // --- TypeLine ------------------------------------------------------------

    #[test]
    fn typeline_permanence() {
        assert!(TypeLine::from(TypeLine::CREATURE).is_permanent());
        assert!(TypeLine::from(TypeLine::LAND).is_permanent());
        assert!(TypeLine::from(TypeLine::PLANESWALKER).is_permanent());
        assert!(TypeLine::from(TypeLine::BATTLE).is_permanent());
        assert!(TypeLine::from(TypeLine::ENCHANTMENT).is_permanent());
        assert!(TypeLine::from(TypeLine::ARTIFACT).is_permanent());

        assert!(!TypeLine::from(TypeLine::INSTANT).is_permanent());
        assert!(!TypeLine::from(TypeLine::SORCERY).is_permanent());
    }

    #[test]
    fn typeline_spells_are_not_permanents() {
        let instant: TypeLine = TypeLine::INSTANT.into();
        assert!(instant.is_spell());
        assert!(instant.is_instant());
        assert!(!instant.is_permanent());
        assert!(!instant.is_creature());
    }

    #[test]
    fn typeline_multitype() {
        // Artifact creature
        let bits = TypeLine::CREATURE | TypeLine::ARTIFACT;
        let tl = TypeLine::from(bits);
        assert!(tl.is_creature());
        assert!(tl.is_artifact());
        assert!(tl.is_permanent());
        assert!(!tl.is_spell());
    }

    #[test]
    fn typeline_bitor_on_values() {
        let a = TypeLine::from(TypeLine::CREATURE);
        let b = TypeLine::from(TypeLine::ENCHANTMENT);
        let combined = a | b;
        assert!(combined.is_creature());
        assert!(combined.is_enchantment());
    }

    #[test]
    fn typeline_with_and_without() {
        let tl = TypeLine::new()
            .with(TypeLine::CREATURE)
            .with(TypeLine::ARTIFACT);
        assert!(tl.is_creature());
        assert!(tl.is_artifact());

        let tl = tl.without(TypeLine::ARTIFACT);
        assert!(tl.is_creature());
        assert!(!tl.is_artifact());
    }

    // --- SupertypeSet --------------------------------------------------------

    #[test]
    fn supertypeset_flags() {
        let st = SupertypeSet::new()
            .with(SupertypeSet::LEGENDARY)
            .with(SupertypeSet::SNOW);
        assert!(st.is_legendary());
        assert!(st.is_snow());
        assert!(!st.is_basic());
        assert!(!st.is_world());
    }

    #[test]
    fn supertypeset_default_empty() {
        let st = SupertypeSet::default();
        assert!(!st.is_basic());
        assert!(!st.is_legendary());
    }

    // --- SubtypeSet ----------------------------------------------------------

    #[test]
    fn subtypeset_insert_and_contains() {
        let mut interner = StringInterner::new();
        let mut st = SubtypeSet::new();
        assert!(st.is_empty());
        assert!(st.insert_name(&mut interner, "Beast"));
        assert!(!st.insert_name(&mut interner, "Beast")); // duplicate returns false
        assert_eq!(st.len(), 1);
        assert!(st.contains_name(&interner, "Beast"));
        assert!(!st.contains_name(&interner, "Horror"));
    }

    #[test]
    fn subtypeset_from_names() {
        let mut interner = StringInterner::new();
        let st = SubtypeSet::from_names(&mut interner, ["Beast", "Horror"]);
        assert_eq!(st.len(), 2);
        assert!(st.contains_name(&interner, "Beast"));
        assert!(st.contains_name(&interner, "Horror"));
    }

    #[test]
    fn subtypeset_remove() {
        let mut interner = StringInterner::new();
        let mut st = SubtypeSet::from_names(&mut interner, ["Human", "Warrior"]);
        let human_id = interner.intern("Human"); // already interned; returns existing id
        assert!(st.remove(human_id));
        assert!(!st.remove(human_id)); // removing again returns false
        assert!(!st.contains_name(&interner, "Human"));
        assert!(st.contains_name(&interner, "Warrior"));
    }

    #[test]
    fn subtypeset_contains_name_unknown_string_is_false() {
        // A name that has never been interned must return false without
        // mutating the interner.
        let interner = StringInterner::new();
        let st = SubtypeSet::new();
        assert!(!st.contains_name(&interner, "PhantomType"));
        assert!(interner.is_empty());
    }

    // --- PermanentStatus -----------------------------------------------------

    #[test]
    fn permanent_status_default_all_false() {
        let s = PermanentStatus::default();
        assert!(!s.tapped);
        assert!(!s.flipped);
        assert!(!s.face_down);
        assert!(!s.phased_out);
        assert!(!s.transformed);
        assert!(!s.monstrous);
        assert!(!s.renowned);
        assert!(!s.summoning_sick);
    }

    // --- CounterKind ---------------------------------------------------------

    #[test]
    fn counter_kind_named_is_interned() {
        let mut i = StringInterner::new();
        let a = CounterKind::Named(i.intern("verse"));
        let b = CounterKind::Named(i.intern("verse"));
        assert_eq!(a, b);

        let c = CounterKind::Named(i.intern("wish"));
        assert_ne!(a, c);
    }

    #[test]
    fn counter_map_works() {
        let mut m: CounterMap = CounterMap::new();
        m.insert(CounterKind::PlusOnePlusOne, 3);
        m.insert(CounterKind::Loyalty, 5);
        assert_eq!(m.get(&CounterKind::PlusOnePlusOne), Some(&3));
        assert_eq!(m.get(&CounterKind::Poison), None);
    }

    // --- ManaColor -----------------------------------------------------------

    #[test]
    fn manacolor_letters() {
        assert_eq!(ManaColor::White.letter(), 'W');
        assert_eq!(ManaColor::Blue.letter(), 'U');
        assert_eq!(ManaColor::Black.letter(), 'B');
        assert_eq!(ManaColor::Red.letter(), 'R');
        assert_eq!(ManaColor::Green.letter(), 'G');
        assert_eq!(ManaColor::Colorless.letter(), 'C');
    }

    #[test]
    fn manacolor_is_color() {
        assert!(ManaColor::Red.is_color());
        assert!(!ManaColor::Colorless.is_color());
    }
}
