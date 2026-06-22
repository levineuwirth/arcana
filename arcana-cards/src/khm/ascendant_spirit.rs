//! Ascendant Spirit — `{U}` 1/1 Snow Creature — Spirit.
//! "{S}{S}: This creature becomes a Spirit Warrior with base power and
//! toughness 2/3."
//! "{S}{S}{S}: If this creature is a Warrior, put a flying counter on it and it
//! becomes a Spirit Warrior Angel with base power and toughness 4/4."
//! "{S}{S}{S}{S}: If this creature is an Angel, put two +1/+1 counters on it
//! and it gains 'Whenever this creature deals combat damage to a player, draw a
//! card.'"
//!
//! Ability 1 sets base P/T 2/3 (the "becomes Spirit Warrior" subtype add has
//! no AddSubtype primitive — GAP'd, but the P/T half is expressed).
//! Abilities 2 and 3 each gate on the creature already being a Warrior / an
//! Angel ("If this creature is a …"). There is no representable
//! activation-condition or Conditional value for "source has subtype", so
//! applying their counter/grant payloads unconditionally would be materially
//! wrong — both are GAP'd whole (their cost/zone shape is still recorded).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ascendant Spirit");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::SNOW),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{S}{S}: This creature becomes a Spirit Warrior with base power and toughness 2/3.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{S}{S}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_warrior,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{S}{S}{S}: If this creature is a Warrior, put a flying counter on it and it becomes a Spirit Warrior Angel with base power and toughness 4/4.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{S}{S}{S}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_angel,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{S}{S}{S}{S}: If this creature is an Angel, put two +1/+1 counters on it and it gains \"Whenever this creature deals combat damage to a player, draw a card.\"".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{S}{S}{S}{S}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: angel_upgrade,
            }),
    )
}

fn become_warrior(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "becomes a Spirit Warrior" subtype add has no AddSubtype primitive.
    // The base P/T 2/3 half is expressed.
    vec![Effect::SetBasePT {
        target: ctx.source,
        power: 2,
        toughness: 3,
        duration: Duration::Permanent,
    }]
}

fn become_angel(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: gated on "If this creature is a Warrior" — no representable
    // source-has-subtype condition, so the flying counter + 4/4 + subtype
    // change can't be applied conditionally.
    Vec::new()
}

fn angel_upgrade(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: gated on "If this creature is an Angel" — no representable
    // source-has-subtype condition, so the +1/+1 counters + granted ability
    // can't be applied conditionally.
    Vec::new()
}
