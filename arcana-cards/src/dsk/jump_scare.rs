//! Jump Scare — `{W}` instant. "Until end of turn, target creature
//! gets +2/+2, gains flying, and becomes a Horror enchantment
//! creature in addition to its other types." Pump+flying via
//! Effect::Pump; the type-becoming via Effect::AddType (enchantment
//! creature) + a targeted Horror subtype-add continuous effect, both
//! until end of turn.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jump Scare");
    let _horror = reg.interner_mut().intern("Horror"); // looked up in resolve
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Until end of turn, target creature gets +2/+2, gains flying, and becomes a Horror enchantment creature in addition to its other types.".into(),
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
    let mut subs = SubtypeSet::default();
    if let Some(horror) = reg.interner().lookup("Horror") {
        subs.0.insert(horror);
    }
    vec![
        Effect::Pump {
            target: *id,
            power: 2,
            toughness: 2,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Flying],
        },
        // "…and becomes a Horror enchantment creature in addition to
        // its other types" — additive Layer-4 type overlay + targeted
        // subtype-add, until end of turn.
        Effect::AddType {
            target: *id,
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            duration: Duration::EndOfTurn,
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::add_subtypes(0, *id, subs, Duration::EndOfTurn),
        },
    ]
}
