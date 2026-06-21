//! Timestream Navigator — `{1}{U}` 1/1 Human Pirate Wizard.
//!
//! Oracle:
//! * Ascend (GAP — not a usable KeywordAbility; the city's-blessing
//!   mechanic is unmodeled).
//! * `{2}{U}{U}, {T}, Put this creature on the bottom of its owner's
//!   library: Take an extra turn after this one. Activate only if you have
//!   the city's blessing.
//!
//! The activated ability is wired with its mana+tap cost and the
//! extra-turn effect. Two cost/gate components are GAP'd: the
//! "put this on the bottom of its owner's library" additional cost (no
//! such ActivationCost field) and the "only if you have the city's
//! blessing" activation condition (no city's-blessing predicate).

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
    let name = reg.interner_mut().intern("Timestream Navigator");
    let human = reg.interner_mut().intern("Human");
    let pirate = reg.interner_mut().intern("Pirate");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(pirate);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Ascend — not in the usable KeywordAbility surface.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{U}{U}, {T}, Put this creature on the bottom of its owner's library: Take an extra turn after this one. Activate only if you have the city's blessing.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{U}{U}").expect("valid cost"),
                tap: true,
                // GAP: "Put this creature on the bottom of its owner's
                //       library" additional cost — no such ActivationCost field.
                // GAP: "Activate only if you have the city's blessing" — no
                //       city's-blessing activation_condition predicate.
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: take_extra_turn,
        }),
    )
}

fn take_extra_turn(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::ExtraTurn {
        player: ctx.controller,
    }]
}
