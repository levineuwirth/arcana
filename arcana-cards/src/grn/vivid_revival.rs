//! Vivid Revival — `{4}{G}` sorcery, "Return up to three target multicolored cards from your
//! graveyard to your hand. Exile Vivid Revival."
//!
//! GAP: 'multicolored' filter on graveyard cards (filter by having 2+ colors) is not an available
//! ObjectFilter refinement. 'Exile Vivid Revival' (self-exile from stack) is also not in the
//! catalog. Best-effort: return up to 3 targets from graveyard (color filter omitted).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vivid Revival");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return up to three target multicolored cards from your graveyard to your hand. Exile Vivid Revival.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card { zone: Zone::Graveyard(0), filter: ObjectFilter::new() },
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
    // GAP: multicolored filter on graveyard cards; self-exile (Exile Vivid Revival) not in catalog
    entry.targets.targets.iter().filter_map(|t| {
        if let TargetChoice::Object(id) = t {
            Some(Effect::ReturnFromGraveyardToHand { target: *id })
        } else {
            None
        }
    }).collect()
}
