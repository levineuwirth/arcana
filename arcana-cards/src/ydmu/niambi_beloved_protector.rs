//! Niambi, Beloved Protector — `{W}{U}` 2/2 Legendary Human Cleric.
//! Flash.
//! When Niambi enters, if you cast it, choose target nonlegendary creature
//! card in your graveyard that was put there from the battlefield this turn.
//! Return it to the battlefield. It perpetually gains a target-triggered draw.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Niambi, Beloved Protector");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    let gy_nonleg_creature = ObjectFilter::creature()
        .without_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY));

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            // GAP: "if you cast it" cast-condition has no documented
            // intervening-if predicate; trigger fires on every ETB.
            intervening_if: None,
            effect: reanimate_target,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                // GAP: "that was put there from the battlefield this turn"
                // restriction is not expressible in the card target filter.
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: gy_nonleg_creature,
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn reanimate_target(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "It perpetually gains '...'" — perpetual ability grants are not
    // expressible with the documented Effect surface.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
