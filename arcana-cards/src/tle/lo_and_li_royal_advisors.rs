//! Lo and Li, Royal Advisors — `{2}{B}{B}` 3/3 Legendary Creature —
//! Human Advisor.
//!
//! Oracle:
//! * "Whenever an opponent discards a card or mills one or more cards, put a
//!   +1/+1 counter on each Advisor you control." — a compound trigger. The
//!   DISCARD side is wired via `CardDiscarded { player: Opponent }`, putting a
//!   +1/+1 counter on each Advisor you control (one `AddCounters` per matching
//!   id). GAP'd: the "or mills one or more cards" side — there is no
//!   mill/library-to-graveyard event trigger condition in the demonstrated
//!   surface, so only the discard half fires.
//! * "{2}{U/B}: Target player mills four cards." — a mana activation that
//!   mills the targeted player four cards.
//! (The Scryfall "Mill" keyword is reminder text, not a KeywordAbility.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lo and Li, Royal Advisors");
    let human = reg.interner_mut().intern("Human");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "or mills one or more cards" — no mill-event trigger
                // condition; only the opponent-discards side is wired.
                trigger_condition: TriggerCondition::CardDiscarded {
                    player: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: counter_each_advisor,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{U/B}: Target player mills four cards.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U/B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: mill_target_player,
            }),
    )
}

fn counter_each_advisor(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter =
        script::subtype_filter(reg, "Advisor").controlled_by(ControllerConstraint::You);
    let targets = script::ids_matching(state, &filter, trig.controller);
    vec![Effect::ForEach {
        targets,
        effect: Box::new(Effect::AddCounters {
            target: arcana_core::objects::NULL_OBJECT_ID,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        }),
    }]
}

fn mill_target_player(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    vec![Effect::Mill {
        player: *p,
        count: 4,
    }]
}
