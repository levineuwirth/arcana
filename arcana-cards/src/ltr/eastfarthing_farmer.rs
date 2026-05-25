//! Eastfarthing Farmer — `{2}{W}` 2/3 Halfling Peasant.
//! Keywords: Food
//! "When this creature enters, create a Food token. When you do,
//! target creature you control gets +1/+1 until end of turn for each
//! Food you control."
//!
//! GAP: "When you do" — a reflexive trigger on the token creation
//! event is not supported in the TriggeredAbilityDef model (only one
//! trigger per card). Emit the Food creation and the pump as a
//! Sequence. The count of Foods is computed at resolution time.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eastfarthing Farmer");
    let halfling = reg.interner_mut().intern("Halfling");
    let peasant = reg.interner_mut().intern("Peasant");
    let _food = reg.interner_mut().intern("Food");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(halfling);
    subtypes.0.insert(peasant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn on_etb(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let food_id = reg
        .interner()
        .lookup("Food")
        .expect("Food interned during register()");
    let mut food_subtypes = SubtypeSet::default();
    food_subtypes.0.insert(food_id);
    let food_token = TokenDefinition {
        name: food_id,
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        subtypes: food_subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    // Count Foods including the one being created (+1)
    let food_filter = script::subtype_filter(reg, "Food");
    let n = script::count_matching(state, &food_filter, trig.controller) + 1;
    vec![
        Effect::CreateToken { controller: trig.controller, token: food_token },
        Effect::Pump {
            target: *id,
            power: n as i32,
            toughness: n as i32,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
    ]
}
