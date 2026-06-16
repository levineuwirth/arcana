//! Recruit Instructor — `{R}{W}` 1/1 Mouse Warrior.
//! "Whenever Recruit Instructor attacks, draft a card from Recruit
//! Instructor's spellbook. Valiant — Whenever Recruit Instructor becomes the
//! target of a spell or ability you control for the first time each turn,
//! create a 1/1 white Mouse creature token."
//!
//! Valiant is not a `KeywordAbility` variant (it's an ability-word label),
//! so no keyword line. The attacks→draft-from-spellbook trigger has no
//! effect surface (GAP'd). The Valiant becomes-target trigger is wired via
//! `SelfBecomesTarget { caster: You }` with `OncePerTurn` frequency for the
//! "first time each turn" clause, creating the Mouse token.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Recruit Instructor");
    let mouse = reg.interner_mut().intern("Mouse");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mouse);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: "Whenever ~ attacks, draft a card from ~'s spellbook" — no draft
    // effect surface.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesTarget {
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_mouse,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_mouse(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mouse = reg.interner().lookup("Mouse").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mouse);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: arcana_core::effects::TokenDefinition {
            name: mouse,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
