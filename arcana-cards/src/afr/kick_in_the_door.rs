//! Kick in the Door — `{R}` sorcery, "Put a +1/+1 counter on target creature.
//! That creature gains haste until end of turn and can't be blocked by Walls
//! this turn. Venture into the dungeon."
//!
//! GAP: "can't be blocked by Walls this turn" restriction not expressible.
//! GAP: Venture into the dungeon mechanic not in effect catalog.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kick in the Door");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Put a +1/+1 counter on target creature. That creature gains haste until end of turn and can't be blocked by Walls this turn. Venture into the dungeon.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::AddCounters { target: *id, kind: CounterKind::PlusOnePlusOne, count: 1 },
        Effect::Pump {
            target: *id,
            power: 0,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Haste],
        },
        // GAP: can't be blocked by Walls this turn
        // GAP: Venture into the dungeon
    ]
}
