//! Infestation — `{3}{B}{B}` 6/5 black Elemental Incarnation.
//!
//! Oracle:
//! * Wither.
//! * When this creature enters, conjure a card named Blowfly Infestation
//!   onto the battlefield. Then put a -1/-1 counter on each creature.
//! * Evoke—{2}{B}{B}, Pay 2 life. GAP: Evoke is an alternative cast cost,
//!   not a battlefield trigger/activation — not expressible here.
//!
//! The ETB conjure half is a GAP (Conjure is an Arena-only mechanic with no
//! `Effect::Conjure` variant). The "-1/-1 counter on each creature" half IS
//! wired via `ForEach` over all creatures on the battlefield.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Infestation");
    let elemental = reg.interner_mut().intern("Elemental");
    let incarnation = reg.interner_mut().intern("Incarnation");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(incarnation);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Wither],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_minus_one_each_creature,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_minus_one_each_creature(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "conjure a card named Blowfly Infestation onto the battlefield" —
    // Conjure is not modeled (Arena-only mechanic; no Effect::Conjure variant).
    // The "-1/-1 counter on each creature" half is wired below.
    let ids = script::ids_matching(state, &ObjectFilter::creature(), trig.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::AddCounters {
            target: arcana_core::objects::NULL_OBJECT_ID,
            kind: CounterKind::MinusOneMinusOne,
            count: 1,
        }),
    }]
}
