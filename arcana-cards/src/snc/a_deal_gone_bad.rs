//! A-Deal Gone Bad — `{3}{B}` instant, "Target creature gets -3/-3
//! until end of turn. Target player mills three cards. You gain 3
//! life." Three effects, two targets (creature + player).

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
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
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

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(creature_target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(creature_id) = creature_target else { return Vec::new(); };
    let mill_player = if let Some(player_target) = entry.targets.targets.get(1) {
        match player_target {
            TargetChoice::Player(p) => *p,
            _ => return Vec::new(),
        }
    } else {
        return Vec::new();
    };
    vec![
        Effect::Pump {
            target: *creature_id,
            power: -3,
            toughness: -3,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::Mill { player: mill_player, count: 3 },
        Effect::GainLife { player: entry.controller, amount: 3 },
    ]
}
