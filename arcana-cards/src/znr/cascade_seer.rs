//! Cascade Seer — `{3}{U}` 3/3 blue Merfolk Wizard creature.
//! "When this creature enters, scry X, where X is the number of creatures in your party."
//! Keywords (Scryfall-parsed): Scry (handled via trigger)
//!
//! # Notes
//! Party = up to one Cleric, Rogue, Warrior, Wizard. Max X = 4.
//! GAP: no script helper for "number of creatures in your party" (party counting requires
//! checking specific subtypes for at most one each). Using count_matching on party subtypes
//! is an approximation; true party count is capped at 4 and counts distinct roles.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cascade Seer");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    let _cleric = reg.interner_mut().intern("Cleric");
    let _rogue = reg.interner_mut().intern("Rogue");
    let _warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_scry_party,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_scry_party(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: party count (at most one Cleric, Rogue, Warrior, Wizard) — approximating by summing
    // distinct party roles, capped at 4.
    let mut party_count: u32 = 0;
    for subtype_name in &["Cleric", "Rogue", "Warrior", "Wizard"] {
        let filter = script::subtype_filter(reg, subtype_name)
            .controlled_by(ControllerConstraint::You);
        if script::count_matching(state, &filter, trig.controller) > 0 {
            party_count += 1;
        }
    }
    if party_count == 0 {
        return Vec::new();
    }
    vec![Effect::Scry { player: trig.controller, count: party_count }]
}
