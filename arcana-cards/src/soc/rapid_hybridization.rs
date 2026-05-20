//! Rapid Hybridization — `{U}` instant.
//! "Destroy target creature. It can't be regenerated. That creature's
//! controller creates a 3/3 green Frog Lizard creature token."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rapid Hybridization");
    let _fl = reg.interner_mut().intern("Frog Lizard");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target creature. It can't be regenerated. That creature's controller creates a 3/3 green Frog Lizard creature token.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // "That creature's controller creates a token": no script helper
    // resolves an object's controller, so the token half (a 3/3
    // Frog Lizard for the destroyed creature's controller) is a GAP.
    vec![Effect::DestroyPermanent { target: *id }]
}
