//! Watchful Blisterzoa — `{4}{U}{U}` 4/4 Phyrexian Jellyfish with Flying.
//! "This creature enters with an oil counter on it."
//! "When this creature dies, draw cards equal to the number of oil counters on it."

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
    let name = reg.interner_mut().intern("Watchful Blisterzoa");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let jellyfish = reg.interner_mut().intern("Jellyfish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(jellyfish);
    // Pre-intern the named counter so the dies-trigger lookup resolves.
    let _oil = reg.interner_mut().intern("oil");

    // GAP: "enters with an oil counter on it" — enters-with-counter replacement
    //      is not in the available Effect/ability catalog.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: dies_draw_oil,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn dies_draw_oil(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let id = trig.dying_object().unwrap_or(trig.source);
    let oil = match reg.interner().lookup("oil") {
        Some(s) => CounterKind::Named(s),
        None => return Vec::new(),
    };
    let n = state.objects.get(id).map_or(0, |o| o.count_counters(oil));
    vec![Effect::DrawCards { player: trig.controller, count: n }]
}
