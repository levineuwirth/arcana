//! Darksteel Hydra — `{X}{W}{B}{G}` */* Phyrexian Hydra (Artifact Creature).
//! Indestructible.
//! Darksteel Hydra enters with X oil counters on it.
//! Its power and toughness are each equal to twice the number of oil
//! counters on it.
//! When Darksteel Hydra enters, conjure a card named Darksteel Ingot
//! and a card named Darksteel Plate into your hand.
//!
//! Indestructible is a base keyword. The two oil-counter pieces (enter
//! with X, twice-oil CDA P/T) and the conjure ETB are not expressible.

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
    let name = reg.interner_mut().intern("Darksteel Hydra");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{W}{B}{G}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black() | ColorSet::green(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        // GAP (CDA): "power and toughness each equal to twice the number of
        // oil counters on it." No computing static is expressible; left as *.
        // GAP (ETB): "enters with X oil counters on it." No enters-with-X
        // counter primitive is available.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Indestructible],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_conjure,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_conjure(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Conjure not modeled (Arena-only mechanic; would need a
    // registry-by-name card creation in Effect::execute). "Conjure a card
    // named Darksteel Ingot and a card named Darksteel Plate into your hand."
    Vec::new()
}
