//! Necrotic Wound — `{B}` instant, "Undergrowth — Target creature gets
//! -X/-X until end of turn, where X is the number of creature cards in your
//! graveyard. If that creature would die this turn, exile it instead."
//!
//! GAP: exile-instead-of-dying replacement effect is not expressible.
//! Partial: -X/-X pump only, where X = creature cards in graveyard.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Necrotic Wound");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Undergrowth — Target creature gets -X/-X until end of turn, where X is the number of creature cards in your graveyard. If that creature would die this turn, exile it instead.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: exile-instead-of-dying replacement effect
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let filter = ObjectFilter::creature();
    let x = script::graveyard_matching(state, &filter, entry.controller, entry.controller) as i32;
    vec![Effect::Pump {
        target: *id,
        power: -x,
        toughness: -x,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
