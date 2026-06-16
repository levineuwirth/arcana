//! Etherium-Horn Sorcerer — `{4}{U}{R}` 3/6 Artifact Creature — Minotaur Wizard
//! Sorcerer.
//!
//! {1}{U}{R}: Return this creature to its owner's hand.
//! Cascade (When you cast this spell, exile cards from the top of your library
//! until you exile a nonland card that costs less. …)
//!
//! The bounce activated ability is wired. Cascade is a cast-trigger keyword not
//! in the usable surface and is not cleanly self-filterable here, so it is GAP'd.

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
    let name = reg.interner_mut().intern("Etherium-Horn Sorcerer");
    let minotaur = reg.interner_mut().intern("Minotaur");
    let wizard = reg.interner_mut().intern("Wizard");
    let sorcerer = reg.interner_mut().intern("Sorcerer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(minotaur);
    subtypes.0.insert(wizard);
    subtypes.0.insert(sorcerer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    // GAP: Cascade keyword/cast-trigger — not in the usable keyword surface and
    // not self-filterable via the SpellCast condition.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{U}{R}: Return this creature to its owner's hand.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{U}{R}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: bounce_self,
        }),
    )
}

fn bounce_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ReturnToHand { target: ctx.source }]
}
