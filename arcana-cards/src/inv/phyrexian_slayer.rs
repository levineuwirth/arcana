//! Phyrexian Slayer — `{3}{B}` 2/2 black Phyrexian Minion.
//! "Flying
//! Whenever this creature becomes blocked by a white creature, destroy that
//! creature. It can't be regenerated."
//!
//! Flying is a base keyword. The block trigger uses the filtered
//! `SelfBecomesBlockedBy { filter: white creature }` and reads the blocker via
//! `trig.other_combatant()`, then destroys it. The "can't be regenerated"
//! rider is GAP'd — `Effect::DestroyPermanent` has no no-regeneration flag.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phyrexian Slayer");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let minion = reg.interner_mut().intern("Minion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(minion);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesBlockedBy {
                    filter: ObjectFilter::creature().with_colors(ColorSet::white()),
                },
                intervening_if: None,
                effect: destroy_white_blocker,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn destroy_white_blocker(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.other_combatant() else {
        return Vec::new();
    };
    // GAP: "It can't be regenerated" — DestroyPermanent has no no-regen flag.
    vec![Effect::DestroyPermanent { target: id }]
}
