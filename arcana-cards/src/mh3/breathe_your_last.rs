//! Breathe Your Last — `{1}{B}{B}` instant. "Destroy target creature
//! or planeswalker. You gain 1 life for each of its colors."
//!
//! The destruction is expressed faithfully (target a creature-or-
//! planeswalker permanent, then `Effect::DestroyPermanent`). The life
//! gain "1 life for each of its colors" is dynamic in the destroyed
//! permanent's color count, and no `script::*` helper exposes a
//! permanent's number of colors — so the life-gain rider is GAP'd
//! rather than hardcoded to a wrong literal.

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
    let name = reg.interner_mut().intern("Breathe Your Last");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target creature or planeswalker. You gain 1 life for each of its colors.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().with_types_any(TypeLine(
                            TypeLine::CREATURE | TypeLine::PLANESWALKER,
                        )),
                    ),
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
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "gain 1 life for each of its colors" — no script helper counts a
    // permanent's number of colors, so the life-gain rider is omitted rather
    // than hardcoded to a wrong fixed amount.
    vec![Effect::DestroyPermanent { target: *id }]
}
