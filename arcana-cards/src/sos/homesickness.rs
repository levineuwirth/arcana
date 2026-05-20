//! Homesickness — `{4}{U}{U}` instant. "Target player draws two
//! cards. Tap up to two target creatures. Put a stun counter on each
//! of them."
//!
//! First target a player (draws two); next up to two creature targets
//! are tapped and given stun counters.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Homesickness");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target player draws two cards. Tap up to two target creatures. Put a stun counter on each of them.".into(),
            target_requirements: vec![
                TargetRequirement::target_player(),
                TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(2),
                    controller: None,
                },
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = Vec::new();
    let mut it = entry.targets.targets.iter();
    if let Some(TargetChoice::Player(p)) = it.next() {
        effects.push(Effect::DrawCards { player: *p, count: 2 });
    }
    for t in it {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::Tap { target: *id });
            effects.push(Effect::AddCounters {
                target: *id,
                kind: CounterKind::Stun,
                count: 1,
            });
        }
    }
    effects
}
