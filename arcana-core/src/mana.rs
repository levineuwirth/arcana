//! Mana system: `ManaCost`, `ManaPool`, `ManaUnit`, `ManaRestrictions`,
//! and the payment solver.
//!
//! Addendum Section 2.5 / Listing 14, Phase 1 Tasks #2 and #12.
//!
//! The solver (Task #12) enumerates every valid `ManaPaymentPlan` for
//! a given `(cost, pool, spend-context)` triple. It's a two-tier
//! algorithm: a fast greedy path for "simple" costs (generic +
//! colored + colorless + snow), and a backtracking branch-enumerator
//! for hybrid / monohybrid / Phyrexian. Convoke, delve, and
//! alternative casts (flashback, foretell, kicker) slot in at the
//! `Action::CastSpell` layer — the caller decides *which* cost to
//! hand the solver; the solver stays context-minimal.

use serde::{Serialize, Deserialize};
use std::fmt;
use thiserror::Error;

use crate::types::{Color, ColorSet, ManaColor, SmallString, TypeLine};
use crate::objects::ObjectId;

// =============================================================================
// ManaCost + components
// =============================================================================

/// A mana cost as written on a card, e.g. `{1}{G}`, `{X}{R}{R}`, `{W/U}{B/P}`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManaCost {
    pub components: Vec<ManaCostComponent>,
}

/// One pip of a mana cost.
///
/// `Colorless` represents the `{C}` symbol — a requirement that must be
/// paid specifically with colorless mana. This is semantically distinct
/// from `Generic(1)`, which may be paid with any mana (including colored).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ManaCostComponent {
    Generic(u32),
    Colored(Color),
    /// `{C}` — requires a unit of colorless mana specifically.
    Colorless,
    Hybrid(Color, Color),
    PhyrexianColored(Color),
    PhyrexianHybrid(Color, Color),
    /// `{2/W}` — pay 2 generic, or 1 of the color.
    MonoHybrid(Color),
    Snow,
    X,
}

impl ManaCostComponent {
    /// Contribution to the card's printed mana value (CR 202.3).
    /// Monohybrid `{2/W}` contributes 2. X contributes 0 (the cast-time X
    /// value is handled by [`ManaCost::mana_value_with_x`]).
    pub const fn mana_value_contribution(&self) -> u32 {
        match self {
            ManaCostComponent::Generic(n)            => *n,
            ManaCostComponent::Colored(_)            => 1,
            ManaCostComponent::Colorless             => 1,
            ManaCostComponent::Hybrid(_, _)          => 1,
            ManaCostComponent::PhyrexianColored(_)   => 1,
            ManaCostComponent::PhyrexianHybrid(_, _) => 1,
            ManaCostComponent::MonoHybrid(_)         => 2,
            ManaCostComponent::Snow                  => 1,
            ManaCostComponent::X                     => 0,
        }
    }

    /// Colors this component potentially requires. Generic, Snow, X, and
    /// `{C}` (Colorless) all return an empty ColorSet.
    pub fn colors(&self) -> ColorSet {
        match self {
            ManaCostComponent::Generic(_)
                | ManaCostComponent::Colorless
                | ManaCostComponent::Snow
                | ManaCostComponent::X => ColorSet::new(),
            ManaCostComponent::Colored(c)
                | ManaCostComponent::PhyrexianColored(c)
                | ManaCostComponent::MonoHybrid(c) => ColorSet::from(*c),
            ManaCostComponent::Hybrid(a, b)
                | ManaCostComponent::PhyrexianHybrid(a, b)
                => ColorSet::from(*a) | ColorSet::from(*b),
        }
    }

    /// True for `Generic`, `Colored`, `Colorless`, and `Snow`. Enables the
    /// fast-path greedy mana solver (addendum Section 8) — anything else
    /// needs the backtracking solver because the payer has a real choice.
    pub const fn is_simple(&self) -> bool {
        matches!(
            self,
            ManaCostComponent::Generic(_)
            | ManaCostComponent::Colored(_)
            | ManaCostComponent::Colorless
            | ManaCostComponent::Snow
        )
    }
}

impl ManaCost {
    pub const fn empty() -> Self { Self { components: Vec::new() } }

    /// Parse a mana cost string like `"{1}{G}"` or `"{X}{W/U}{B/P}"`.
    /// An empty string parses to an empty cost (zero-mana spells).
    pub fn parse(s: &str) -> Result<Self, ManaCostParseError> {
        let mut components = Vec::new();
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if c.is_whitespace() { continue; }
            if c != '{' {
                return Err(ManaCostParseError::ExpectedOpenBrace(c));
            }
            let mut token = String::new();
            let mut closed = false;
            while let Some(&c) = chars.peek() {
                chars.next();
                if c == '}' { closed = true; break; }
                token.push(c);
            }
            if !closed {
                return Err(ManaCostParseError::UnterminatedBrace);
            }
            components.push(parse_component(&token)?);
        }
        Ok(ManaCost { components })
    }

    /// Printed mana value, counting `{X}` as 0 (CR 202.3e).
    pub fn mana_value(&self) -> u32 {
        self.components.iter().map(|c| c.mana_value_contribution()).sum()
    }

    /// Effective mana value when cast with a chosen X. Each `{X}` in the
    /// cost counts `x` once.
    pub fn mana_value_with_x(&self, x: u32) -> u32 {
        self.components.iter().map(|c| match c {
            ManaCostComponent::X => x,
            _ => c.mana_value_contribution(),
        }).sum()
    }

    /// Color identity contribution of this cost (CR 903.4).
    pub fn colors(&self) -> ColorSet {
        self.components.iter().fold(
            ColorSet::new(),
            |acc, c| acc | c.colors(),
        )
    }

    /// Number of `{X}` symbols (usually 0 or 1).
    pub fn x_count(&self) -> u32 {
        self.components.iter()
            .filter(|c| matches!(c, ManaCostComponent::X))
            .count() as u32
    }

    pub fn is_empty(&self) -> bool { self.components.is_empty() }
    pub fn len(&self) -> usize { self.components.len() }

    /// True if every component is simple (no hybrid / Phyrexian / X).
    /// Enables the fast-path greedy mana solver.
    pub fn is_simple(&self) -> bool {
        self.components.iter().all(|c| c.is_simple())
    }
}

impl fmt::Display for ManaCost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in &self.components {
            write!(f, "{c}")?;
        }
        Ok(())
    }
}

impl fmt::Display for ManaCostComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ManaCostComponent::Generic(n)            => write!(f, "{{{n}}}"),
            ManaCostComponent::Colored(c)            => write!(f, "{{{}}}", c.letter()),
            ManaCostComponent::Colorless             => f.write_str("{C}"),
            ManaCostComponent::Hybrid(a, b)          => write!(f, "{{{}/{}}}", a.letter(), b.letter()),
            ManaCostComponent::PhyrexianColored(c)   => write!(f, "{{{}/P}}", c.letter()),
            ManaCostComponent::PhyrexianHybrid(a, b) => write!(f, "{{{}/{}/P}}", a.letter(), b.letter()),
            ManaCostComponent::MonoHybrid(c)         => write!(f, "{{2/{}}}", c.letter()),
            ManaCostComponent::Snow                  => f.write_str("{S}"),
            ManaCostComponent::X                     => f.write_str("{X}"),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ManaCostParseError {
    #[error("expected '{{', found {0:?}")]
    ExpectedOpenBrace(char),
    #[error("unterminated '{{' — missing closing '}}'")]
    UnterminatedBrace,
    #[error("empty mana symbol '{{}}'")]
    EmptyToken,
    #[error("unknown mana symbol: {{{0}}}")]
    InvalidToken(String),
    #[error("unknown color code: {0:?}")]
    InvalidColor(String),
    /// `{N/W}` is a monohybrid, but only N=2 is valid in standard MTG.
    #[error("unsupported monohybrid numerator: {0} (only {{2/X}} is valid)")]
    InvalidMonoHybrid(u32),
}

fn parse_component(tok: &str) -> Result<ManaCostComponent, ManaCostParseError> {
    use ManaCostComponent::*;

    if tok.is_empty() {
        return Err(ManaCostParseError::EmptyToken);
    }

    // Pure number → Generic
    if let Ok(n) = tok.parse::<u32>() {
        return Ok(Generic(n));
    }

    // Single-symbol tokens
    if tok.len() == 1 {
        return match tok {
            "W" => Ok(Colored(Color::White)),
            "U" => Ok(Colored(Color::Blue)),
            "B" => Ok(Colored(Color::Black)),
            "R" => Ok(Colored(Color::Red)),
            "G" => Ok(Colored(Color::Green)),
            "C" => Ok(Colorless), // {C} is its own variant, not Colored(something)
            "S" => Ok(Snow),
            "X" => Ok(X),
            _ => Err(ManaCostParseError::InvalidToken(tok.to_string())),
        };
    }

    // Slash-separated: hybrid, Phyrexian, monohybrid
    if tok.contains('/') {
        let parts: Vec<&str> = tok.split('/').collect();
        return match parts.as_slice() {
            [a, "P"] => Ok(PhyrexianColored(parse_color(a)?)),
            [a, b, "P"] => Ok(PhyrexianHybrid(parse_color(a)?, parse_color(b)?)),
            [a, b] => {
                // Monohybrid {N/C} — only N=2 is standard in paper Magic.
                if let Ok(n) = a.parse::<u32>() {
                    if n == 2 { Ok(MonoHybrid(parse_color(b)?)) }
                    else { Err(ManaCostParseError::InvalidMonoHybrid(n)) }
                } else {
                    Ok(Hybrid(parse_color(a)?, parse_color(b)?))
                }
            }
            _ => Err(ManaCostParseError::InvalidToken(tok.to_string())),
        };
    }

    Err(ManaCostParseError::InvalidToken(tok.to_string()))
}

fn parse_color(s: &str) -> Result<Color, ManaCostParseError> {
    match s {
        "W" => Ok(Color::White),
        "U" => Ok(Color::Blue),
        "B" => Ok(Color::Black),
        "R" => Ok(Color::Red),
        "G" => Ok(Color::Green),
        // Note: {C} is not a valid leaf of a slash-separated symbol in
        // printed Magic (there's no {W/C} or {2/C}). Reject it here.
        _ => Err(ManaCostParseError::InvalidColor(s.to_string())),
    }
}

// =============================================================================
// SpendRestriction
// =============================================================================

/// A restriction on how a particular [`ManaUnit`] may be spent.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpendRestriction {
    OnlyCastCreatureSpells,
    OnlyCastSpellsOfColor(ColorSet),
    OnlyCastSpellsOfType(TypeLine),
    OnlyActivateAbilities,
    /// Arbitrary interned description, matched at payment time.
    Custom(SmallString),
}

// =============================================================================
// ManaPool, ManaUnit, ManaRestrictions
// =============================================================================

/// A single unit of mana in a player's pool.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManaUnit {
    pub color: ManaColor,
    /// Object that produced this mana (for "spend only for" restrictions).
    pub source: ObjectId,
    pub restrictions: ManaRestrictions,
}

impl ManaUnit {
    /// Convenience: an unrestricted mana unit of a color from an object.
    pub fn plain(color: ManaColor, source: ObjectId) -> Self {
        Self { color, source, restrictions: ManaRestrictions::default() }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManaRestrictions {
    /// e.g. "spend only to cast creature spells".
    pub spend_only_on: Option<SpendRestriction>,
    /// Snow mana (for snow-requiring costs like `{S}` or "snow mana" checks).
    pub is_snow: bool,
}

/// A player's mana pool. Cleared at the end of each phase (CR 106.4).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManaPool {
    pub pool: Vec<ManaUnit>,
}

impl ManaPool {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, unit: ManaUnit) { self.pool.push(unit); }

    /// Add `amount` plain (unrestricted) mana of `color` from `source`.
    pub fn add_mana(&mut self, color: ManaColor, amount: u32, source: ObjectId) {
        for _ in 0..amount {
            self.pool.push(ManaUnit::plain(color, source));
        }
    }

    pub fn clear(&mut self) { self.pool.clear(); }

    pub fn len(&self) -> usize { self.pool.len() }
    pub fn is_empty(&self) -> bool { self.pool.is_empty() }

    pub fn iter(&self) -> std::slice::Iter<'_, ManaUnit> { self.pool.iter() }

    /// Count mana of a specific color in the pool.
    pub fn count_color(&self, color: ManaColor) -> usize {
        self.pool.iter().filter(|u| u.color == color).count()
    }

    /// Count of unrestricted mana — ignores per-unit spend restrictions but
    /// not color. Convenience for simple affordability checks.
    pub fn count_unrestricted_color(&self, color: ManaColor) -> usize {
        self.pool.iter()
            .filter(|u| u.color == color && u.restrictions.spend_only_on.is_none())
            .count()
    }

    pub fn total(&self) -> usize { self.pool.len() }
}

// =============================================================================
// Payment solver — Section 8 / Listing 14
// =============================================================================

use crate::actions::{ManaAssignment, ManaPaymentPlan};

/// What the solver needs to evaluate [`SpendRestriction`]s on pool
/// units. Deliberately minimal — the solver should not need to reach
/// into `GameState` or `GameObject`.
///
/// The caller (usually the legal-action enumerator or the engine's
/// cast-spell procedure) builds this from the spell or ability being
/// paid for.
#[derive(Clone, Debug)]
pub struct SpendContext {
    pub casting_kind: CastingKind,
}

impl SpendContext {
    pub fn for_spell(types: TypeLine, colors: ColorSet) -> Self {
        Self { casting_kind: CastingKind::Spell { types, colors } }
    }
    pub fn for_activated_ability() -> Self {
        Self { casting_kind: CastingKind::ActivatedAbility }
    }
    pub fn unrestricted() -> Self { Self { casting_kind: CastingKind::Other } }
}

/// What sort of thing is being paid for. Determines which
/// [`SpendRestriction`]s are satisfied.
#[derive(Clone, Debug)]
pub enum CastingKind {
    Spell { types: TypeLine, colors: ColorSet },
    ActivatedAbility,
    /// Other paid events: e.g. escape into exile, cycling. Treat as
    /// "not a spell, not an ability" for restriction purposes.
    Other,
}

/// Does `restriction` permit spending a unit carrying it on this
/// cost?
fn restriction_permits(restriction: &SpendRestriction, ctx: &SpendContext) -> bool {
    use crate::mana::CastingKind::*;
    match (restriction, &ctx.casting_kind) {
        (SpendRestriction::OnlyCastCreatureSpells, Spell { types, .. }) => types.is_creature(),
        (SpendRestriction::OnlyCastSpellsOfColor(filter), Spell { colors, .. }) => {
            // The spell must contain at least one of the required colors.
            (colors.0 & filter.0) != 0
        }
        (SpendRestriction::OnlyCastSpellsOfType(filter), Spell { types, .. }) => {
            (types.0 & filter.0) != 0
        }
        (SpendRestriction::OnlyActivateAbilities, ActivatedAbility) => true,
        // TODO(restriction-custom): `Custom(tag)` is held by the unit but
        // has no matching slot on SpendContext yet. Treat as
        // non-permissive for now so we don't silently allow it.
        (SpendRestriction::Custom(_), _) => false,
        _ => false,
    }
}

/// Is this unit spendable toward this context at all (any cost role)?
fn unit_is_spendable(unit: &ManaUnit, ctx: &SpendContext) -> bool {
    match &unit.restrictions.spend_only_on {
        None => true,
        Some(r) => restriction_permits(r, ctx),
    }
}

/// Is this unit spendable toward a *colored* cost component? Matches
/// color; respects `{S}` as a color-free snow constraint.
fn unit_matches_specific(
    unit: &ManaUnit,
    required: SpecificPip,
    ctx: &SpendContext,
) -> bool {
    if !unit_is_spendable(unit, ctx) { return false; }
    match required {
        SpecificPip::Color(c) => unit.color == c,
        SpecificPip::Colorless => unit.color == ManaColor::Colorless,
        SpecificPip::Snow => unit.restrictions.is_snow,
    }
}

/// Internal: which specific-pip kind is required by a cost component?
#[derive(Clone, Copy, Debug)]
enum SpecificPip {
    Color(ManaColor),
    Colorless,
    Snow,
}

// --- Entry points -----------------------------------------------------------

/// Enumerate every valid, deduplicated [`ManaPaymentPlan`] for paying
/// `cost` from `pool` in `ctx`. `x_value` is required iff `cost`
/// contains `{X}` (the solver will panic otherwise — that's a caller
/// bug since X is chosen up-front in [`crate::actions::Action::CastSpell`]).
///
/// The empty cost returns a single trivial plan. An unpayable cost
/// returns an empty vector.
///
/// **Not** yet implemented: convoke, delve, alternative casts (those
/// are the caller's problem — flashback/foretell/kicker pass a
/// *different* cost to this function).
pub fn enumerate_payment_plans(
    cost: &ManaCost,
    pool: &ManaPool,
    x_value: Option<u32>,
    ctx: &SpendContext,
) -> Vec<ManaPaymentPlan> {
    let expanded = expand_x(cost, x_value);

    if expanded.is_empty() {
        return vec![ManaPaymentPlan::empty()];
    }

    if expanded.is_simple() {
        // Fast path: a single canonical plan (or none).
        return greedy_simple_pay(&expanded, pool, ctx)
            .into_iter()
            .collect();
    }

    // Full backtracking: enumerate specific-pip choices, accumulate
    // generic buckets as we go, then enumerate generic consumptions
    // at the leaf.
    let mut plans = Vec::new();
    let mut consumed = vec![false; pool.pool.len()];
    let mut plan = ManaPaymentPlan::empty();
    let mut generic_buckets: Vec<(usize, u32)> = Vec::new();
    backtrack_specific(
        &expanded, pool, ctx,
        &mut consumed, &mut plan, &mut generic_buckets,
        /*cost_idx=*/ 0, &mut plans,
    );

    dedup_plans_with_pool(&mut plans, pool);
    plans
}

/// Quick yes/no: is there at least one valid plan?
///
/// For simple costs this short-circuits to the fast path; for complex
/// costs it still enumerates (because correctness requires
/// backtracking — a purely-greedy yes/no can miss hybrid cases).
pub fn can_afford(
    cost: &ManaCost,
    pool: &ManaPool,
    x_value: Option<u32>,
    ctx: &SpendContext,
) -> bool {
    !enumerate_payment_plans(cost, pool, x_value, ctx).is_empty()
}

// --- Tier 1: greedy simple solver -------------------------------------------

/// Greedy solver for costs with only Generic / Colored / Colorless /
/// Snow pips. Pays specific pips first (most constrained), then
/// generic. Returns `None` if any specific pip has no match or
/// generic can't be covered.
fn greedy_simple_pay(
    cost: &ManaCost,
    pool: &ManaPool,
    ctx: &SpendContext,
) -> Option<ManaPaymentPlan> {
    let mut plan = ManaPaymentPlan::empty();
    let mut consumed = vec![false; pool.pool.len()];

    // Pass 1: specific pips.
    for (ci, comp) in cost.components.iter().enumerate() {
        let required = match comp {
            ManaCostComponent::Colored(c) => Some(SpecificPip::Color(c.to_mana())),
            ManaCostComponent::Colorless  => Some(SpecificPip::Colorless),
            ManaCostComponent::Snow       => Some(SpecificPip::Snow),
            ManaCostComponent::Generic(_) => None,
            _ => return None, // unreachable for is_simple costs
        };
        if let Some(req) = required {
            let pos = (0..pool.pool.len()).find(|&i|
                !consumed[i] && unit_matches_specific(&pool.pool[i], req, ctx))?;
            consumed[pos] = true;
            plan.assignments.push(ManaAssignment { pool_index: pos, cost_index: ci });
        }
    }

    // Pass 2: generic pips.
    for (ci, comp) in cost.components.iter().enumerate() {
        if let ManaCostComponent::Generic(n) = comp {
            for _ in 0..*n {
                let pos = (0..pool.pool.len()).find(|&i|
                    !consumed[i] && unit_is_spendable(&pool.pool[i], ctx))?;
                consumed[pos] = true;
                plan.assignments.push(ManaAssignment { pool_index: pos, cost_index: ci });
            }
        }
    }

    Some(plan)
}

// --- Tier 2: backtracking ---------------------------------------------------

/// Walk cost components left-to-right. For each choice-bearing
/// component, branch on the possible pays; for Generic-like
/// components, accumulate into the bucket list and recurse. At the
/// leaf, call [`distribute_generic`] to enumerate generic
/// consumption patterns.
#[allow(clippy::too_many_arguments)]
fn backtrack_specific(
    cost: &ManaCost,
    pool: &ManaPool,
    ctx: &SpendContext,
    consumed: &mut [bool],
    plan: &mut ManaPaymentPlan,
    generic_buckets: &mut Vec<(usize, u32)>,
    cost_idx: usize,
    plans: &mut Vec<ManaPaymentPlan>,
) {
    if cost_idx >= cost.components.len() {
        distribute_generic(pool, ctx, consumed, plan, generic_buckets, 0, plans);
        return;
    }

    // Helper closure: branch on paying the current cost_idx with a
    // unit matching `required`.
    //
    // Implemented inline since closures + &mut make borrowck unhappy.
    let comp = cost.components[cost_idx];
    match comp {
        ManaCostComponent::Generic(n) => {
            generic_buckets.push((cost_idx, n));
            backtrack_specific(cost, pool, ctx, consumed, plan,
                generic_buckets, cost_idx + 1, plans);
            generic_buckets.pop();
        }
        ManaCostComponent::Colored(c) => {
            branch_specific_pip(
                cost, pool, ctx, consumed, plan, generic_buckets,
                cost_idx, SpecificPip::Color(c.to_mana()), plans);
        }
        ManaCostComponent::Colorless => {
            branch_specific_pip(
                cost, pool, ctx, consumed, plan, generic_buckets,
                cost_idx, SpecificPip::Colorless, plans);
        }
        ManaCostComponent::Snow => {
            branch_specific_pip(
                cost, pool, ctx, consumed, plan, generic_buckets,
                cost_idx, SpecificPip::Snow, plans);
        }
        ManaCostComponent::Hybrid(a, b) => {
            // Two branches: pay a OR pay b.
            branch_specific_pip(
                cost, pool, ctx, consumed, plan, generic_buckets,
                cost_idx, SpecificPip::Color(a.to_mana()), plans);
            if a != b {
                branch_specific_pip(
                    cost, pool, ctx, consumed, plan, generic_buckets,
                    cost_idx, SpecificPip::Color(b.to_mana()), plans);
            }
        }
        ManaCostComponent::MonoHybrid(c) => {
            // Branch A: pay 2 generic.
            generic_buckets.push((cost_idx, 2));
            backtrack_specific(cost, pool, ctx, consumed, plan,
                generic_buckets, cost_idx + 1, plans);
            generic_buckets.pop();
            // Branch B: pay 1 of the color.
            branch_specific_pip(
                cost, pool, ctx, consumed, plan, generic_buckets,
                cost_idx, SpecificPip::Color(c.to_mana()), plans);
        }
        ManaCostComponent::PhyrexianColored(c) => {
            // Branch A: pay 1 of the color.
            branch_specific_pip(
                cost, pool, ctx, consumed, plan, generic_buckets,
                cost_idx, SpecificPip::Color(c.to_mana()), plans);
            // Branch B: pay 2 life.
            plan.phyrexian_life_payments.push(cost_idx);
            backtrack_specific(cost, pool, ctx, consumed, plan,
                generic_buckets, cost_idx + 1, plans);
            plan.phyrexian_life_payments.pop();
        }
        ManaCostComponent::PhyrexianHybrid(a, b) => {
            branch_specific_pip(
                cost, pool, ctx, consumed, plan, generic_buckets,
                cost_idx, SpecificPip::Color(a.to_mana()), plans);
            if a != b {
                branch_specific_pip(
                    cost, pool, ctx, consumed, plan, generic_buckets,
                    cost_idx, SpecificPip::Color(b.to_mana()), plans);
            }
            plan.phyrexian_life_payments.push(cost_idx);
            backtrack_specific(cost, pool, ctx, consumed, plan,
                generic_buckets, cost_idx + 1, plans);
            plan.phyrexian_life_payments.pop();
        }
        ManaCostComponent::X => {
            panic!("backtrack_specific: {{X}} should have been expanded");
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn branch_specific_pip(
    cost: &ManaCost,
    pool: &ManaPool,
    ctx: &SpendContext,
    consumed: &mut [bool],
    plan: &mut ManaPaymentPlan,
    generic_buckets: &mut Vec<(usize, u32)>,
    cost_idx: usize,
    required: SpecificPip,
    plans: &mut Vec<ManaPaymentPlan>,
) {
    for i in 0..pool.pool.len() {
        if consumed[i] { continue; }
        if !unit_matches_specific(&pool.pool[i], required, ctx) { continue; }

        consumed[i] = true;
        plan.assignments.push(ManaAssignment { pool_index: i, cost_index: cost_idx });

        backtrack_specific(cost, pool, ctx, consumed, plan,
            generic_buckets, cost_idx + 1, plans);

        plan.assignments.pop();
        consumed[i] = false;
    }
}

/// At the end of specific-pip backtracking, distribute remaining
/// usable units into the accumulated generic buckets. Each bucket is
/// a (cost_idx, need) pair; fill each bucket by choosing a size-`need`
/// combination of remaining units.
#[allow(clippy::too_many_arguments)]
fn distribute_generic(
    pool: &ManaPool,
    ctx: &SpendContext,
    consumed: &mut [bool],
    plan: &mut ManaPaymentPlan,
    buckets: &[(usize, u32)],
    bucket_idx: usize,
    plans: &mut Vec<ManaPaymentPlan>,
) {
    if bucket_idx >= buckets.len() {
        plans.push(plan.clone());
        return;
    }
    let (cost_idx, need) = buckets[bucket_idx];
    if need == 0 {
        distribute_generic(pool, ctx, consumed, plan, buckets, bucket_idx + 1, plans);
        return;
    }

    let usable: Vec<usize> = (0..pool.pool.len())
        .filter(|&i| !consumed[i] && unit_is_spendable(&pool.pool[i], ctx))
        .collect();

    if (usable.len() as u32) < need { return; }

    // Enumerate combinations of `need` indices from `usable`.
    let mut scratch = Vec::with_capacity(need as usize);
    enum_combinations(&usable, need as usize, 0, &mut scratch, &mut |combo| {
        for &i in combo {
            consumed[i] = true;
            plan.assignments.push(ManaAssignment { pool_index: i, cost_index: cost_idx });
        }
        distribute_generic(pool, ctx, consumed, plan, buckets, bucket_idx + 1, plans);
        for _ in 0..need {
            plan.assignments.pop();
        }
        for &i in combo {
            consumed[i] = false;
        }
    });
}

/// Recursive combination enumeration: all size-`k` subsets of `items`,
/// reported via `visit` with a borrowed slice.
fn enum_combinations(
    items: &[usize],
    k: usize,
    start: usize,
    scratch: &mut Vec<usize>,
    visit: &mut impl FnMut(&[usize]),
) {
    if scratch.len() == k {
        visit(scratch);
        return;
    }
    let remaining_needed = k - scratch.len();
    let end = items.len().saturating_sub(remaining_needed - 1);
    for i in start..end {
        scratch.push(items[i]);
        enum_combinations(items, k, i + 1, scratch, visit);
        scratch.pop();
    }
}

// --- X expansion ------------------------------------------------------------

fn expand_x(cost: &ManaCost, x_value: Option<u32>) -> ManaCost {
    if cost.x_count() == 0 {
        return cost.clone();
    }
    let x = x_value.unwrap_or_else(||
        panic!("mana solver: cost {cost} has {{X}} but no x_value supplied"));
    let components = cost.components.iter().map(|c| match c {
        ManaCostComponent::X => ManaCostComponent::Generic(x),
        other => *other,
    }).collect();
    ManaCost { components }
}

// --- Deduplication ----------------------------------------------------------

/// Canonical fingerprint for plan equivalence. Two plans hash equal
/// iff they consume pool units with the same `(color, is_snow)` tags
/// assigned to the same `cost_index`es, and they paid the same
/// `cost_index`es via Phyrexian life.
///
/// Unit restrictions are intentionally NOT part of the fingerprint:
/// if two units with different restrictions both pass
/// [`unit_is_spendable`] for this spend, they're fungible for *this*
/// cost. Future spends may distinguish them but the plan's outcome
/// is identical.
///
/// Convoke and delve lists are included because those assignments
/// are distinguishable (different tapped creature = different leftover
/// battlefield state). Both vectors are empty today but the
/// fingerprint will keep working when we add the features.
#[derive(Hash, PartialEq, Eq)]
struct PlanFingerprint {
    assignments: Vec<(usize, (ManaColor, bool))>,
    phyrexian_life: Vec<usize>,
    convoke: Vec<u32>,
    delve: Vec<u32>,
}

fn fingerprint(plan: &ManaPaymentPlan, pool: &ManaPool) -> PlanFingerprint {
    let mut assignments: Vec<(usize, (ManaColor, bool))> = plan.assignments.iter()
        .map(|a| {
            let u = &pool.pool[a.pool_index];
            (a.cost_index, (u.color, u.restrictions.is_snow))
        })
        .collect();
    // Within a cost_index the ordering of assignments doesn't matter
    // (Generic(2) paid R then G is the same plan as G then R).
    assignments.sort();

    let mut phyrexian_life = plan.phyrexian_life_payments.clone();
    phyrexian_life.sort();

    // Tokens: keep these ordered by id to match canonical form.
    let mut convoke: Vec<u32> = plan.convoke_creatures.iter().copied().collect();
    convoke.sort();
    let mut delve: Vec<u32> = plan.delve_cards.iter().copied().collect();
    delve.sort();

    PlanFingerprint { assignments, phyrexian_life, convoke, delve }
}

/// Drop functionally-equivalent plans from `plans` using
/// [`fingerprint`] as the equivalence key.
fn dedup_plans_with_pool(plans: &mut Vec<ManaPaymentPlan>, pool: &ManaPool) {
    if plans.len() <= 1 { return; }
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    plans.retain(|p| seen.insert(fingerprint(p, pool)));
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    // Color is used inside ManaCostComponent variants; ManaColor is used for
    // ManaPool / ManaUnit. Import both explicitly and qualify at call sites
    // to keep the distinction visible in test code.
    use crate::types::{Color, ManaColor};

    // --- Parsing -------------------------------------------------------------

    fn parse(s: &str) -> ManaCost {
        ManaCost::parse(s).unwrap_or_else(|e| panic!("failed to parse {s:?}: {e}"))
    }

    #[test]
    fn parse_empty_cost() {
        let c = parse("");
        assert!(c.is_empty());
        assert_eq!(c.mana_value(), 0);
    }

    #[test]
    fn parse_simple_cost() {
        let c = parse("{1}{G}");
        assert_eq!(c.components, vec![
            ManaCostComponent::Generic(1),
            ManaCostComponent::Colored(Color::Green),
        ]);
        assert_eq!(c.mana_value(), 2);
    }

    #[test]
    fn parse_pure_generic() {
        let c = parse("{3}");
        assert_eq!(c.components, vec![ManaCostComponent::Generic(3)]);
        assert_eq!(c.mana_value(), 3);
    }

    #[test]
    fn parse_zero() {
        let c = parse("{0}");
        assert_eq!(c.components, vec![ManaCostComponent::Generic(0)]);
        assert_eq!(c.mana_value(), 0);
    }

    #[test]
    fn parse_double_digit_generic() {
        let c = parse("{15}");
        assert_eq!(c.components, vec![ManaCostComponent::Generic(15)]);
    }

    #[test]
    fn parse_x_cost() {
        let c = parse("{X}{R}");
        assert_eq!(c.components, vec![
            ManaCostComponent::X,
            ManaCostComponent::Colored(Color::Red),
        ]);
        assert_eq!(c.mana_value(), 1); // X counts as 0 off-stack
        assert_eq!(c.mana_value_with_x(3), 4);
        assert_eq!(c.x_count(), 1);
    }

    #[test]
    fn parse_colorless_symbol() {
        // {C} is its own variant — distinct from both Colored(...) and
        // Generic(1). Must be paid with colorless mana specifically.
        let c = parse("{C}");
        assert_eq!(c.components, vec![ManaCostComponent::Colorless]);
        assert_eq!(c.mana_value(), 1);
        assert!(c.colors().is_colorless());
    }

    #[test]
    fn parse_colorless_vs_generic_one_are_distinct() {
        // {C} and {1} both contribute 1 to mana value, but they are NOT
        // equal components — they express different payment requirements.
        let colorless = parse("{C}");
        let generic_one = parse("{1}");
        assert_ne!(colorless.components, generic_one.components);
    }

    #[test]
    fn parse_hybrid() {
        let c = parse("{W/U}");
        assert_eq!(
            c.components,
            vec![ManaCostComponent::Hybrid(Color::White, Color::Blue)],
        );
        assert_eq!(c.mana_value(), 1);
        let colors = c.colors();
        assert!(colors.contains(Color::White));
        assert!(colors.contains(Color::Blue));
    }

    #[test]
    fn parse_phyrexian() {
        let c = parse("{W/P}");
        assert_eq!(
            c.components,
            vec![ManaCostComponent::PhyrexianColored(Color::White)],
        );
    }

    #[test]
    fn parse_phyrexian_hybrid() {
        let c = parse("{B/R/P}");
        assert_eq!(
            c.components,
            vec![ManaCostComponent::PhyrexianHybrid(Color::Black, Color::Red)],
        );
    }

    #[test]
    fn parse_monohybrid() {
        let c = parse("{2/W}");
        assert_eq!(c.components, vec![ManaCostComponent::MonoHybrid(Color::White)]);
        assert_eq!(c.mana_value(), 2);
    }

    #[test]
    fn parse_snow() {
        let c = parse("{S}{2}");
        assert_eq!(c.components, vec![
            ManaCostComponent::Snow,
            ManaCostComponent::Generic(2),
        ]);
        assert_eq!(c.mana_value(), 3);
    }

    #[test]
    fn parse_complex_cost() {
        // "Reaper King"-ish: {2/W}{2/U}{2/B}{2/R}{2/G}
        let c = parse("{2/W}{2/U}{2/B}{2/R}{2/G}");
        assert_eq!(c.components.len(), 5);
        assert_eq!(c.mana_value(), 10);
        assert_eq!(c.colors().len(), 5);
    }

    #[test]
    fn parse_whitespace_is_tolerated() {
        let c = parse("  {1} {G}  ");
        assert_eq!(c.components, vec![
            ManaCostComponent::Generic(1),
            ManaCostComponent::Colored(Color::Green),
        ]);
    }

    // --- Parse errors --------------------------------------------------------

    #[test]
    fn parse_error_missing_brace() {
        let err = ManaCost::parse("W").unwrap_err();
        assert_eq!(err, ManaCostParseError::ExpectedOpenBrace('W'));
    }

    #[test]
    fn parse_error_unterminated() {
        let err = ManaCost::parse("{W").unwrap_err();
        assert_eq!(err, ManaCostParseError::UnterminatedBrace);
    }

    #[test]
    fn parse_error_empty_symbol() {
        let err = ManaCost::parse("{}").unwrap_err();
        assert_eq!(err, ManaCostParseError::EmptyToken);
    }

    #[test]
    fn parse_error_invalid_token() {
        let err = ManaCost::parse("{Q}").unwrap_err();
        assert_eq!(err, ManaCostParseError::InvalidToken("Q".into()));
    }

    #[test]
    fn parse_error_bad_monohybrid() {
        // {3/W} is not a thing in paper Magic.
        let err = ManaCost::parse("{3/W}").unwrap_err();
        assert_eq!(err, ManaCostParseError::InvalidMonoHybrid(3));
    }

    // --- Display roundtrip ---------------------------------------------------

    #[test]
    fn display_roundtrip() {
        for s in &[
            "",
            "{0}",
            "{1}{G}",
            "{X}{R}{R}",
            "{W/U}{W/U}",
            "{2/W}{2/U}",
            "{W/P}",
            "{B/R/P}",
            "{S}{S}{2}",
            "{C}",
            "{15}",
        ] {
            let parsed = ManaCost::parse(s).expect(s);
            let displayed = parsed.to_string();
            assert_eq!(displayed, *s, "roundtrip failed for {s:?}");
        }
    }

    // --- Queries -------------------------------------------------------------

    #[test]
    fn mana_value_ignores_x_by_default() {
        assert_eq!(parse("{X}{X}{R}").mana_value(), 1);
    }

    #[test]
    fn mana_value_with_x_multiplies() {
        assert_eq!(parse("{X}{X}{R}").mana_value_with_x(3), 7);
    }

    #[test]
    fn colors_of_generic_only() {
        assert!(parse("{3}").colors().is_colorless());
    }

    #[test]
    fn colors_of_multicolor_spell() {
        let c = parse("{1}{W}{U}{B}");
        let colors = c.colors();
        assert_eq!(colors.len(), 3);
        assert!(colors.contains(Color::White));
        assert!(colors.contains(Color::Blue));
        assert!(colors.contains(Color::Black));
    }

    #[test]
    fn is_simple_detection() {
        assert!(parse("{1}{G}").is_simple());
        assert!(parse("{3}").is_simple());
        assert!(parse("{S}{1}").is_simple());
        assert!(!parse("{X}{R}").is_simple());
        assert!(!parse("{W/U}").is_simple());
        assert!(!parse("{2/W}").is_simple());
        assert!(!parse("{W/P}").is_simple());
    }

    // --- ManaPool ------------------------------------------------------------

    #[test]
    fn manapool_starts_empty() {
        let p = ManaPool::new();
        assert!(p.is_empty());
        assert_eq!(p.len(), 0);
    }

    #[test]
    fn manapool_add_and_count() {
        let mut p = ManaPool::new();
        let src = 42;
        p.add_mana(ManaColor::Red, 3, src);
        p.add_mana(ManaColor::Blue, 2, src);
        assert_eq!(p.len(), 5);
        assert_eq!(p.count_color(ManaColor::Red), 3);
        assert_eq!(p.count_color(ManaColor::Blue), 2);
        assert_eq!(p.count_color(ManaColor::Green), 0);
    }

    #[test]
    fn manapool_clear_empties() {
        let mut p = ManaPool::new();
        p.add_mana(ManaColor::Red, 3, 0);
        p.clear();
        assert!(p.is_empty());
    }

    #[test]
    fn manapool_holds_colorless_mana() {
        // Colorless mana (e.g. from Wastes or Eldrazi Temple) counts toward
        // colorless-producing pools without contaminating color counts.
        let mut p = ManaPool::new();
        p.add_mana(ManaColor::Colorless, 2, 0);
        p.add_mana(ManaColor::Red, 1, 0);
        assert_eq!(p.count_color(ManaColor::Colorless), 2);
        assert_eq!(p.count_color(ManaColor::Red), 1);
    }

    #[test]
    fn manapool_restricted_vs_unrestricted_count() {
        let mut p = ManaPool::new();
        p.add(ManaUnit::plain(ManaColor::Red, 0));
        p.add(ManaUnit {
            color: ManaColor::Red,
            source: 0,
            restrictions: ManaRestrictions {
                spend_only_on: Some(SpendRestriction::OnlyCastCreatureSpells),
                is_snow: false,
            },
        });
        assert_eq!(p.count_color(ManaColor::Red), 2);
        assert_eq!(p.count_unrestricted_color(ManaColor::Red), 1);
    }

    // =========================================================================
    // Payment solver (Task #12)
    // =========================================================================

    fn pool_of(colors: &[ManaColor]) -> ManaPool {
        let mut p = ManaPool::new();
        for c in colors { p.add(ManaUnit::plain(*c, 0)); }
        p
    }

    fn nonspell_ctx() -> SpendContext { SpendContext::unrestricted() }

    fn creature_spell_ctx() -> SpendContext {
        SpendContext::for_spell(TypeLine::CREATURE.into(), ColorSet::green())
    }

    fn instant_spell_ctx() -> SpendContext {
        SpendContext::for_spell(TypeLine::INSTANT.into(), ColorSet::red())
    }

    fn solve(cost: &str, pool: &ManaPool, ctx: &SpendContext) -> Vec<ManaPaymentPlan> {
        enumerate_payment_plans(&parse(cost), pool, None, ctx)
    }

    fn solve_x(cost: &str, x: u32, pool: &ManaPool, ctx: &SpendContext)
        -> Vec<ManaPaymentPlan>
    {
        enumerate_payment_plans(&parse(cost), pool, Some(x), ctx)
    }

    // --- Empty and affordability edge cases ---------------------------------

    #[test]
    fn empty_cost_yields_one_trivial_plan() {
        let plans = solve("", &ManaPool::new(), &nonspell_ctx());
        assert_eq!(plans.len(), 1);
        assert!(plans[0].assignments.is_empty());
    }

    #[test]
    fn zero_generic_needs_no_mana() {
        let plans = solve("{0}", &ManaPool::new(), &nonspell_ctx());
        assert_eq!(plans.len(), 1);
        assert!(plans[0].assignments.is_empty());
    }

    #[test]
    fn unpayable_simple_cost_returns_empty() {
        let pool = pool_of(&[ManaColor::Red]);
        let plans = solve("{G}", &pool, &nonspell_ctx());
        assert!(plans.is_empty());
        assert!(!can_afford(&parse("{G}"), &pool, None, &nonspell_ctx()));
    }

    // --- Simple costs (greedy fast path) ------------------------------------

    #[test]
    fn simple_cost_pays_colored_first_then_generic() {
        let pool = pool_of(&[ManaColor::Green, ManaColor::Red]);
        let plans = solve("{1}{G}", &pool, &nonspell_ctx());
        assert_eq!(plans.len(), 1);
        let plan = &plans[0];
        assert_eq!(plan.assignments.len(), 2);
        // Green unit must be assigned to the {G} cost index (1).
        let green_assign = plan.assignments.iter().find(|a| a.cost_index == 1).unwrap();
        assert_eq!(pool.pool[green_assign.pool_index].color, ManaColor::Green);
        let gen_assign = plan.assignments.iter().find(|a| a.cost_index == 0).unwrap();
        assert_eq!(pool.pool[gen_assign.pool_index].color, ManaColor::Red);
    }

    #[test]
    fn simple_cost_distinguishes_colorless_from_generic_one() {
        // {C} requires a colorless unit specifically. {1} accepts anything.
        let all_red = pool_of(&[ManaColor::Red]);
        assert!(solve("{C}", &all_red, &nonspell_ctx()).is_empty());
        assert_eq!(solve("{1}", &all_red, &nonspell_ctx()).len(), 1);

        let with_c = pool_of(&[ManaColor::Colorless]);
        assert_eq!(solve("{C}", &with_c, &nonspell_ctx()).len(), 1);
    }

    #[test]
    fn simple_generic_with_any_leftover() {
        // {2} with 2 Mountains and 1 Forest — greedy picks first 2 units;
        // after dedup (no dedup needed since no branching), 1 plan.
        let pool = pool_of(&[ManaColor::Red, ManaColor::Red, ManaColor::Green]);
        let plans = solve("{2}", &pool, &nonspell_ctx());
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].assignments.len(), 2);
    }

    #[test]
    fn simple_ignores_insufficient_pool() {
        let pool = pool_of(&[ManaColor::Red]);
        assert!(solve("{2}", &pool, &nonspell_ctx()).is_empty());
    }

    // --- Snow ---------------------------------------------------------------

    #[test]
    fn snow_cost_requires_snow_mana() {
        let mut plain = ManaPool::new();
        plain.add(ManaUnit::plain(ManaColor::Red, 0));
        assert!(solve("{S}", &plain, &nonspell_ctx()).is_empty());

        let mut snow = ManaPool::new();
        snow.add(ManaUnit {
            color: ManaColor::Red,
            source: 0,
            restrictions: ManaRestrictions { spend_only_on: None, is_snow: true },
        });
        assert_eq!(solve("{S}", &snow, &nonspell_ctx()).len(), 1);
    }

    // --- Hybrid -------------------------------------------------------------

    #[test]
    fn hybrid_enumerates_both_color_choices() {
        let pool = pool_of(&[ManaColor::White, ManaColor::Blue]);
        let plans = solve("{W/U}", &pool, &nonspell_ctx());
        // Two plans: pay with W, or pay with U.
        assert_eq!(plans.len(), 2);
    }

    #[test]
    fn hybrid_with_only_one_color_available_yields_one_plan() {
        let pool = pool_of(&[ManaColor::White]);
        let plans = solve("{W/U}", &pool, &nonspell_ctx());
        assert_eq!(plans.len(), 1);
    }

    #[test]
    fn hybrid_same_color_deduplicates() {
        // {W/W} (hypothetical) — a=b means only one branch, one plan.
        // We don't have such a cost in practice; simulate by composing
        // cost manually.
        let cost = ManaCost {
            components: vec![ManaCostComponent::Hybrid(Color::White, Color::White)],
        };
        let pool = pool_of(&[ManaColor::White, ManaColor::White]);
        let plans = enumerate_payment_plans(&cost, &pool, None, &nonspell_ctx());
        // Two Whites are fungible for a single pip → 1 plan post-dedup.
        assert_eq!(plans.len(), 1);
    }

    // --- Monohybrid ---------------------------------------------------------

    #[test]
    fn monohybrid_has_two_branches() {
        // {2/W}: with enough W for both branches.
        let pool = pool_of(&[ManaColor::White, ManaColor::Red, ManaColor::Green]);
        let plans = solve("{2/W}", &pool, &nonspell_ctx());
        // Branch A (pay 2 generic): pick any 2 of the 3 units.
        //   C(3,2) = 3 raw combos; after class-dedup by (ManaColor, is_snow),
        //   distinct multisets of size 2 are: {W,R}, {W,G}, {R,G} → 3 plans.
        // Branch B (pay 1 W): 1 plan.
        // Total: 4 plans.
        assert_eq!(plans.len(), 4);
    }

    #[test]
    fn monohybrid_insufficient_for_2_generic_falls_to_color() {
        // Only 1 White, no other mana.
        let pool = pool_of(&[ManaColor::White]);
        let plans = solve("{2/W}", &pool, &nonspell_ctx());
        // Branch A needs 2 total generic — can't. Branch B pays 1 W — can.
        assert_eq!(plans.len(), 1);
    }

    // --- Phyrexian ----------------------------------------------------------

    #[test]
    fn phyrexian_colored_with_color_and_without() {
        // {W/P} with 1 White: 2 plans (pay W; pay 2 life).
        let pool = pool_of(&[ManaColor::White]);
        let plans = solve("{W/P}", &pool, &nonspell_ctx());
        assert_eq!(plans.len(), 2);
        assert!(plans.iter().any(|p| !p.phyrexian_life_payments.is_empty()));
        assert!(plans.iter().any(|p| p.phyrexian_life_payments.is_empty()));
    }

    #[test]
    fn phyrexian_colored_with_no_color_only_life() {
        let pool = ManaPool::new();
        let plans = solve("{W/P}", &pool, &nonspell_ctx());
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].phyrexian_life_payments, vec![0]);
    }

    #[test]
    fn phyrexian_hybrid_three_branches() {
        // {B/R/P} with B, R, and life available: 3 plans.
        let pool = pool_of(&[ManaColor::Black, ManaColor::Red]);
        let plans = solve("{B/R/P}", &pool, &nonspell_ctx());
        assert_eq!(plans.len(), 3);
    }

    // --- X expansion --------------------------------------------------------

    #[test]
    fn x_expansion_makes_it_generic() {
        let pool = pool_of(&[ManaColor::Red, ManaColor::Red, ManaColor::Red]);
        let plans = solve_x("{X}{R}", 2, &pool, &nonspell_ctx());
        // {X}{R} with X=2 → {2}{R}. Simple cost (is_simple after expansion),
        // 3 Rs available — unique greedy plan.
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].assignments.len(), 3);
    }

    #[test]
    fn x_zero_is_legal() {
        let pool = pool_of(&[ManaColor::Red]);
        let plans = solve_x("{X}{R}", 0, &pool, &nonspell_ctx());
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].assignments.len(), 1);
    }

    #[test]
    #[should_panic(expected = "no x_value supplied")]
    fn x_without_value_panics() {
        let pool = pool_of(&[ManaColor::Red]);
        let _ = solve("{X}{R}", &pool, &nonspell_ctx());
    }

    // --- Restrictions -------------------------------------------------------

    #[test]
    fn only_cast_creature_spells_rejects_instant() {
        let mut pool = ManaPool::new();
        pool.add(ManaUnit {
            color: ManaColor::Red,
            source: 0,
            restrictions: ManaRestrictions {
                spend_only_on: Some(SpendRestriction::OnlyCastCreatureSpells),
                is_snow: false,
            },
        });
        // Cast an instant — forbidden.
        assert!(solve("{R}", &pool, &instant_spell_ctx()).is_empty());
        // Cast a creature spell of the color — allowed.
        let ctx = SpendContext::for_spell(TypeLine::CREATURE.into(), ColorSet::red());
        assert_eq!(solve("{R}", &pool, &ctx).len(), 1);
    }

    #[test]
    fn only_activate_abilities_rejects_spells() {
        let mut pool = ManaPool::new();
        pool.add(ManaUnit {
            color: ManaColor::Red,
            source: 0,
            restrictions: ManaRestrictions {
                spend_only_on: Some(SpendRestriction::OnlyActivateAbilities),
                is_snow: false,
            },
        });
        assert!(solve("{R}", &pool, &creature_spell_ctx()).is_empty());
        assert_eq!(solve("{R}", &pool, &SpendContext::for_activated_ability()).len(), 1);
    }

    // --- Deduplication ------------------------------------------------------

    #[test]
    fn three_mountains_paying_r_is_one_plan() {
        let pool = pool_of(&[ManaColor::Red, ManaColor::Red, ManaColor::Red]);
        // {W/R} has two branches but W is unavailable; only R branch survives.
        // With 3 interchangeable Rs, the one-R-pay dedups to a single plan.
        let plans = solve("{W/R}", &pool, &nonspell_ctx());
        assert_eq!(plans.len(), 1);
    }

    #[test]
    fn hybrid_dedup_does_not_collapse_distinct_colors() {
        // Two different-color units paying a hybrid cost — each branch is
        // a distinct plan (consumed unit class differs).
        let pool = pool_of(&[ManaColor::White, ManaColor::Blue]);
        let plans = solve("{W/U}", &pool, &nonspell_ctx());
        assert_eq!(plans.len(), 2);
    }

    #[test]
    fn monohybrid_dedup_collapses_same_color_pairs() {
        // {2/W} with 3 Whites:
        //   Branch A (2 generic): all 3 Ws fungible → 1 plan.
        //   Branch B (1 W): 1 plan.
        // Total 2 plans (not C(3,2)=3 from branch A).
        let pool = pool_of(&[ManaColor::White, ManaColor::White, ManaColor::White]);
        let plans = solve("{2/W}", &pool, &nonspell_ctx());
        assert_eq!(plans.len(), 2);
    }

    // --- can_afford --------------------------------------------------------

    #[test]
    fn can_afford_shortcuts_happy_path() {
        let pool = pool_of(&[ManaColor::Red, ManaColor::Red]);
        assert!(can_afford(&parse("{1}{R}"), &pool, None, &nonspell_ctx()));
        assert!(!can_afford(&parse("{R}{R}{R}"), &pool, None, &nonspell_ctx()));
    }

    // --- Complex realistic cost --------------------------------------------

    #[test]
    fn saw_it_coming_mana_cost_pays_with_three_islands() {
        // Saw It Coming (KHM): mana cost {3}{U}. Foretell would be a
        // different cost supplied by the caller; the solver sees only
        // the specific cost for this cast.
        let pool = pool_of(&[
            ManaColor::Blue, ManaColor::Blue, ManaColor::Blue, ManaColor::Blue,
        ]);
        let plans = solve("{3}{U}", &pool, &instant_spell_ctx());
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].assignments.len(), 4);
    }

    #[test]
    fn reaper_king_ish_cost_solves() {
        // {2/W}{2/U}{2/B}{2/R}{2/G} — every component monohybrid.
        // With one of each color + 5 colorless, many branches — we
        // just verify it doesn't explode and yields >= 1 plan.
        let mut pool = ManaPool::new();
        for c in [
            ManaColor::White, ManaColor::Blue, ManaColor::Black,
            ManaColor::Red, ManaColor::Green,
        ] {
            pool.add(ManaUnit::plain(c, 0));
        }
        for _ in 0..5 { pool.add(ManaUnit::plain(ManaColor::Colorless, 0)); }
        let plans = solve("{2/W}{2/U}{2/B}{2/R}{2/G}", &pool, &nonspell_ctx());
        assert!(!plans.is_empty(), "should find at least one plan");
    }
}
