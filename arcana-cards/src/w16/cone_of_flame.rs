//! Cone of Flame — `{3}{R}{R}` sorcery. "Cone of Flame deals 1
//! damage to any target, 2 damage to another target, and 3 damage to
//! a third target."

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
    let name = reg.interner_mut().intern("Cone of Flame");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Cone of Flame deals 1 damage to any target, 2 damage to another target, and 3 damage to a third target.".into(),
            target_requirements: vec![
                TargetRequirement::any_target(),
                TargetRequirement::any_target(),
                TargetRequirement::any_target(),
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn to_dt(t: &TargetChoice) -> Option<DamageTarget> {
    Some(match t {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    })
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let ts = &entry.targets.targets;
    let mut effects = Vec::new();
    for (i, amount) in [1u32, 2, 3].into_iter().enumerate() {
        if let Some(dt) = ts.get(i).and_then(to_dt) {
            effects.push(Effect::DealDamage {
                source: entry.source,
                target: dt,
                amount,
            });
        }
    }
    effects
}
