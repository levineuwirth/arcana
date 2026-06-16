//! X, Mystery Racer — `{3}{U}{B}` 5/5 Legendary Human Spy.
//! "When X enters, you may put it into target opponent's hand. If you do, as
//! long as it remains in their hand, that player can't cast X and they play
//! with their hand revealed.
//! During your turn while X is in an opponent's hand, you may activate
//! abilities of nonland permanents opponents control. If you activate the crew
//! or saddle ability of a permanent this way, gain control of it until end of
//! turn. Untap it. It gains haste until end of turn."
//!
//! Both abilities exceed the demonstrated API:
//! - The ETB "put it into target opponent's hand" with the trailing can't-cast
//!   + revealed-hand restrictions has no `Effect` for moving a battlefield
//!   permanent into an opponent's hand plus the linked static riders.
//! - The "activate opponent permanents from an opponent's hand" line is a
//!   static permission with bespoke control/untap/haste riders, not expressible.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::effects::Effect;
use arcana_core::zones::Zone;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("X, Mystery Racer");
    let human = reg.interner_mut().intern("Human");
    let spy = reg.interner_mut().intern("Spy");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(spy);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: ETB "you may put it into target opponent's hand" — no Effect
            //      to move a battlefield permanent into an opponent's hand, and
            //      the linked can't-cast / revealed-hand riders are unexpressible.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_to_opponent_hand,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP static: "During your turn while X is in an opponent's hand, you
        //      may activate abilities of nonland permanents opponents control…"
        //      — bespoke permission + crew/saddle control rider, no API surface.
    )
}

fn etb_to_opponent_hand(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: cannot move self from battlefield into an opponent's hand, nor wire
    //      the linked can't-cast / play-with-hand-revealed restrictions.
    Vec::new()
}
