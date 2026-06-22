//! Renegade Reaper — `{2}{B}` 2/3 Angel Berserker with Flying.
//! "When this creature enters, mill four cards. If at least one Angel card
//! is milled this way, you gain 4 life."

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
    let name = reg.interner_mut().intern("Renegade Reaper");
    let angel = reg.interner_mut().intern("Angel");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    subtypes.0.insert(berserker);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_mill_four,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_mill_four(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "If at least one Angel card is milled this way, you gain 4 life."
    // The just-milled cards can't be inspected within the resolver, so the
    // conditional life gain is omitted; only the mill is emitted.
    vec![Effect::Mill { player: trig.controller, count: 4 }]
}
