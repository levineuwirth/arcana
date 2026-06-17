//! Ambassador Blorpityblorpboop — `{3}{G}{U}` 3/3 Legendary Alien Advisor Guest.
//! "When ~ enters, you get {TK}{TK}{TK}, then you may put a sticker on a
//!  nonland permanent you own."
//! "At the beginning of each combat, you may have ~'s base power become equal
//!  to the total power of all stickers on permanents you control and its base
//!  toughness become equal to those stickers' total toughness."
//!
//! Stickers ({TK} tickets and sticker placement / sticker-derived P/T) are not
//! modeled in the engine. Both abilities' effects are GAP'd; the trigger
//! structure is preserved.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ambassador Blorpityblorpboop");
    let alien = reg.interner_mut().intern("Alien");
    let advisor = reg.interner_mut().intern("Advisor");
    let guest = reg.interner_mut().intern("Guest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(alien);
    subtypes.0.insert(advisor);
    subtypes.0.insert(guest);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tickets_and_sticker,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: combat_sticker_pt,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_tickets_and_sticker(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: {TK} ticket gain and sticker placement are not modeled.
    Vec::new()
}

fn combat_sticker_pt(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: sticker-derived base power/toughness is not modeled.
    Vec::new()
}
