//! Second Breakfast — `{2}{W}` instant. "Up to two target creatures
//! each get +2/+1 until end of turn. Create a Food token."
//!
//! GAP: Food token has an activated ability ({2},{T}, Sacrifice: gain 3
//! life) which is not expressible via TokenDefinition.abilities. Token
//! is created without the ability.
//! Two target creatures require two separate Pump effects.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Second Breakfast");
    let _food = reg.interner_mut().intern("Food");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Up to two target creatures each get +2/+1 until end of turn. Create a Food token.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::creature()),
                    count: TargetCount::UpTo(2),
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
    reg: &CardRegistry,
) -> Vec<Effect> {
    let food = reg.interner().lookup("Food")
        .expect("Food interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(food);
    let mut effects = Vec::new();
    for target in &entry.targets.targets {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::Pump {
                target: *id,
                power: 2,
                toughness: 1,
                duration: Duration::EndOfTurn,
                keywords: vec![],
            });
        }
    }
    // GAP: Food token activated ability ({2},{T}, Sacrifice: gain 3 life)
    let token = TokenDefinition {
        name: food,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    effects.push(Effect::CreateToken { controller: entry.controller, token });
    effects
}
