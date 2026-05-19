//! Broken Visage — `{4}{B}` instant. "Destroy target nonartifact attacking
//! creature. It can't be regenerated. Create a black Spirit creature token.
//! Its power equals that creature's power and toughness equals that creature's
//! toughness. Sacrifice the token at the beginning of the next end step."
//!
//! GAP: nonartifact+attacking filter not in ObjectFilter; token P/T copying
//! source creature's power/toughness (dynamic) not expressible; sacrifice-at-
//! next-end-step delayed trigger not in catalog. Partial: creature destroy
//! expressed with fixed-size Spirit token placeholder (1/1).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Broken Visage");
    let _spirit = reg.interner_mut().intern("Spirit");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target nonartifact attacking creature. It can't be regenerated. Create a black Spirit creature token with the same power and toughness. Sacrifice the token at the beginning of the next end step.".into(),
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
    // GAP: nonartifact+attacking filter not available in ObjectFilter
    // GAP: token P/T copying source creature's stats (dynamic P/T) not expressible
    // GAP: sacrifice-at-next-end-step delayed trigger not in catalog
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };

    let spirit = reg.interner().lookup("Spirit").expect("Spirit interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let token = TokenDefinition {
        name: spirit,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };

    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
