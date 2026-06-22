//! Kambal, Profiteering Mayor — `{1}{W}{B}` 2/4 Legendary Human Advisor.
//!
//! Whenever one or more tokens your opponents control enter, for each of
//! them, create a tapped token that's a copy of it. This ability triggers
//! only once each turn.
//! Whenever one or more tokens you control enter, each opponent loses 1
//! life and you gain 1 life.
//!
//! Decomposed as: two token-enters triggers, each watching a token entering
//! the battlefield (ZoneChange with a tokens-only filter). The first
//! (opponents' tokens, once per turn) copies each entering token tapped;
//! that effect is GAP'd because the batch of "each of them" tokens can't be
//! enumerated in the resolver and CopyPermanent can't create the copy
//! tapped. The second (your tokens) makes each opponent lose 1 life and you
//! gain 1 life, which is fully wired.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kambal, Profiteering Mayor");
    let human = reg.interner_mut().intern("Human");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .tokens_only()
                        .controlled_by(ControllerConstraint::Opponent),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: copy_opponent_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .tokens_only()
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: drain_each_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn copy_opponent_tokens(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "for each of them, create a tapped token that's a copy of it" —
    // the batch of tokens that entered cannot be enumerated in the resolver,
    // and CopyPermanent cannot create the copy tapped.
    Vec::new()
}

fn drain_each_opponent(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![Effect::GainLife { player: trig.controller, amount: 1 }];
    for opp in script::opponents(state, trig.controller) {
        effects.push(Effect::LoseLife { player: opp, amount: 1 });
    }
    vec![Effect::Sequence(effects)]
}
