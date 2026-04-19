//! Monastery Swiftspear — KTK common. `{R}` 1/2 Human Monk with
//! Haste and Prowess. The seed's touchstone for CR 702.108 Prowess
//! — "Whenever you cast a noncreature spell, this creature gets +1/+1
//! until end of turn." — and a regression anchor for
//! [`arcana_core::engine::apply_prowess_on_cast`].
//!
//! # Rules references
//!
//! * CR 702.108a — Prowess. Triggered static ability. The engine
//!   applies the pump directly on `SpellCast` events without
//!   routing through the stack, since the trigger has no agent
//!   choice (CR 603.2 short-circuit).
//! * CR 702.10b — Haste. Overrides summoning sickness. The engine
//!   already honors this via
//!   [`arcana_core::state::GameState::has_keyword`] in combat
//!   legality enumeration, which is why Swiftspear can attack on
//!   the turn it's cast.
//!
//! # Engine wiring
//!
//! Both keywords ride on `base_characteristics.keywords` — the
//! engine's SpellCast handler dispatches through
//! [`arcana_core::effects::KeywordAbility::Prowess`], and combat
//! legality reads [`arcana_core::effects::KeywordAbility::Haste`].
//! No per-card effect function is needed; the keywords are
//! declarative.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Monastery Swiftspear");
    let human = reg.interner_mut().intern("Human");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(monk);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Haste, KeywordAbility::Prowess],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
