//! Arbiter of Woe — `{4}{B}{B}` 5/4 Demon with Flying.
//! "As an additional cost to cast this spell, sacrifice a creature.
//!  When this creature enters, each opponent discards a card and loses
//!  2 life. You draw a card and gain 2 life."
//!
//! The additional cast cost (sacrifice a creature) is a casting-time
//! cost with no demonstrated primitive — GAP'd. Flying is a base
//! keyword. The ETB is a no-target multi-clause effect: each opponent
//! discards + loses 2, then you draw + gain 2.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arbiter of Woe");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        // GAP: additional cast cost "sacrifice a creature" — no demonstrated primitive for extra casting costs.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_woe,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_woe(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut out: Vec<Effect> = Vec::new();
    for opp in script::opponents(state, trig.controller) {
        out.push(Effect::Discard {
            player: opp,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        });
        out.push(Effect::LoseLife { player: opp, amount: 2 });
    }
    out.push(Effect::DrawCards { player: trig.controller, count: 1 });
    out.push(Effect::GainLife { player: trig.controller, amount: 2 });
    vec![Effect::Sequence(out)]
}
