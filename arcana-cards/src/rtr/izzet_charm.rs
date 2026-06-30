//! Izzet Charm — `{U}{R}` modal instant. "Choose one — Counter target
//! noncreature spell unless its controller pays {2}; or Izzet Charm deals 2
//! damage to target creature; or draw two cards, then discard two cards."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    CardDefinition, CardRegistry, ModalSpec, ModeClause, SpellAbilityDef,
};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Izzet Charm");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
        text: "Choose one — Counter target noncreature spell unless its controller pays {2}; or Izzet Charm deals 2 damage to target creature; or draw two cards, then discard two cards.".into(),
        target_requirements: vec![],
        modal: Some(ModalSpec {
            min_modes: 1,
            max_modes: 1,
            clauses: vec![
                ModeClause {
                    text: "Counter target noncreature spell unless its controller pays {2}.".into(),
                    target_requirements: vec![TargetRequirement {
                        filter: TargetFilter::Spell(
                            ObjectFilter::new().without_types(TypeLine::CREATURE.into()),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    }],
                },
                ModeClause {
                    text: "Izzet Charm deals 2 damage to target creature.".into(),
                    target_requirements: vec![TargetRequirement::target_creature()],
                },
                ModeClause {
                    text: "Draw two cards, then discard two cards.".into(),
                    target_requirements: vec![],
                },
            ],
        }),
        effect: resolve,
    }))
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(choice) = entry.modes.first() else { return Vec::new(); };
    let Some(&mode) = choice.mode_indices.first() else { return Vec::new(); };
    match mode {
        0 => match entry.targets.targets.first() {
            Some(TargetChoice::Object(id)) => vec![Effect::CounterUnlessPays {
                target: *id,
                cost: ManaCost::parse("{2}").expect("valid cost"),
            }],
            _ => Vec::new(),
        },
        1 => match entry.targets.targets.first() {
            Some(TargetChoice::Object(id)) => vec![Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Object(*id),
                amount: 2,
            }],
            _ => Vec::new(),
        },
        2 => vec![
            Effect::DrawCards { player: entry.controller, count: 2 },
            Effect::Discard {
                player: entry.controller,
                count: 2,
                choice: DiscardChoice::ControllerChooses,
            },
        ],
        _ => Vec::new(),
    }
}
