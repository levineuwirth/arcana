//! A-Futurist Operative — `{U}` 0/4 Creature — Human Ninja.
//! "As long as Futurist Operative is tapped, it's a Human Citizen with base
//! power and toughness 1/1 and can't be blocked."
//! "{2}{U}: Untap Futurist Operative."
//!
//! The tapped-state continuous ability (type/subtype change, base 1/1, and
//! can't-be-blocked while tapped) is a static conditional CDA with no
//! demonstrated primitive and is GAP'd. The {2}{U} untap activated ability
//! is wired.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Futurist Operative");
    let human = reg.interner_mut().intern("Human");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(ninja);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "As long as Futurist Operative is tapped, it's a Human Citizen
    // with base power and toughness 1/1 and can't be blocked." — a
    // tapped-state continuous self-modification (type/subtype set, base P/T
    // set, can't-be-blocked) with no demonstrated static-conditional CDA.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{U}: Untap Futurist Operative.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{U}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: untap_self,
        }),
    )
}

fn untap_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Untap { target: ctx.source }]
}
