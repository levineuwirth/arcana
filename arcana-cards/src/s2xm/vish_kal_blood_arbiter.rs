//! Vish Kal, Blood Arbiter — `{4}{W}{B}{B}` 5/5 Legendary Vampire with
//! Flying and Lifelink.
//! "Sacrifice a creature: Put X +1/+1 counters on Vish Kal, where X is the
//!  sacrificed creature's power." (X = sacrificed creature's power is not
//!  recoverable post-sacrifice.)
//! "Remove all +1/+1 counters from Vish Kal: Target creature gets -1/-1
//!  until end of turn for each +1/+1 counter removed this way." ("remove
//!  all" cost not expressible.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vish Kal, Blood Arbiter");
    let vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{B}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        ..Default::default()
    };

    // GAP: "Remove all +1/+1 counters from Vish Kal: Target creature gets -1/-1
    //       for each removed." — no "remove ALL counters" activation cost field,
    //       and the dynamic -1/-1 count cannot be expressed.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice a creature: Put X +1/+1 counters on Vish Kal, \
                       where X is the sacrificed creature's power."
                    .into(),
                cost: ActivationCost {
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::CREATURE.into()),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: sacrifice_for_counters,
            }),
    )
}

fn sacrifice_for_counters(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "X is the sacrificed creature's power" — the sacrificed creature is
    //      gone from the battlefield by resolution and the cost machinery
    //      exposes no accessor for its power, so the AddCounters amount cannot
    //      be computed.
    Vec::new()
}
