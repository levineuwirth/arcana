//! Justiciar's Portal — `{1}{W}` instant. "Exile target creature you
//! control, then return that card to the battlefield under its
//! owner's control. It gains first strike until end of turn." Blink
//! shape: ExilePermanent now + DelayedAction Return-from-exile at
//! NextEndStep would model 'until EOT' blinks; here the card returns
//! immediately, which doesn't quite match the resolver model — we
//! use DelayedAction with a NextEndStep return as the closest
//! catalog approximation, and grant first strike on the live target
//! before exile.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Justiciar's Portal");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target creature you control, then return that card to the battlefield under its owner's control. It gains first strike until end of turn.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: 'exile then immediately return' — modeled with
    // DelayedAction at NextEndStep (Cloudshift-class), the closest
    // catalog approximation. First-strike grant on the still-live
    // creature precedes the exile.
    vec![
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::FirstStrike,
            duration: Duration::EndOfTurn,
        },
        Effect::ExilePermanent { target: *id },
        Effect::DelayedAction {
            source: *id,
            controller: entry.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::ReturnFromExileToBattlefield,
        },
    ]
}
