//! Rapid Hybridization — `{U}` Instant. "Destroy target creature. It
//! can't be regenerated. That creature's controller creates a 3/3 green
//! Frog Lizard creature token."
//!
//! # Implementation note
//! DestroyPermanent is expressed. "Can't be regenerated" is not
//! expressible (no Effect::PreventRegenerate). The Frog Lizard token
//! is created for the target's controller; since the only controller
//! we can read is `entry.controller`, we approximate by giving it to
//! the resolver's controller.
//!
//! # GAP
//! "Can't be regenerated" flag not in Effect catalog; token given to
//! caster rather than target's controller (no API to look up target's
//! controller at resolve time).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rapid Hybridization");
    let _frog_lizard = reg.interner_mut().intern("Frog Lizard");
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
                text: "Destroy target creature. It can't be regenerated. That creature's controller creates a 3/3 green Frog Lizard creature token.".into(),
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
    let frog_lizard = reg.interner().lookup("Frog Lizard")
        .expect("Frog Lizard interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(frog_lizard);
    let token = TokenDefinition {
        name: frog_lizard,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::DestroyPermanent { target: *id },
        // GAP: "can't be regenerated" not in Effect catalog
        // GAP: token should go to target's controller, not entry.controller
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
