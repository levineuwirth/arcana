//! Savor — `{1}{B}` instant. "Target creature gets -2/-2 until end of turn.
//! Create a Food token."
//!
//! # GAP: Food token is a special artifact token with an activated ability
//! ("{2}, {T}, Sacrifice: Gain 3 life"). TokenDefinition has no activated
//! abilities field. We create a generic artifact token and note the gap.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Savor");
    let _food = reg.interner_mut().intern("Food");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature gets -2/-2 until end of turn. Create a Food token.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let food = reg.interner().lookup("Food").expect("Food interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(food);
    // GAP: Food token's activated ability ("{2},{T},Sacrifice: gain 3 life") not
    // expressible in TokenDefinition.abilities (no activated ability type)
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
    vec![
        Effect::Pump {
            target: *id,
            power: -2,
            toughness: -2,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
