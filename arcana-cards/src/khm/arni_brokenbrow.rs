//! Arni Brokenbrow — `{2}{R}` 3/3 Legendary Human Berserker with Haste.
//! "Boast — {1}: You may change Arni's base power to 1 plus the greatest
//! power among other creatures you control until end of turn. (Activate
//! only if this creature attacked this turn and only once each turn.)"
//!
//! The Boast ability is wired as an activated ability: cost {1}, gated
//! to "attacked this turn" via activation_condition and limited to one
//! activation per turn via once_per_turn.
//!
//! GAP (effect): the new base power is "1 plus the greatest power among
//! other creatures you control" — a resolution-time amount with no
//! script:: helper for "greatest power among matching creatures", so the
//! Effect::SetBasePT value cannot be computed and the resolver returns no
//! effect.

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arni Brokenbrow");
    let human = reg.interner_mut().intern("Human");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(berserker);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Boast — {1}: You may change Arni's base power to 1 plus the greatest power among other creatures you control until end of turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                activation_condition: Some(attacked_this_turn),
                once_per_turn: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: boast_set_power,
        }),
    )
}

fn attacked_this_turn(s: &GameState, src: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::source_attacked_this_turn(s, src)
}

fn boast_set_power(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: new base power = 1 + greatest power among OTHER creatures you
    // control; no script:: helper computes a max power over a filter, so
    // the SetBasePT amount is uncomputable.
    Vec::new()
}
