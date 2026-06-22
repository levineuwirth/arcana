//! Kylem All-Star — `{3}{W}` 2/2 Creature — Human Warrior.
//! When Kylem All-Star enters the battlefield, create a token copy of All that
//!   Glitters and attach it to this creature.
//! Enchantments you control are Gold artifacts in addition to their other types
//!   and have "Sacrifice this artifact: Add one mana of any color."
//!
//! Decomposition:
//! 1. ETB trigger recorded. GAP the body: "create a token copy of All that
//!    Glitters" requires minting a token copy of a NAMED CARD (an Aura that
//!    isn't on the battlefield). Effect::CopyPermanent copies an existing
//!    permanent by ObjectId, not a card by name, so there is no expressible
//!    effect here. Omitted.
//! 2. "Enchantments you control are Gold artifacts … and have 'Sacrifice this
//!    artifact: Add one mana of any color.'" — pure static (type-grant + grant
//!    of an activated ability to other permanents). No Effect on this card
//!    class; GAP (documented only).

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
    let name = reg.interner_mut().intern("Kylem All-Star");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_token_copy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_token_copy(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "create a token copy of All that Glitters and attach it to this
    // creature." Requires minting a token copy of a NAMED CARD (not an
    // on-battlefield permanent); Effect::CopyPermanent copies by ObjectId only.
    // Effect omitted.
    Vec::new()
}
