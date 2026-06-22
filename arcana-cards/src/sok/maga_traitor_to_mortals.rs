//! Maga, Traitor to Mortals — `{X}{B}{B}{B}` 0/0 Legendary Human Wizard.
//!
//! Oracle:
//! * "Maga enters with X +1/+1 counters on it." — modeled via
//!   `EntersWithSpec::CountersFromX`, which reads the cast's X.
//! * "When Maga enters, target player loses life equal to the number of
//!   +1/+1 counters on it." — an ETB trigger; the amount is read at
//!   resolution from the entering object's +1/+1 counter count.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Maga, Traitor to Mortals");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::CountersFromX {
                kind: CounterKind::PlusOnePlusOne,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: lose_life_for_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            }),
    )
}

fn lose_life_for_counters(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    let src = trig.entering_object().unwrap_or(trig.source);
    let n = state
        .objects
        .get(src)
        .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne));
    vec![Effect::LoseLife { player: *p, amount: n }]
}
