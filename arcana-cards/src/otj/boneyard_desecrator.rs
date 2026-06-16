//! Boneyard Desecrator — `{3}{B}` 3/4 Zombie Mercenary with Menace.
//! "{1}{B}, Sacrifice another creature: Put a +1/+1 counter on this
//!  creature. If an outlaw was sacrificed this way, create a Treasure
//!  token."
//!
//! Menace is a base keyword (the Treasure entry on the keyword line is
//! a mechanic marker, not a `KeywordAbility`). The activated ability
//! costs {1}{B} + sacrifice another creature (`sacrifice_other`) and
//! puts a +1/+1 counter on this creature. GAP: the conditional "if an
//! outlaw was sacrificed this way, create a Treasure token" rider can't
//! inspect which creature was sacrificed as the cost, so it is omitted.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Boneyard Desecrator");
    let zombie = reg.interner_mut().intern("Zombie");
    let mercenary = reg.interner_mut().intern("Mercenary");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(mercenary);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{B}, Sacrifice another creature: Put a +1/+1 counter on this creature.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{B}").expect("valid cost"),
                sacrifice_other: Some(ObjectFilter::creature()),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: add_counter,
        }),
    )
}

fn add_counter(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "if an outlaw was sacrificed this way, create a Treasure
    // token" — no accessor for the creature sacrificed as the cost.
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
