//! Kemba, Kha Regent — `{1}{W}{W}` 2/4 white Legendary Creature — Cat Cleric.
//! "At the beginning of your upkeep, create a 2/2 white Cat creature token for each
//! Equipment attached to Kemba."
//!
//! # Notes
//! The number of tokens is dynamic (equal to number of Equipment attached to this creature).
//! GAP: script::count_matching can count Equipment on the battlefield but cannot filter
//! specifically for Equipment attached to a particular permanent. Using count_matching with
//! a rough Equipment filter as best approximation; engine will need to refine.
//! GAP: no ObjectFilter for "attached to this permanent" — using Equipment type filter.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kemba, Kha Regent");
    let cat = reg.interner_mut().intern("Cat");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(cleric);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_create_cat_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_create_cat_tokens(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let cat = reg.interner().lookup("Cat").expect("Cat interned during register()");
    // GAP: count Equipment specifically attached to Kemba — using all Equipment you control
    // as approximation; no "attached to this permanent" filter available.
    let equipment_filter = ObjectFilter::permanent()
        .controlled_by(ControllerConstraint::You)
        .with_types(TypeLine::ARTIFACT.into());
    let n = script::count_matching(state, &equipment_filter, trig.controller);
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(cat);
    let token = TokenDefinition {
        name: cat,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    (0..n).map(|_| Effect::CreateToken { controller: trig.controller, token: token.clone() }).collect()
}
