//! Hired Giant — `{3}{R}` 4/4 red Giant.
//! "When this creature enters, each other player may search their
//! library for a land card and put that card onto the battlefield.
//! Then each player who searched their library this way shuffles."
//!
//! The oracle gives each OPPONENT a land tutor to battlefield (tapped
//! is not specified, so untapped). There is no "each opponent" variant
//! of TutorToBattlefield — we use script::opponents and build one
//! effect per opponent.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hired Giant");
    let giant = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_opponent_land_tutor,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_opponent_land_tutor(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Each OTHER player (opponent) may search their library for a land
    // and put it onto the battlefield. The "may" is not expressible as
    // OptionalPayment (no cost), so we emit TutorToBattlefield for each
    // opponent unconditionally — GAP: the "may" (optional) search is not
    // modeled; all opponents always get the land.
    let opponents = script::opponents(state, trig.controller);
    let land_filter = ObjectFilter::new().with_types(TypeLine::LAND.into());
    let effects: Vec<Effect> = opponents
        .into_iter()
        .map(|p| Effect::TutorToBattlefield {
            player: p,
            filter: land_filter.clone(),
            tapped: false,
        })
        .collect();
    if effects.is_empty() {
        Vec::new()
    } else {
        vec![Effect::Sequence(effects)]
    }
}
