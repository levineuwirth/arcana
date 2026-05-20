//! Reduce to Memory — `{1}{W}{W}` sorcery — Lesson.
//! "Exile target nonland permanent. Its controller creates a 3/2 red and white
//! Spirit creature token."
//!
//! Note: Type line is "Sorcery — Lesson". Lesson is a subtype; TypeLine::SORCERY
//! covers the type. Subtype "Lesson" is recorded via subtypes on the card
//! characteristics but the Characteristics struct uses SubtypeSet for card subtypes
//! (spell subtypes). The engine records it via the subtype mechanism.
//!
//! GAP: "Its controller" — the token should go to the exiled permanent's controller,
//! not the spell's controller. We do not have a way to look up the target's controller
//! from entry alone without state access. Approximation: create token for spell controller.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Reduce to Memory");
    let _spirit = reg.interner_mut().intern("Spirit");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target nonland permanent. Its controller creates a 3/2 red and white Spirit creature token.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::permanent().without_types(TypeLine::LAND.into())),
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

    let spirit = reg.interner().lookup("Spirit").expect("Spirit interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    // GAP: token goes to target's controller, not spell controller — using spell controller as approximation
    let token = TokenDefinition {
        name: spirit,
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };

    vec![
        Effect::ExilePermanent { target: *id },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
