//! First Stage of Magic Design — `{W}{U}{B}{R}{G}` instant. "You gain
//! 3 life. Draw three cards. Add {B}{B}{B}. This spell deals 3 damage
//! to any target. Target creature gets +3/+3 until end of turn."
//!
//! The mana-addition ("Add {B}{B}{B}") is not expressible as a
//! resolution Effect in the catalog; the rest is emitted.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("First Stage of Magic Design");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "You gain 3 life. Draw three cards. Add {B}{B}{B}. This spell deals 3 damage to any target. Target creature gets +3/+3 until end of turn.".into(),
            target_requirements: vec![
                TargetRequirement::any_target(),
                TargetRequirement::target_creature(),
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = vec![
        Effect::GainLife { player: entry.controller, amount: 3 },
        Effect::DrawCards { player: entry.controller, count: 3 },
    ];
    // GAP: "Add {B}{B}{B}" — mana addition is not a catalog Effect.
    if let Some(TargetChoice::ObjectOrPlayer(o)) = entry.targets.targets.first() {
        let dt = match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        };
        effects.push(Effect::DealDamage { source: entry.source, target: dt, amount: 3 });
    } else if let Some(target) = entry.targets.targets.first() {
        let dt = match target {
            TargetChoice::Object(id) => DamageTarget::Object(*id),
            TargetChoice::Player(p) => DamageTarget::Player(*p),
            TargetChoice::ObjectOrPlayer(o) => match o {
                ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
                ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
            },
        };
        effects.push(Effect::DealDamage { source: entry.source, target: dt, amount: 3 });
    }
    if let Some(TargetChoice::Object(id)) = entry.targets.targets.get(1) {
        effects.push(Effect::Pump {
            target: *id,
            power: 3,
            toughness: 3,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        });
    }
    effects
}
