//! Gonti, Canny Acquisitor — `{2}{B}{G}{U}` Legendary 5/5 Aetherborn Rogue.
//!
//! Spells you cast but don't own cost {1} less to cast.
//!   — GAP: static cost-reduction is not a triggered/activated ability
//!     and has no expressible Effect in the demonstrated catalog.
//! Whenever one or more creatures you control deal combat damage to a
//! player, look at the top card of that player's library, then exile
//! it face down. You may play that card for as long as it remains
//! exiled, and mana of any type can be spent to cast that spell.
//!   — GAP: "exile top of opponent's library face down + play it from
//!     exile with any color of mana" has no Effect in the demonstrated
//!     catalog (no top-of-opponent-library exile-with-play primitive).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gonti, Canny Acquisitor");
    let aetherborn = reg.interner_mut().intern("Aetherborn");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aetherborn);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{G}{U}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: steal_top_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn steal_top_card(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no Effect for "look at top card of that player's library,
    // exile it face down, and let you play it from exile with any mana".
    Vec::new()
}
