//! Prime Speaker Zegana — `{2}{G}{G}{U}{U}` 1/1 Legendary Creature — Merfolk
//! Wizard.
//!
//! * Prime Speaker Zegana enters with X +1/+1 counters on it, where X is the
//!   greatest power among other creatures you control. — GAP: this is a
//!   replacement "enters with N counters" whose N is the greatest power among
//!   other creatures; there is no enters-with-counters primitive and no
//!   "greatest power among" script helper in this surface.
//! * When Prime Speaker Zegana enters, draw cards equal to its power. — wired
//!   as an ETB trigger drawing `script::power_of(source)` cards.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Prime Speaker Zegana");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}{U}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: static replacement "enters with X +1/+1 counters on it, where X is
    // the greatest power among other creatures you control" — no
    // enters-with-counters primitive / no greatest-power-among script helper.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_draw_equal_to_power,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "When Prime Speaker Zegana enters, draw cards equal to its power."
fn etb_draw_equal_to_power(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let id = trig.entering_object().unwrap_or(trig.source);
    let n = script::power_of(state, id).max(0) as u32;
    vec![Effect::DrawCards { player: trig.controller, count: n }]
}
