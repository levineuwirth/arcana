//! Harsh Annotation — `{1}{W}` instant. "Destroy target creature. Its
//! controller creates a 1/1 white and black Inkling creature token with flying."
//!
//! GAP: "its controller creates a token" — CreateToken gives token to a PlayerId;
//! cannot retrieve the destroyed creature's controller from the resolver without
//! state field access. Best-effort: give the token to entry.controller.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Harsh Annotation");
    let _inkling = reg.interner_mut().intern("Inkling");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target creature. Its controller creates a 1/1 white and black Inkling creature token with flying.".into(),
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
    let inkling = reg.interner().lookup("Inkling").expect("Inkling interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(inkling);
    let token = TokenDefinition {
        name: inkling,
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    // GAP: token should go to the destroyed creature's controller, not necessarily entry.controller
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
