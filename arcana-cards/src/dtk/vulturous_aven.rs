//! Vulturous Aven — `{3}{B}` 2/3 Bird Shaman with Flying.
//! "Exploit (When this creature enters, you may sacrifice a creature.)"
//! "When this creature exploits a creature, you draw two cards and you
//! lose 2 life."

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Vulturous Aven");
    let bird = reg.interner_mut().intern("Bird");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        // Exploit is not an expressible KeywordAbility variant.
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: Exploit — the "when this enters, you may sacrifice a creature"
            // cost and the resulting "when this exploits a creature" trigger are
            // not modeled, so the payoff (draw two, lose 2 life) can't be gated
            // on an exploit event.
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
    // GAP: Exploit mechanic not modeled — cannot sacrifice-then-trigger, so the
    // "draw two cards, lose 2 life" payoff is not emitted (firing it
    // unconditionally would be a materially wrong card).
    Vec::new()
}
