//! Punish the Enemy — `{4}{R}` instant. Deals 3 damage to target player
//! or planeswalker; 3 damage to target creature.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, ObjectOrPlayer, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Punish the Enemy");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Punish the Enemy deals 3 damage to target player or planeswalker and 3 damage to target creature.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::AnyTarget,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement::target_creature(),
                ],
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
    let targets = &entry.targets.targets;
    let mut effects = Vec::new();
    if let Some(t) = targets.first() {
        let dt = match t {
            TargetChoice::Player(p) => DamageTarget::Player(*p),
            TargetChoice::Object(id) => DamageTarget::Object(*id),
            TargetChoice::ObjectOrPlayer(o) => match o {
                ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
                ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
            },
        };
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: dt,
            amount: 3,
        });
    }
    if let Some(TargetChoice::Object(b)) = targets.get(1) {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*b),
            amount: 3,
        });
    }
    let _ = ObjectFilter::creature();
    effects
}
