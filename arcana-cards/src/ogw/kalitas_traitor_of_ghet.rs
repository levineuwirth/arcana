//! Kalitas, Traitor of Ghet — `{2}{B}{B}` 3/4 Legendary Vampire Warrior.
//!
//! Oracle:
//! * Lifelink.
//! * "If a nontoken creature an opponent controls would die, instead
//!   exile that card and create a 2/2 black Zombie creature token." —
//!   GAP: a continuous death-replacement static; no triggered/activated
//!   form expressible with the available catalog.
//! * "{2}{B}, Sacrifice another Vampire or Zombie: Put two +1/+1
//!   counters on Kalitas." — a mana + sacrifice-other activation.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kalitas, Traitor of Ghet");
    let vampire = reg.interner_mut().intern("Vampire");
    let warrior = reg.interner_mut().intern("Warrior");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    // "Sacrifice another Vampire or Zombie" — a subtype-OR filter over
    // creatures you control (the engine excludes Kalitas itself).
    let sac_filter = ObjectFilter::creature().with_subtypes_any(vec![vampire, zombie]);

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{B}, Sacrifice another Vampire or Zombie: Put two +1/+1 counters on Kalitas."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                sacrifice_other: Some(sac_filter),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: add_two_counters,
        }),
    )
}

fn add_two_counters(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 2,
    }]
}
