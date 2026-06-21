//! Lozhan, Dragons' Legacy — `{3}{U}{R}` 4/2 Legendary Dragon Shaman with Flying.
//! "Whenever you cast an Adventure or Dragon spell, Lozhan deals damage equal
//!  to that spell's mana value to any target that isn't a commander."
//!
//! The cast trigger structure is preserved (filtered to Dragon spells you
//! cast; the "Adventure spell" half is not separately expressible). The
//! effect's amount is "that spell's mana value" — there is no PendingTrigger
//! accessor for the cast spell's mana value, so per the dynamic-amount rule
//! the whole effect is GAP'd rather than hardcoded.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lozhan, Dragons' Legacy");
    let dragon = reg.interner_mut().intern("Dragon");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // "a Dragon spell you cast" (the "Adventure spell" half is not separately
    // expressible).
    let dragon_spell = script::subtype_filter(reg, "Dragon");

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(dragon_spell),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: damage_for_mana_value,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn damage_for_mana_value(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
    // GAP: "deals damage equal to that spell's mana value to any target" — no
    // PendingTrigger accessor exposes the cast spell's mana value, so the
    // dynamic damage amount is not computable.
}
