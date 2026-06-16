//! Skinshifter — `{1}{G}` 1/1 Human Shaman.
//! {G}: Choose one. Activate only once each turn.
//! • Until end of turn, this creature becomes a Rhino with base power and
//!   toughness 4/4 and gains trample.
//! • Until end of turn, this creature becomes a Bird with base power and
//!   toughness 2/2 and gains flying.
//! • Until end of turn, this creature becomes a Plant with base power and
//!   toughness 0/8.
//!
//! The {G} cost and once-each-turn restriction are wired. The modal
//! "choose one" body is GAP'd: modal mode-selection is only available on
//! spell abilities (dispatch_modal_effect / with_mode_effects), not on
//! activated abilities, so the three creature-form modes cannot be posted
//! as a player choice here.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skinshifter");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G}: Choose one. Activate only once each turn. • Becomes a 4/4 Rhino with trample. • Becomes a 2/2 Bird with flying. • Becomes a 0/8 Plant.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}").expect("valid cost"),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: choose_form_gap,
            }),
    )
}

fn choose_form_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: modal "choose one" on an activated ability is not expressible —
    // mode selection (dispatch_modal_effect / with_mode_effects) is
    // spell-ability only.
    Vec::new()
}
