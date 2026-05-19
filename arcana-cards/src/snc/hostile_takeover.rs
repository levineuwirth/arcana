//! Hostile Takeover — `{2}{U}{B}{R}` sorcery. "Up to one target creature
//! has base power and toughness 1/1 until end of turn. Up to one other
//! target creature has base power and toughness 4/4 until end of turn.
//! Then Hostile Takeover deals 3 damage to each creature."
//!
//! GAP: ForEach+DealDamage to each creature requires enumerating state
//! battlefield by type — state accessor pattern not shown in catalog.
//! The 1/1 and 4/4 SetBasePT effects on the two targets are expressed;
//! the "deals 3 damage to each creature" sweep is omitted.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hostile Takeover");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Up to one target creature has base power and toughness 1/1 until end of turn. Up to one other target creature has base power and toughness 4/4 until end of turn. Then Hostile Takeover deals 3 damage to each creature.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
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
    let mut effects: Vec<Effect> = Vec::new();

    if let Some(TargetChoice::Object(id)) = entry.targets.targets.first() {
        effects.push(Effect::SetBasePT {
            target: *id,
            power: 1,
            toughness: 1,
            duration: Duration::EndOfTurn,
        });
    }
    if let Some(TargetChoice::Object(id)) = entry.targets.targets.get(1) {
        effects.push(Effect::SetBasePT {
            target: *id,
            power: 4,
            toughness: 4,
            duration: Duration::EndOfTurn,
        });
    }
    // GAP: DealDamageToEachCreature (3 damage to each creature on the battlefield)
    effects
}
