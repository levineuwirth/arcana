//! Hypnox — `{8}{B}{B}{B}` 8/8 Nightmare Horror with Flying.
//! "When this creature enters, if you cast it from your hand, exile all
//! cards from target opponent's hand. When this creature leaves the
//! battlefield, return the exiled cards to their owner's hand."

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
    let name = reg.interner_mut().intern("Hypnox");
    let nightmare = reg.interner_mut().intern("Nightmare");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightmare);
    subtypes.0.insert(horror);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{8}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_exile_hand,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfLeavesBattlefield,
                intervening_if: None,
                effect: ltb_return_exiled,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_exile_hand(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if you cast it from your hand, exile all cards from target
    // opponent's hand." No cast-from-hand condition helper, and no
    // exile-entire-hand effect primitive exists.
    Vec::new()
}

fn ltb_return_exiled(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "return the exiled cards to their owner's hand." There is no
    // tracking for the set of hand cards exiled by the ETB, so the linked
    // return is unexpressible.
    Vec::new()
}
