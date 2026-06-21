//! Chivalrous Chevalier — `{4}{W}` 3/3 Artifact Creature — Cyborg Knight.
//!
//! Flying.
//! When this creature enters, return a creature you control to its
//! owner's hand unless you compliment an opponent.
//!
//! Flying is expressible. The ETB is GAP'd: "compliment an opponent" is
//! an Un-set social mechanic with no engine support, and the "unless you
//! [compliment]" escape can't be modeled (`OptionalPaymentKind` only
//! covers Mana / Life). Firing the bounce unconditionally would be wrong
//! (it would always return a creature with no opt-out), so the whole
//! effect is omitted.

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
    let name = reg.interner_mut().intern("Chivalrous Chevalier");
    let cyborg = reg.interner_mut().intern("Cyborg");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cyborg);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_bounce_unless_compliment,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_bounce_unless_compliment(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "return a creature you control to its owner's hand UNLESS you
    // compliment an opponent" — the compliment escape (an Un-set social
    // mechanic) can't be modeled and OptionalPaymentKind covers only
    // Mana/Life, so the bounce can't be offered with its opt-out.
    Vec::new()
}
