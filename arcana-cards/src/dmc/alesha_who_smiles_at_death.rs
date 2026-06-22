//! Alesha, Who Smiles at Death — `{2}{R}` 3/2 Legendary Human Warrior
//! with First strike. "Whenever Alesha attacks, you may pay {W/B}{W/B}.
//! If you do, return target creature card with power 2 or less from your
//! graveyard to the battlefield tapped and attacking."
//!
//! First strike is a base keyword. The attack trigger targets a creature
//! card (power 2 or less) in your graveyard and, on optional {W/B}{W/B}
//! payment, returns it to the battlefield. The "tapped and attacking"
//! rider has no documented graveyard-return variant, so the reanimation
//! uses the plain battlefield return (a fidelity GAP on tapped+attacking).

use arcana_core::actions::OptionalPaymentKind;
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
    let name = reg.interner_mut().intern("Alesha, Who Smiles at Death");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: alesha_reanimate,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature().with_max_power(2),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn alesha_reanimate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{W/B}{W/B}").expect("valid cost")),
        // Returns to the battlefield; the "tapped and attacking" rider is
        // a fidelity GAP (no graveyard-return-tapped-attacking variant).
        then: Box::new(Effect::ReturnFromGraveyardToBattlefield { target: *id }),
        else_effect: None,
    }]
}
