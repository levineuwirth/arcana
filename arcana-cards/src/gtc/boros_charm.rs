//! Boros Charm — `{R}{W}` modal instant. "Choose one — Boros Charm deals 4
//! damage to target player or planeswalker; or permanents you control gain
//! indestructible until end of turn; or target creature gains double strike
//! until end of turn." (GAPs: mode 1 indestructible-all needs a controller-
//! scoped grant not yet available — no-op if chosen; mode 0 targets a player
//! only, planeswalker option elided.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    CardDefinition, CardRegistry, ModalSpec, ModeClause, SpellAbilityDef,
};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Boros Charm");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
        text: "Choose one — Boros Charm deals 4 damage to target player or planeswalker; or permanents you control gain indestructible until end of turn; or target creature gains double strike until end of turn.".into(),
        target_requirements: vec![],
        modal: Some(ModalSpec {
            min_modes: 1,
            max_modes: 1,
            clauses: vec![
                ModeClause {
                    text: "Boros Charm deals 4 damage to target player.".into(),
                    target_requirements: vec![TargetRequirement::target_player()],
                },
                ModeClause {
                    text: "Permanents you control gain indestructible until end of turn.".into(),
                    target_requirements: vec![],
                },
                ModeClause {
                    text: "Target creature gains double strike until end of turn.".into(),
                    target_requirements: vec![TargetRequirement::target_creature()],
                },
            ],
        }),
        effect: resolve,
    }))
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(choice) = entry.modes.first() else { return Vec::new(); };
    let Some(&mode) = choice.mode_indices.first() else { return Vec::new(); };
    match mode {
        0 => match entry.targets.targets.first() {
            Some(TargetChoice::Player(p)) => vec![Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Player(*p),
                amount: 4,
            }],
            _ => Vec::new(),
        },
        1 => Vec::new(), // GAP: indestructible-to-all-your-permanents
        2 => match entry.targets.targets.first() {
            Some(TargetChoice::Object(id)) => vec![Effect::InstallContinuousEffect {
                effect: ContinuousEffect::grant_keyword(
                    entry.source,
                    *id,
                    KeywordAbility::DoubleStrike,
                    Duration::EndOfTurn,
                ),
            }],
            _ => Vec::new(),
        },
        _ => Vec::new(),
    }
}
