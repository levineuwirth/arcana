//! Hungry Graffalon — `{3}{G}` 3/4 green Giraffe.
//!
//! * Reach (keyword).
//! * Increment — "Whenever you cast a spell, if the amount of mana you
//!   spent is greater than this creature's power or toughness, put a
//!   +1/+1 counter on this creature." The Increment keyword is not in
//!   the usable keyword surface, and the intervening-if compares the
//!   mana spent against this creature's power/toughness, which is not
//!   expressible with the demonstrated `conditions::` predicates — so
//!   the gating "if" is GAP'd. We still wire the spell-cast trigger
//!   that puts the +1/+1 counter (the firing half).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hungry Graffalon");
    let giraffe = reg.interner_mut().intern("Giraffe");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giraffe);

    // GAP: Increment is not an available keyword for this card class —
    // emitting only the firing half below.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: None,
                caster: ControllerConstraint::You,
            },
            // GAP: intervening-if "if the amount of mana you spent is
            // greater than this creature's power or toughness" — no
            // conditions:: predicate compares mana-spent vs source P/T.
            intervening_if: None,
            effect: put_counter_on_self,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn put_counter_on_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
