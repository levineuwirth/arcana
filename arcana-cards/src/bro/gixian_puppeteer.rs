//! Gixian Puppeteer — `{3}{B}` 4/3 Creature — Phyrexian Warlock.
//!
//! * Whenever you draw your second card each turn, each opponent loses 2 life
//!   and you gain 2 life. — `CardDrawn { player: You }` gated by an
//!   intervening-if that fires exactly when the current turn's draw count is 2.
//! * When this creature dies, return another target creature card with mana
//!   value 3 or less from your graveyard to the battlefield. — `SelfDies`
//!   trigger targeting a creature card (mv ≤ 3) in your graveyard, returned
//!   with `ReturnFromGraveyardToBattlefield`.

use arcana_core::effects::Effect;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gixian Puppeteer");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::You,
                },
                intervening_if: Some(if_second_draw_this_turn),
                effect: drain_two,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: reanimate_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature().with_max_cmc(3),
                    },
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::You),
                }],
            }),
    )
}

/// Intervening-if: fire only on the exact draw that makes today's count two.
fn if_second_draw_this_turn(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::cards_drawn_this_turn(s, you) == 2
}

/// "each opponent loses 2 life and you gain 2 life."
fn drain_two(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects: Vec<Effect> = script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::LoseLife { player: p, amount: 2 })
        .collect();
    effects.push(Effect::GainLife { player: trig.controller, amount: 2 });
    effects
}

/// "return another target creature card with mana value 3 or less from your
/// graveyard to the battlefield."
fn reanimate_target(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
