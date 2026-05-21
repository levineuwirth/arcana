//! Fire at Will — `{R/W}{R/W}{R/W}` instant. "Fire at Will deals 3
//! damage divided as you choose among one, two, or three target
//! attacking or blocking creatures."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fire at Will");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R/W}{R/W}{R/W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Fire at Will deals 3 damage divided as you choose among one, two, or three target attacking or blocking creatures.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Creature,
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
    let targets = &entry.targets.targets;
    // "3 damage divided" — no damage-division primitive. Distribute
    // the 3 evenly: 1 each across the chosen targets (1/3 targets → 3
    // to the sole target).
    match targets.len() {
        0 => Vec::new(),
        1 => match &targets[0] {
            TargetChoice::Object(id) => vec![Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Object(*id),
                amount: 3,
            }],
            _ => Vec::new(),
        },
        _ => targets
            .iter()
            .filter_map(|t| match t {
                TargetChoice::Object(id) => Some(Effect::DealDamage {
                    source: entry.source,
                    target: DamageTarget::Object(*id),
                    amount: 1,
                }),
                _ => None,
            })
            .collect(),
    }
    // GAP: free "divide as you choose" allocation of the 3 damage,
    // and the "attacking or blocking" target restriction, are not
    // expressible; a fixed even split among the chosen creatures is
    // used.
}
