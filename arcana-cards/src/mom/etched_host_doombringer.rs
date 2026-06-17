//! Etched Host Doombringer — `{4}{B}` 3/5 Phyrexian Demon.
//! When it enters, choose one — drain 2, OR add/remove defense counters
//! on a target battle. Modal-on-a-TRIGGER isn't expressible via the
//! documented (spell-only) modal surface, and battle targeting / defense-
//! counter manipulation has no TargetFilter, so the second mode is GAP'd.
//! We wire the ETB to the first mode (drain 2) only.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::script;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Etched Host Doombringer");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_drain,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

// GAP: this is a "Choose one —" modal trigger; modal dispatch is only
// available for spell abilities, and the second mode targets a battle
// (no TargetFilter for battles + defense-counter manipulation). We
// resolve the first mode (target opponent loses 2 life, you gain 2 life)
// unconditionally.
fn etb_drain(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let opps = script::opponents(state, trig.controller);
    let Some(&opp) = opps.first() else {
        return vec![Effect::GainLife { player: trig.controller, amount: 2 }];
    };
    vec![
        Effect::LoseLife { player: opp, amount: 2 },
        Effect::GainLife { player: trig.controller, amount: 2 },
    ]
}
