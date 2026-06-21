//! Kroxa and Kunoros — `{3}{R}{W}{B}` 6/6 Legendary Elder Giant Dog.
//! Vigilance, menace, lifelink.
//! Whenever Kroxa and Kunoros enters or attacks, you may exile five
//! cards from your graveyard. When you do, return target creature card
//! from your graveyard to the battlefield.
//!
//! "Enters or attacks" is modeled as two triggers (enters / attacks)
//! sharing one resolver. The resolver posts the optional "exile five
//! cards from your graveyard" pick (ChooseNFromZone min=max=5, Exile),
//! then returns the chosen graveyard creature card to the battlefield.
//! The reflexive "When you do" gating is approximated by sequencing both
//! steps (the target is chosen up front); if fewer than five cards exist
//! the pick is clamped — a documented fidelity gap.

use arcana_core::effects::{Effect, PickAction};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::effects::KeywordAbility;
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kroxa and Kunoros");
    let elder = reg.interner_mut().intern("Elder");
    let giant = reg.interner_mut().intern("Giant");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elder);
    subtypes.0.insert(giant);
    subtypes.0.insert(dog);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{W}{B}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![
            KeywordAbility::Vigilance,
            KeywordAbility::Menace,
            KeywordAbility::Lifelink,
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: exile_five_reanimate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![reanimate_target()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: exile_five_reanimate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![reanimate_target()],
            }),
    )
}

fn reanimate_target() -> TargetRequirement {
    TargetRequirement {
        filter: TargetFilter::Card {
            zone: Zone::Graveyard(0),
            filter: ObjectFilter::creature(),
        },
        count: TargetCount::Exactly(1),
        controller: None,
    }
}

fn exile_five_reanimate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Sequence(vec![
        Effect::ChooseNFromZone {
            chooser: trig.controller,
            zone: Zone::Graveyard(trig.controller),
            filter: ObjectFilter::new(),
            min: 5,
            max: 5,
            action: PickAction::Exile,
        },
        Effect::ReturnFromGraveyardToBattlefield { target: *id },
    ])]
}
