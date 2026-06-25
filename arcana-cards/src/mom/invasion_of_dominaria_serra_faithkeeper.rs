//! Invasion of Dominaria // Serra Faithkeeper — `{2}{W}` Battle — Siege
//! with 4 defense counters (front) / Angel creature (back).
//!
//! Front face (Invasion of Dominaria):
//!   (As a Siege enters, choose an opponent to protect it. You and others
//!   can attack it. When it's defeated, exile it, then cast it transformed.)
//!   When this Siege enters, you gain 4 life and draw a card.
//!
//! Back face (Serra Faithkeeper):
//!   Flying, vigilance — both are intrinsic keywords on the back-face
//!   characteristics, so they are live the moment the defeat-transform
//!   (auto-wired by the engine SBA, CR 310.11) flips to the creature face.
//!
//! GAP: Siege protector-designation rule simplified (any opponent's battle
//!   is attackable).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Dominaria");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Serra Faithkeeper — Angel
    let back_name = reg.interner_mut().intern("Serra Faithkeeper");
    let angel_sub = reg.interner_mut().intern("Angel");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(angel_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 4,
            })
            // ETB trigger: gain 4 life and draw a card.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_siege_enters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
    )
}

fn on_siege_enters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::GainLife {
            player: trig.controller,
            amount: 4,
        },
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
    ]
}
