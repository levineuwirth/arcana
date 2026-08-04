//! Drownyard Lurker — `{7}` 7/7 Creature — Eldrazi Trilobite.
//!
//! Vigilance.
//! When you cast or cycle Drownyard Lurker, create a 0/1 colorless
//! Eldrazi Spawn creature token with "Sacrifice this token: Add {C}."
//! Cycling {2}{U}.
//!
//! Wired here: Vigilance keyword; Cycling {2}{U} keyword (the engine
//! synthesizes the discard-to-draw ability). The cast-or-cycle ETB-ish
//! trigger fires "when you cast" via SpellCast on this card — we model
//! the "when you cast" half as a self-spell-cast trigger creating the
//! token. (The token's "Sacrifice this token: Add {C}" intrinsic
//! activation is not expressible on a hand-rolled TokenDefinition — see
//! GAP below.)

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Drownyard Lurker");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let trilobite = reg.interner_mut().intern("Trilobite");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(trilobite);

    // Pre-intern the token's subtype names.
    let _spawn = reg.interner_mut().intern("Eldrazi");
    let _spawn2 = reg.interner_mut().intern("Spawn");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![
            KeywordAbility::Vigilance,
            KeywordAbility::Cycling(ManaCost::parse("{2}{U}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "When you cast ... Drownyard Lurker, create a token."
            // (The "or cycle" half fires off a different event that is
            // not separately expressible as a trigger condition here.)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_spawn_token,
                trigger_zones: vec![Zone::Stack],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_spawn_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let spawn = reg.interner().lookup("Spawn").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    if let Some(eldrazi) = reg.interner().lookup("Eldrazi") {
        subtypes.0.insert(eldrazi);
    }
    subtypes.0.insert(spawn);
    // GAP: token's intrinsic "Sacrifice this token: Add {C}" activated
    // ability cannot be attached to a hand-rolled TokenDefinition with
    // the demonstrated API (abilities: vec![] only).
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: spawn,
            colors: ColorSet::colorless(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
