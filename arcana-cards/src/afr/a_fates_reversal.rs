//! A-Fates' Reversal — `{B}` sorcery. "Return up to one target creature card
//! from your graveyard to your hand. Venture into the dungeon."
//!
//! GAP: Venture into the dungeon is not in the engine Effect catalog.
//! The graveyard recursion is rendered; venture is not expressible.

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
    let name = reg.interner_mut().intern("A-Fates' Reversal");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return up to one target creature card from your graveyard to your hand. Venture into the dungeon.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card { zone: Zone::Graveyard(0), filter: ObjectFilter::creature() },
                    count: TargetCount::UpTo(1),
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
    // GAP: Venture into the dungeon is not in the Effect catalog.
    let mut effects = Vec::new();
    if let Some(t0) = entry.targets.targets.first() {
        if let TargetChoice::Object(id) = t0 {
            effects.push(Effect::ReturnFromGraveyardToHand { target: *id });
        }
    }
    effects
}
