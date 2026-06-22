//! Tegan Jovanka — `{2}{W}` 2/2 Legendary Human.
//!
//! * "Brave Heart — Whenever you attack, target attacking historic creature
//!   gets +1/+1 and gains indestructible until end of turn." — modelled as a
//!   `CreatureAttacks` (your creatures) trigger targeting an attacking creature
//!   and pumping it +1/+1 with indestructible. The "historic" (artifact OR
//!   legendary OR Saga) restriction on the target is an OR across
//!   type/supertype/subtype that the demonstrated `ObjectFilter` can't express,
//!   so the target is just an attacking creature and "historic" is GAP'd.
//! * "Doctor's companion" — GAP: a Commander deckbuilding rule, not a
//!   triggered/activated ability and not in the usable KeywordAbility surface.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
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
    let name = reg.interner_mut().intern("Tegan Jovanka");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: "Doctor's companion" / "Brave Heart" are not in the usable
        // KeywordAbility surface.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // "Whenever you attack" — CreatureAttacks with your creatures.
            trigger_condition: TriggerCondition::CreatureAttacks {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            },
            intervening_if: None,
            effect: pump_attacker,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            // GAP: "historic" target restriction (artifact OR legendary OR
            // Saga) is an OR across type/supertype/subtype, not expressible.
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(ObjectFilter::creature().attacking_only()),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn pump_attacker(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Indestructible],
    }]
}
