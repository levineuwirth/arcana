//! Impetuous Protege — `{2}{R}` 0/4 red Human Warrior.
//!
//! * Partner with Proud Mentor — NOT an expressible keyword (`Partner` /
//!   `Partner with` are not in the supported `KeywordAbility` surface), so
//!   `keywords` is empty and the partner ETB tutor is GAP'd.
//! * Whenever this creature attacks, it gets +X/+0 until end of turn, where X
//!   is the greatest power among tapped creatures your opponents control.

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

// GAP: "Partner with Proud Mentor" — the Partner / Partner-with keyword is not
// in the supported KeywordAbility surface, and its ETB ("target player may put
// Proud Mentor into their hand from their library, then shuffle") has no
// expressible primitive. Omitted; keywords vec is empty.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Impetuous Protege");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attack_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attack_pump(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: +X/+0 where X = the GREATEST power among tapped creatures your
    // opponents control. No script helper computes a max-power over a filtered
    // set (only count_matching / devotion etc.), so this amount is not
    // expressible without hardcoding a wrong literal.
    Vec::new()
}
