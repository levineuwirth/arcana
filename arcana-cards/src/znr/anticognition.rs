//! Anticognition — `{1}{U}` instant. "Counter target creature or
//! planeswalker spell unless its controller pays {2}. If an opponent
//! has eight or more cards in their graveyard, instead counter that
//! spell, then scry 2." The graveyard-conditional hard-counter+scry
//! branch isn't expressible; we emit the base soft counter ({2}).

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Anticognition");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Counter target creature or planeswalker spell unless its controller pays {2}. If an opponent has eight or more cards in their graveyard, instead counter that spell, then scry 2.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Spell(ObjectFilter::default()),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: opponent-graveyard-size-conditional hard counter + scry 2
    // branch not expressible. Base soft counter emitted.
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    vec![Effect::CounterUnlessPays {
        target: *id,
        cost: ManaCost::parse("{2}").expect("valid cost"),
    }]
}
