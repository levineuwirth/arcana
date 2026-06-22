//! Finneas, Ace Archer — `{G}{W}` 2/2 Legendary Rabbit Archer.
//! Vigilance, reach.
//! "Whenever Finneas attacks, put a +1/+1 counter on each other creature
//!  you control that's a token or a Rabbit. Then if creatures you control
//!  have total power 10 or greater, draw a card."
//!
//! Keyword line + the attack-trigger counters are expressed.
//! GAP: "Then if creatures you control have total power 10 or greater, draw
//!      a card" — there is no total-power script helper / condition, so the
//!      conditional draw is omitted (the counters still resolve).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Finneas, Ace Archer");
    let rabbit = reg.interner_mut().intern("Rabbit");
    let archer = reg.interner_mut().intern("Archer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rabbit);
    subtypes.0.insert(archer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: counter_tokens_and_rabbits,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn counter_tokens_and_rabbits(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "token or a Rabbit" is a disjunction that one ObjectFilter cannot
    // express (filters AND their predicates), so collect both id sets and
    // union them.
    let tokens = script::ids_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .tokens_only(),
        trig.controller,
    );
    let rabbits = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Rabbit")
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );

    let mut effects = Vec::new();
    let mut seen: Vec<_> = Vec::new();
    for id in tokens.into_iter().chain(rabbits.into_iter()) {
        if id == trig.source || seen.contains(&id) {
            continue;
        }
        seen.push(id);
        effects.push(Effect::AddCounters {
            target: id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        });
    }
    effects
}
