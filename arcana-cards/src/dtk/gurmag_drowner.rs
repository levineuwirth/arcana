//! Gurmag Drowner — `{3}{U}` 2/4 Snake Wizard.
//! "Exploit (When this creature enters, you may sacrifice a creature.)"
//! "When this creature exploits a creature, look at the top four cards of
//! your library. Put one of them into your hand and the rest into your
//! graveyard."
//!
//! Exploit is not in the usable keyword surface. The "when this enters,
//! you may sacrifice a creature" cost and the resulting "when this
//! exploits a creature" trigger are not modeled, so the dig payoff cannot
//! be gated on an exploit event and the body is GAP'd (firing it
//! unconditionally on enter would be a materially wrong card).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gurmag Drowner");
    let snake = reg.interner_mut().intern("Snake");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: Exploit — the "when this enters, you may sacrifice a
            // creature" cost and the resulting "when this exploits a
            // creature" trigger are not modeled, so the dig payoff can't
            // be gated on an exploit event.
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: exploit_payoff,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn exploit_payoff(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Exploit mechanic not modeled — cannot sacrifice-then-trigger,
    // so the "look at top four, one to hand, rest to graveyard" dig is not
    // emitted (firing it unconditionally on enter would be a materially
    // wrong card).
    Vec::new()
}
