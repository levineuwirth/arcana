//! A-Deal Gone Bad — `{3}{B}` instant. "Target creature gets -3/-3
//! until end of turn. Target player mills three cards. You gain 3
//! life." Two targets: a creature and a player.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Deal Gone Bad");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature gets -3/-3 until end of turn. Target player mills three cards. You gain 3 life.".into(),
            target_requirements: vec![
                TargetRequirement::target_creature(),
                TargetRequirement::target_player(),
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mut out = Vec::new();
    if let Some(TargetChoice::Object(id)) = entry.targets.targets.first() {
        out.push(Effect::Pump {
            target: *id,
            power: -3,
            toughness: -3,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        });
    }
    if let Some(TargetChoice::Player(p)) = entry.targets.targets.get(1) {
        out.push(Effect::Mill { player: *p, count: 3 });
    }
    out.push(Effect::GainLife { player: entry.controller, amount: 3 });
    out
}
