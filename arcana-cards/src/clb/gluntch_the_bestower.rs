//! Gluntch, the Bestower — `{1}{G}{W}` 0/5 Legendary Jellyfish with Flying.
//! At the beginning of your end step, choose a player. They put two +1/+1
//! counters on a creature they control. Choose a second player to draw a card.
//! Then choose a third player to create two Treasure tokens.
//!
//! Flying is modeled. The end-step trigger's second clause (choose a player to
//! draw) and third clause (choose a player to make two Treasures) are wired via
//! `ChoosePlayerThen`. The first clause — the chosen player then puts counters
//! on a creature THEY choose among their own — needs a player-pick that itself
//! posts an object-pick over the chosen player's creatures; that two-stage
//! choice isn't expressible, so it's GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gluntch, the Bestower");
    let jellyfish = reg.interner_mut().intern("Jellyfish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jellyfish);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_bestow,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn end_step_bestow(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "choose a player; they put two +1/+1 counters on a creature they
    // control" — requires a player pick that then posts an object pick over the
    // chosen player's creatures; the two-stage choice isn't expressible.
    vec![Effect::Sequence(vec![
        // Choose a second player to draw a card.
        Effect::ChoosePlayerThen {
            chooser: trig.controller,
            opponents_only: false,
            then: Box::new(Effect::DrawCards {
                player: trig.controller,
                count: 1,
            }),
        },
        // Then choose a third player to create two Treasure tokens.
        Effect::ChoosePlayerThen {
            chooser: trig.controller,
            opponents_only: false,
            then: Box::new(Effect::CreateCommodityToken {
                controller: trig.controller,
                kind: arcana_core::effects::CommodityToken::Treasure,
                count: 2,
            }),
        },
    ])]
}
