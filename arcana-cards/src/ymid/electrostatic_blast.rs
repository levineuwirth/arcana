//! Electrostatic Blast — `{1}{R}` instant, "Electrostatic Blast deals 2 damage to any target.
//! You get a one-time boon with 'When you cast an instant or sorcery spell, exile the top three
//! cards of your library. You may play one of those cards until end of turn.'"
//!
//! GAP: No engine effect for creating a one-time triggered boon (emblem/effect that triggers
//! on casting instant/sorcery, exiling top N cards and allowing play of one until end of turn).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Electrostatic Blast");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Electrostatic Blast deals 2 damage to any target. You get a one-time boon with \"When you cast an instant or sorcery spell, exile the top three cards of your library. You may play one of those cards until end of turn.\"".into(),
                target_requirements: vec![TargetRequirement::any_target()],
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
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    // GAP: No engine effect for creating a one-time triggered boon (cast-trigger with exile+play)
    vec![Effect::DealDamage {
        source: entry.source,
        target: dt,
        amount: 2,
    }]
}
