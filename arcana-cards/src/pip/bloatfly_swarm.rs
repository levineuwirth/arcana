//! Bloatfly Swarm — `{3}{B}` 0/0 Insect Mutant with Flying.
//!
//! Oracle:
//! * Flying.
//! * This creature enters with five +1/+1 counters on it.
//! * If damage would be dealt to this creature while it has a +1/+1 counter on
//!   it, prevent that damage, remove that many +1/+1 counters from it, then
//!   give each player a rad counter for each +1/+1 counter removed this way.
//!
//! "Enters with five +1/+1 counters" is modeled as an ETB trigger adding five
//! +1/+1 counters to itself (id 1). The damage-replacement clause is GAP'd: it
//! is a self-referential replacement effect (prevent damage → remove that many
//! +1/+1 counters → distribute rad counters by the amount removed), which has
//! no expressible triggered/activated form.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bloatfly Swarm");
    let insect = reg.interner_mut().intern("Insect");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: enter_with_five_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
    // GAP (replacement): "If damage would be dealt to this creature while it
    // has a +1/+1 counter, prevent that damage, remove that many +1/+1
    // counters, then give each player a rad counter per counter removed." —
    // a self-referential damage replacement, not expressible.
}

fn enter_with_five_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 5,
    }]
}
