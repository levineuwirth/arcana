//! Aven Windreader — `{3}{U}{U}` 3/3 Bird Soldier Wizard.
//! Flying.
//! `{1}{U}: Target player reveals the top card of their library.`
//!
//! GAP: "Target player reveals the top card of their library" — there is
//! no Effect for a pure reveal of a fixed top card (no choice, no zone
//! change). The closest engine reveal primitives (DigTopN / RevealUntil)
//! move cards; emitting either would misrepresent the card. The activated
//! ability is wired with its cost + target, but its effect is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::effects::KeywordAbility;
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aven Windreader");
    let bird = reg.interner_mut().intern("Bird");
    let soldier = reg.interner_mut().intern("Soldier");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(soldier);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{U}: Target player reveals the top card of their library.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{U}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_player()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: reveal_top,
        }),
    )
}

fn reveal_top(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "reveals the top card of their library" — no fixed-top-card
    // reveal Effect (DigTopN / RevealUntil both move cards).
    Vec::new()
}
