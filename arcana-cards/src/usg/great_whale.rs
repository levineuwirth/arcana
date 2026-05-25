//! Great Whale — `{5}{U}{U}` 5/5 blue Whale.
//! "When this creature enters, untap up to seven lands."
//!
//! GAP: "untap up to seven target lands" — TargetCount::UpTo(7) with a land
//! filter, but the handler would need to iterate over all chosen targets.
//! Emitting a ForEach over up to 7 chosen land targets via target_requirements
//! is the closest approximation; using a single Untap per chosen target is
//! not directly supported without multiple targets declared. Best-effort:
//! declare UpTo(7) creature targets and untap each.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Great Whale");
    let whale = reg.interner_mut().intern("Whale");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(whale);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_untap_lands,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::new().with_types(TypeLine::LAND.into()),
                ),
                count: TargetCount::UpTo(7),
                controller: None,
            }],
        }),
    )
}

fn etb_untap_lands(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    trig.targets
        .targets
        .iter()
        .filter_map(|t| {
            if let TargetChoice::Object(id) = t {
                Some(Effect::Untap { target: *id })
            } else {
                None
            }
        })
        .collect()
}
