//! Unchecked Growth — `{2}{G}` Instant — Arcane. "Target creature gets +4/+4 until end of turn.
//! If it's a Spirit, it gains trample until end of turn."
//! GAP: Arcane subtype — TypeLine::INSTANT used; Arcane is a spell subtype not modeled.
//! GAP: "If it's a Spirit, it gains trample" — conditional keyword grant based on subtype check
//! not expressible (Effect::Conditional condition type for subtype not available).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unchecked Growth");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature gets +4/+4 until end of turn. If it's a Spirit, it gains trample until end of turn.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "If it's a Spirit, gains trample" — conditional keyword grant based on subtype check not expressible
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Pump {
        target: *id,
        power: 4,
        toughness: 4,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
