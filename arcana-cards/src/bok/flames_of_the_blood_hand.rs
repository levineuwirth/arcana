//! Flames of the Blood Hand — `{2}{R}` instant. "Flames of the Blood
//! Hand deals 4 damage to target player or planeswalker."

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
    let name = reg.interner_mut().intern("Flames of the Blood Hand");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Flames of the Blood Hand deals 4 damage to target player or planeswalker. The damage can't be prevented. If that player or that planeswalker's controller would gain life this turn, that player gains no life instead.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let p = match target {
        TargetChoice::Player(p) => *p,
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => *p,
        _ => return Vec::new(),
    };
    // GAP: "damage can't be prevented" and the can't-gain-life
    // replacement effect are not expressible; the 4 damage is modeled.
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Player(p),
        amount: 4,
    }]
}
