//! Armix, Filigree Thrasher — `{2}{B}` 3/2 Legendary Artifact Creature —
//! Golem (black).
//!
//! * "Whenever Armix attacks, you may discard a card. When you do, target
//!   creature defending player controls gets -X/-X until end of turn, where
//!   X is the number of artifacts you control plus the number of artifact
//!   cards in your graveyard." — modeled as a SelfAttacks trigger that pumps
//!   target opponent-controlled creature by -X/-X. GAP: the "you may discard
//!   a card" reflexive cost/gate is not expressible on a triggered ability,
//!   so the pump is applied unconditionally (the dynamic X is faithful).
//! * Partner — not a usable KeywordAbility variant; recorded as a GAP.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Armix, Filigree Thrasher");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    // GAP: Partner is not a usable KeywordAbility variant.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
        id: 1,
        trigger_condition: TriggerCondition::SelfAttacks,
        intervening_if: None,
        effect: minus_x_x,
        trigger_zones: vec![Zone::Battlefield],
        frequency: TriggerFrequency::EachTime,
        target_requirements: vec![TargetRequirement {
            filter: TargetFilter::Permanent(
                ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
            ),
            count: TargetCount::Exactly(1),
            controller: None,
        }],
    }))
}

fn minus_x_x(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let artifact_filter = ObjectFilter::new()
        .with_types(TypeLine::ARTIFACT.into())
        .controlled_by(ControllerConstraint::You);
    let on_battlefield = script::count_matching(state, &artifact_filter, trig.controller);
    let in_graveyard = script::graveyard_matching(
        state,
        &ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
        trig.controller,
        trig.controller,
    );
    let x = (on_battlefield + in_graveyard) as i32;
    vec![Effect::Pump {
        target: *id,
        power: -x,
        toughness: -x,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
