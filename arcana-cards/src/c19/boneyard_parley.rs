//! Boneyard Parley — `{5}{B}{B}` sorcery. "Exile up to five target creature
//! cards from graveyards. An opponent separates those cards into two piles. Put
//! all cards from the pile of your choice onto the battlefield under your
//! control and the rest into their owners' graveyards."
//!
//! # GAP: "opponent separates into two piles, controller chooses pile" is a
//! Will-of-the-Council style interactive choice not expressible via the catalog.
//! We exile the targets and note the pile-choice gap.

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
    let name = reg.interner_mut().intern("Boneyard Parley");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile up to five target creature cards from graveyards. An opponent separates those cards into two piles. Put all cards from the pile of your choice onto the battlefield under your control and the rest into their owners' graveyards.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card { zone: Zone::Graveyard(0), filter: ObjectFilter::creature() },
                    count: TargetCount::UpTo(5),
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
    // GAP: opponent-separates-two-piles choice not expressible via catalog
    let mut effects = Vec::new();
    for t in &entry.targets.targets {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::ExileFromGraveyard { target: *id });
        }
    }
    effects
}
