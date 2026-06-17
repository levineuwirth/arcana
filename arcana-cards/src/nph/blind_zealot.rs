//! Blind Zealot — `{1}{B}{B}` 2/2 black Phyrexian Human Cleric with
//! Intimidate.
//! "Whenever this creature deals combat damage to a player, you may
//! sacrifice it. If you do, destroy target creature that player
//! controls." The effect is gated on an optional self-sacrifice cost
//! ("you may sacrifice it. If you do, …"); the available API has no
//! optional-self-sacrifice gate (OptionalPayment covers only mana/life),
//! and emitting the destroy unconditionally would be materially stronger
//! than the card. The trigger is wired but its effect is GAP'd. Intimidate
//! is emitted as a keyword.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blind Zealot");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Intimidate],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: on_combat_damage,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_combat_damage(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may sacrifice it. If you do, destroy target creature that
    // player controls." — the optional-self-sacrifice gate on the
    // targeted destroy is not expressible (OptionalPayment covers only
    // mana/life costs), and an unconditional destroy would be materially
    // stronger than the card.
    Vec::new()
}
