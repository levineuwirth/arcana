//! The Art of Tea — `{1}{G}` instant. "Put a +1/+1 counter on up to
//! one target creature you control. Create a Food token."
//!
//! The Food token's activated ability ("{2}, {T}, Sacrifice: gain 3
//! life") is not modeled — the token is created as an artifact named
//! Food with the Food subtype only.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};
use arcana_core::effects::TokenDefinition;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Art of Tea");
    let _food = reg.interner_mut().intern("Food");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Put a +1/+1 counter on up to one target creature you control. Create a Food token.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::UpTo(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let food = reg.interner().lookup("Food").expect("interned");
    let mut out = Vec::new();
    if let Some(TargetChoice::Object(id)) = entry.targets.targets.first() {
        out.push(Effect::AddCounters {
            target: *id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        });
    }
    let mut sub = SubtypeSet::default();
    sub.0.insert(food);
    out.push(Effect::CreateToken {
        controller: entry.controller,
        token: TokenDefinition {
            name: food,
            colors: ColorSet::new(),
            types: TypeLine::ARTIFACT.into(),
            subtypes: sub,
            power: None,
            toughness: None,
            keywords: vec![],
            abilities: vec![],
        },
    });
    out
}
