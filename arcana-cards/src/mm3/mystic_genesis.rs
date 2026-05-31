//! Mystic Genesis — `{2}{G}{U}{U}` instant. "Counter target spell.
//! Create an X/X green Ooze creature token, where X is that spell's
//! mana value." The counter is expressible; the token's X (the
//! countered spell's mana value) is a dynamic amount not available
//! from the scripting helpers.

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
    let name = reg.interner_mut().intern("Mystic Genesis");
    let _ooze = reg.interner_mut().intern("Ooze");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target spell. Create an X/X green Ooze creature token, where X is that spell's mana value.".into(),
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

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let stack_id = match target {
        TargetChoice::Object(id) => *id,
        _ => return Vec::new(),
    };
    // GAP: cannot read the countered spell's mana value at resolution
    // (no script:: helper exposes a stack object's mana value), so the
    // X/X Ooze token's dynamic power/toughness cannot be computed.
    // Emitting only the counter half rather than a wrong fixed-size token.
    vec![Effect::Counter { target: stack_id }]
}
