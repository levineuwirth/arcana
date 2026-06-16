//! Nulldrifter — `{7}` 4/4 Eldrazi Elemental with Flying.
//! "When you cast this spell, draw two cards. Annihilator 1. Evoke {2}{U}."
//!
//! Flying is expressible. Annihilator 1 is modeled as its reminder text: when
//! this creature attacks, the defending player sacrifices a permanent. The cast
//! trigger and Evoke are GAP'd (no self-cast trigger scoping / no evoke cost).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nulldrifter");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        // GAP: Evoke {2}{U} — no evoke alternative-cost field in the demonstrated API.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "When you cast this spell, draw two cards" — no self-only cast
            // trigger scoping; SpellCast{caster:You} would over-fire on every spell.
            // Annihilator 1 (reminder text): when this attacks, defending player
            // sacrifices a permanent of their choice.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: annihilator_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn annihilator_one(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(p) = trig.defending_player() else { return Vec::new(); };
    vec![Effect::Sacrifice { player: p, filter: ObjectFilter::permanent(), count: 1 }]
}
