//! Ace, Fearless Rebel — `{3}{G}` 2/2 Legendary Human Rebel.
//!
//! Oracle:
//! * "Nitro-9 — Whenever Ace attacks, you may sacrifice an artifact.
//!   When you do, put a +1/+1 counter on Ace, then it fights up to one
//!   target creature defending player controls." — a SelfAttacks
//!   trigger. The "you may sacrifice an artifact" gate (and the
//!   reflexive "when you do" linkage) has no expressible primitive, so
//!   the payoff (counter + fight) is emitted directly and the sacrifice
//!   gate is a documented fidelity gap.
//! * "Doctor's companion" — a commander-construction static, GAP'd.
//!
//! Note: the Scryfall keywords (Nitro-9, Fight, Doctor's companion) are
//! not engine KeywordAbility variants, so `keywords` is empty.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ace, Fearless Rebel");
    let human = reg.interner_mut().intern("Human");
    let rebel = reg.interner_mut().intern("Rebel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rebel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "Doctor's companion" — a commander-deckbuilding static, not
    // a triggered/activated ability.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attacks_counter_fight,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn attacks_counter_fight(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP fidelity: "you may sacrifice an artifact. When you do, ..." —
    // the sacrifice gate and reflexive linkage are not expressible; the
    // counter + fight payoff is applied directly.
    let mut effects = vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }];
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.first() {
        effects.push(Effect::Fight {
            a: trig.source,
            b: *id,
        });
    }
    effects
}
