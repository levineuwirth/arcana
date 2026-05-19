//! Angelic Ascension — `{1}{W}` instant. "Exile target creature or planeswalker.
//! Its controller creates a 4/4 white Angel creature token with flying."
//!
//! # GAP: token created under target's controller (not spell controller) is
//! not expressible — CreateToken only supports `entry.controller`. The exile
//! is implemented; the token is created under the spell's controller as an
//! approximation, which may differ from rules text.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Angelic Ascension");
    let _angel = reg.interner_mut().intern("Angel");
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
                text: "Exile target creature or planeswalker. Its controller creates a 4/4 white Angel creature token with flying.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types_any(TypeLine(TypeLine::CREATURE))
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
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let angel = reg.interner().lookup("Angel").expect("Angel interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    let token = TokenDefinition {
        name: angel,
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    // GAP: token should be created under the target's controller, not entry.controller
    vec![
        Effect::ExilePermanent { target: *id },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
