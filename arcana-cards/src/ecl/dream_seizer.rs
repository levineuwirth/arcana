//! Dream Seizer — `{3}{B}` 3/2 Faerie Rogue with Flying.
//!
//! Oracle:
//! * Flying — keyword, base characteristic.
//! * "When this creature enters, you may blight 1. If you do, each opponent
//!    discards a card. (To blight 1, put a -1/-1 counter on a creature you
//!    control.)" — GAP: "blight 1" is an optional self-imposed cost (put a
//!    -1/-1 counter on a chosen creature you control) gating the discard; the
//!    "you may [pay this non-mana, non-sacrifice cost]. If you do, …" shape has
//!    no API surface (OptionalPayment only supports Mana/Life). Emitting the
//!    discard unconditionally would be materially wrong, so the trigger body
//!    returns `Vec::new()`.

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
    let name = reg.interner_mut().intern("Dream Seizer");
    let faerie = reg.interner_mut().intern("Faerie");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_blight,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_blight(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may blight 1 (put a -1/-1 counter on a chosen creature you
    //      control). If you do, each opponent discards a card." — the optional
    //      non-mana cost gate has no API surface.
    Vec::new()
}
