//! Grixis Sojourners — `{1}{U}{B}{R}` 4/3 Zombie Ogre with Cycling {2}{B}.
//!
//! * "When you cycle this card and when this creature dies, you may exile target card
//!   from a graveyard." Two distinct triggers. The dies half is wired (SelfDies →
//!   exile target graveyard card). GAP: the cycle half — no "when you cycle this card"
//!   TriggerCondition variant. The "you may" is modeled as a mandatory exile (no
//!   optional-trigger gate available).
//! * Cycling {2}{B}.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::effects::KeywordAbility;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grixis Sojourners");
    let zombie = reg.interner_mut().intern("Zombie");
    let ogre = reg.interner_mut().intern("Ogre");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(ogre);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Cycling(
            ManaCost::parse("{2}{B}").expect("valid cost"),
        )],
        ..Default::default()
    };

    reg.register(
        // GAP: the "when you cycle this card" trigger half — no cycle TriggerCondition.
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: exile_graveyard_card,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::default(),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn exile_graveyard_card(
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
    vec![Effect::ExileFromGraveyard { target: *id }]
}
