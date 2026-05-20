//! Combustion Technique — `{1}{R}` instant — Lesson. "Combustion Technique
//! deals damage equal to 2 plus the number of Lesson cards in your graveyard
//! to target creature. If that creature would die this turn, exile it
//! instead."
//!
//! GAP: no 'Lesson card in graveyard' filter (no subtype helper for cards in
//! graveyard via graveyard_matching needs an ObjectFilter that can specify a
//! subtype card-zone-side); using script::graveyard_matching with a generic
//! filter. No 'dies → exile' replacement available.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Combustion Technique");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Combustion Technique deals damage equal to 2 plus the number of Lesson cards in your graveyard to target creature. If that creature would die this turn, exile it instead.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let lessons = script::graveyard_matching(
        state,
        &script::subtype_filter(reg, "Lesson"),
        entry.controller,
        entry.controller,
    );
    // GAP: 'dies-replacement' rider dropped
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount: 2 + lessons,
    }]
}
