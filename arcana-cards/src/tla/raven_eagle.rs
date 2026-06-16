//! Raven Eagle — `{2}{B}` 2/3 Bird Assassin with Flying.
//! "Whenever this creature enters or attacks, exile up to one target card from a
//! graveyard. If a creature card is exiled this way, create a Clue token."
//! "Whenever you draw your second card each turn, each opponent loses 1 life and
//! you gain 1 life."
//!
//! Flying is a base keyword. "Enters or attacks" is decomposed into two triggers
//! (an ETB and an attack trigger), each exiling up to one target card from a
//! graveyard. The "if a creature card is exiled this way, create a Clue" rider
//! is conditional on the exiled card's type at resolution and is GAP'd. The
//! second-card-each-turn trigger has no ordinal-draw trigger condition — GAP'd.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Raven Eagle");
    let bird = reg.interner_mut().intern("Bird");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(assassin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: exile_card_from_graveyard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![exile_target()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: exile_card_from_graveyard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![exile_target()],
            }),
    )
    // GAP: "Whenever you draw your second card each turn, each opponent loses 1
    // life and you gain 1 life" — no ordinal (Nth draw this turn) trigger
    // condition is expressible in this class.
}

fn exile_target() -> TargetRequirement {
    TargetRequirement {
        filter: TargetFilter::Card {
            zone: Zone::Graveyard(0),
            filter: ObjectFilter::default(),
        },
        count: TargetCount::UpTo(1),
        controller: None,
    }
}

fn exile_card_from_graveyard(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "If a creature card is exiled this way, create a Clue token" — the
    // conditional Clue creation depends on the exiled card's type at resolution
    // and is not expressible here; the exile itself is emitted.
    vec![Effect::ExileFromGraveyard { target: *id }]
}
