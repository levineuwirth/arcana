//! Forked Lightning — `{3}{R}` sorcery. Deals 4 damage divided as you
//! choose among one, two, or three target creatures.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Forked Lightning");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Forked Lightning deals 4 damage divided as you choose among one, two, or three target creatures.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::creature()),
                    count: TargetCount::UpTo(3),
                    controller: None,
                }],
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
    // GAP: "damage divided as you choose" with custom per-target allocation
    // is not in the catalog. Best-effort: split 4 evenly across the chosen
    // targets (4/N each, with the first taking the remainder).
    let ids: Vec<_> = entry
        .targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(*id),
            _ => None,
        })
        .collect();
    if ids.is_empty() {
        return Vec::new();
    }
    let base = 4u32 / ids.len() as u32;
    let extra = 4u32 % ids.len() as u32;
    ids.iter()
        .enumerate()
        .map(|(i, id)| Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*id),
            amount: base + if (i as u32) < extra { 1 } else { 0 },
        })
        .collect()
}
