//! Ahn-Crop Crasher — AKH uncommon. `{2}{R}` 3/2 Human Warrior with
//! Menace, plus a triggered ability that hobbles a chosen creature
//! for the turn (stubbed — see below). The seed's touchstone for
//! CR 702.110 Menace.
//!
//! # Scope compression
//!
//! The printed oracle includes a second ability: "Whenever Ahn-Crop
//! Crasher attacks, target creature can't block this turn." This is
//! a triggered ability that installs a continuous "can't block"
//! restriction until end of turn, expiring at cleanup like Prowess's
//! +1/+1 pump. The engine doesn't yet model "can't block" continuous
//! effects on creatures (CR 509.1c restrictions orthogonal to
//! blocker eligibility), so the ability is deferred.
//!
//! TODO(menace-seed): Ahn-Crop Crasher's "target creature can't
//! block this turn" trigger is stubbed. Land it when the engine
//! gains support for creature-level combat-restriction continuous
//! effects (same layer/duration shape as the Prowess pump or the
//! Snapcaster flashback grant, but applied to a targeted creature
//! rather than the source).
//!
//! # Rules references
//!
//! * CR 702.110a — Menace. "A creature with menace can't be blocked
//!   except by two or more creatures." Expressed in the engine via
//!   [`arcana_core::combat::AttackerBlockConstraints`] with
//!   `min_blockers = 2`; the enumerator then produces pair-and-larger
//!   subsets of eligible blockers and the apply-side drops
//!   declarations that violate the count constraint.
//!
//! # Engine wiring
//!
//! Menace rides on `base_characteristics.keywords` and dispatches
//! through the layer-aware
//! [`arcana_core::state::GameState::has_keyword`] lookup —
//! [`arcana_core::combat::GameState::block_constraints`] raises
//! `min_blockers` to 2 when it sees the keyword. No per-card effect
//! function is needed; the keyword is declarative.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ahn-Crop Crasher");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
