//! Ayesha Tanaka, Armorer — `{3}{W}{U}` 2/4 Legendary Human Artificer.
//! "Whenever Ayesha attacks, look at the top four cards of your library.
//! You may put any number of artifact cards with mana value <= Ayesha's
//! power from among them onto the battlefield tapped. Put the rest on
//! the bottom of your library in a random order."
//! "Ayesha Tanaka can't be blocked as long as defending player controls
//! three or more artifacts."
//!
//! GAP: the attack trigger — `DigTopN` is single-take to hand only; no
//! primitive expresses "look at the top N, put ANY NUMBER of matching
//! cards onto the battlefield tapped, rest to bottom". Effect GAP'd.
//! GAP: "can't be blocked as long as defending player controls three or
//! more artifacts" — a conditional can't-be-blocked static keyed on the
//! defending player's board is not expressible as a static here.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ayesha Tanaka, Armorer");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: dig_put_artifacts,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dig_put_artifacts(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: see module doc — no "look at top N, put any number onto the
    // battlefield tapped" primitive. No effect emitted.
    Vec::new()
}
