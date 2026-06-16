//! Werewolf Pack Leader — `{G}{G}` 3/3 Human Werewolf.
//! Pack tactics — Whenever this creature attacks, if you attacked with
//! creatures with total power 6 or greater this combat, draw a card.
//! (intervening-if GAP'd — no total-attacking-power predicate available.)
//! {3}{G}: Until end of turn, this creature has base power and toughness 5/3,
//! gains trample, and isn't a Human. ("isn't a Human" GAP'd — no
//! remove-subtype effect.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Werewolf Pack Leader");
    let human = reg.interner_mut().intern("Human");
    let werewolf = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(werewolf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                // GAP: intervening-if "if you attacked with creatures with
                // total power 6 or greater this combat" — no predicate for a
                // total-attacking-power check; fires unconditionally.
                intervening_if: None,
                effect: draw_on_attack,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{G}: Until end of turn, this creature has base power and toughness 5/3, gains trample, and isn't a Human.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_wolf,
            }),
    )
}

fn draw_on_attack(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}

fn become_wolf(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "isn't a Human" — no remove-subtype effect in the catalog.
    vec![
        Effect::SetBasePT {
            target: ctx.source,
            power: 5,
            toughness: 3,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Trample,
            duration: Duration::EndOfTurn,
        },
    ]
}
