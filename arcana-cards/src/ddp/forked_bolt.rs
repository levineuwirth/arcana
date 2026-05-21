//! Forked Bolt — `{R}` sorcery. "Forked Bolt deals 2 damage divided
//! as you choose among one or two targets."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectOrPlayer, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Forked Bolt");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Forked Bolt deals 2 damage divided as you choose among one or two targets.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::AnyTarget,
                count: TargetCount::UpTo(2),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn to_damage(target: &TargetChoice) -> DamageTarget {
    match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    }
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let targets = &entry.targets.targets;
    // "2 damage divided" — no damage-division primitive exists. With
    // two targets, deal 1 to each (the only even, deterministic
    // split); with one target, deal all 2.
    match targets.len() {
        0 => Vec::new(),
        1 => vec![Effect::DealDamage {
            source: entry.source,
            target: to_damage(&targets[0]),
            amount: 2,
        }],
        _ => vec![
            Effect::DealDamage {
                source: entry.source,
                target: to_damage(&targets[0]),
                amount: 1,
            },
            Effect::DealDamage {
                source: entry.source,
                target: to_damage(&targets[1]),
                amount: 1,
            },
        ],
    }
    // GAP: free "divide as you choose" allocation of the 2 damage is
    // not expressible; a fixed 1/1 split is used for two targets.
}
