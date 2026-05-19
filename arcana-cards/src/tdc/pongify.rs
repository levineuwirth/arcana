//! Pongify — `{U}` instant, "Destroy target creature. It can't be regenerated.
//! That creature's controller creates a 3/3 green Ape creature token."
//!
//! GAP: "it can't be regenerated" (no regeneration-prevention effect);
//! target's controller creates the Ape token (no way to access target's
//! controller as PlayerId).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pongify");
    let _ape = reg.interner_mut().intern("Ape");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target creature. It can't be regenerated. That creature's controller creates a 3/3 green Ape creature token.".into(),
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
    let ape = reg.interner().lookup("Ape")
        .expect("Ape interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ape);
    let token = TokenDefinition {
        name: ape,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: "can't be regenerated" prevention effect not in catalog;
    // token should be created for target's controller (not entry.controller)
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
