//! Eternal Flame — `{2}{R}{R}` sorcery. "Eternal Flame deals X damage
//! to target opponent or planeswalker and half X damage, rounded up,
//! to you, where X is the number of Mountains you control." Dynamic X
//! = number of Mountains you control.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eternal Flame");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Eternal Flame deals X damage to target opponent or planeswalker and half X damage, rounded up, to you, where X is the number of Mountains you control.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let x = script::count_matching(
        state,
        &script::subtype_filter(reg, "Mountain"),
        entry.controller,
    );
    let half = x.div_ceil(2);
    let mut out = Vec::new();
    if let Some(TargetChoice::Player(p)) = entry.targets.targets.first() {
        out.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(*p),
            amount: x,
        });
    }
    out.push(Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Player(entry.controller),
        amount: half,
    });
    out
}
